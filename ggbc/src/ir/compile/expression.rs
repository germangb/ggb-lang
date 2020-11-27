use crate::{
    byteorder::ByteOrder,
    ir::{
        alloc::{FnAlloc, RegisterAlloc, SymbolAlloc, SymbolMemorySpace},
        layout::Layout,
        opcodes::{Destination, Pointer, Source, Statement},
    },
    parser::ast::{Expression, Path},
};

// match to a particular `Expression` enum variant.
// panics if not possible.
// match_expr!(expr, Expression::Index);
macro_rules! match_expr {
    ($expr:expr, $enum:ident :: $var:ident) => {
        match $expr {
            $enum::$var(inner) => inner,
            _ => panic!(),
        }
    };

    ($expr:expr, $enum:ident :: $var:ident, $field:ident) => {
        match $expr {
            $enum::$var { $field, .. } => $field,
            _ => panic!(),
        }
    };
}

/// Utility function to free all registers referenced inside a Source.
pub fn free_source_registers(source: &Source<u8>, register_alloc: &mut RegisterAlloc) {
    use Source::*;
    match source {
        Register(r) => register_alloc.free(*r),
        Pointer { offset: Some(offset),
                  .. } => free_source_registers(offset, register_alloc),
        _ => {}
    }
}

/// Utility function to free any registers referenced within a given
/// `Destination`.
pub fn free_destination_registers(destination: &Destination, register_alloc: &mut RegisterAlloc) {
    use Destination::*;
    match destination {
        Register(r) => register_alloc.free(*r),
        Pointer { offset: Some(offset),
                  .. } => free_source_registers(offset, register_alloc),
        _ => {}
    }
}

fn destination_to_source(destination: &Destination) -> Source<u8> {
    use Destination::*;
    match destination {
        Pointer { base, offset } => Source::Pointer { base: base.clone(),
                                                      offset: offset.clone() },
        Register(register) => Source::Register(*register),
    }
}

/// Evaluate and return the result of a constant expression.
/// If the passed expression is not a constant expression, returns `None`.
pub fn const_expr<B: ByteOrder>(expression: &Expression<'_>,
                                symbol_alloc: Option<&SymbolAlloc<B>>)
                                -> Option<u16> {
    use Expression as E;
    match (symbol_alloc, expression) {
        (Some(symbol_alloc), E::Path(path)) => {
            let name = path_to_symbol_name(path);
            let symbol = symbol_alloc.get(&name);
            match symbol.memory_space {
                SymbolMemorySpace::Const => {
                    Some(symbol_alloc.const_data()[symbol.offset as usize] as _)
                }
                _ => None,
            }
        }
        (_, E::Lit(lit)) => {
            let num = lit.to_string();
            Some(if num.starts_with("0x") {
                     u16::from_str_radix(&num[2..], 16).expect("Not a hex number")
                 } else if num.starts_with('0') && num.len() > 1 {
                     u16::from_str_radix(&num[1..], 8).expect("Not an octal number")
                 } else if num.starts_with("0b") {
                     u16::from_str_radix(&num[2..], 2).expect("Not a bin number")
                 } else {
                     u16::from_str_radix(&num[..], 10).expect("Not a decimal number")
                 })
        }
        (_, E::Minus(_e)) => todo!(),
        (_, E::Not(e)) => Some(!const_expr(&e.inner, symbol_alloc)?),
        (_, E::Add(e)) => Some(const_expr(&e.inner.left, symbol_alloc)?
                               + const_expr(&e.inner.right, symbol_alloc)?),
        (_, E::Sub(e)) => Some(const_expr(&e.inner.left, symbol_alloc)?
                               - const_expr(&e.inner.right, symbol_alloc)?),
        (_, E::Mul(e)) => Some(const_expr(&e.inner.left, symbol_alloc)?
                               * const_expr(&e.inner.right, symbol_alloc)?),
        (_, E::Div(e)) => Some(const_expr(&e.inner.left, symbol_alloc)?
                               / const_expr(&e.inner.right, symbol_alloc)?),
        (_, E::And(e)) => Some(const_expr(&e.inner.left, symbol_alloc)?
                               & const_expr(&e.inner.right, symbol_alloc)?),
        (_, E::Or(e)) => Some(const_expr(&e.inner.left, symbol_alloc)?
                              | const_expr(&e.inner.right, symbol_alloc)?),
        (_, E::Xor(e)) => Some(const_expr(&e.inner.left, symbol_alloc)?
                               ^ const_expr(&e.inner.right, symbol_alloc)?),
        (_, E::LeftShift(e)) => Some(const_expr(&e.inner.left, symbol_alloc)?
                                     << const_expr(&e.inner.right, symbol_alloc)?),
        (_, E::RightShift(e)) => Some(const_expr(&e.inner.left, symbol_alloc)?
                                      >> const_expr(&e.inner.right, symbol_alloc)?),
        (_, E::Eq(e)) => {
            let l = const_expr(&e.inner.left, symbol_alloc)?;
            let r = const_expr(&e.inner.left, symbol_alloc)?;
            Some(if l == r { 1 } else { 0 })
        }
        (_, E::NotEq(e)) => {
            let l = const_expr(&e.inner.left, symbol_alloc)?;
            let r = const_expr(&e.inner.left, symbol_alloc)?;
            Some(if l != r { 1 } else { 0 })
        }
        (_, E::Greater(e)) => {
            let l = const_expr(&e.inner.left, symbol_alloc)?;
            let r = const_expr(&e.inner.left, symbol_alloc)?;
            Some(if l > r { 1 } else { 0 })
        }
        (_, E::GreaterEq(e)) => {
            let l = const_expr(&e.inner.left, symbol_alloc)?;
            let r = const_expr(&e.inner.left, symbol_alloc)?;
            Some(if l >= r { 1 } else { 0 })
        }
        (_, E::Less(e)) => {
            let l = const_expr(&e.inner.left, symbol_alloc)?;
            let r = const_expr(&e.inner.left, symbol_alloc)?;
            Some(if l < r { 1 } else { 0 })
        }
        (_, E::LessEq(e)) => {
            let l = const_expr(&e.inner.left, symbol_alloc)?;
            let r = const_expr(&e.inner.left, symbol_alloc)?;
            Some(if l <= r { 1 } else { 0 })
        }
        _ => None,
    }
}

/// Compile assignment statement/expression.
/// These expressions evaluate to no value in particular.
pub fn compile_assign<B: ByteOrder>(expression: &Expression<'_>,
                                    symbol_alloc: &SymbolAlloc<B>,
                                    fn_alloc: &FnAlloc,
                                    register_alloc: &mut RegisterAlloc,
                                    statements: &mut Vec<Statement>) {
    macro_rules! arithmetic_branch {
        ($var:ident, $node:expr) => {{
            let destination = assign_destination(&$node.inner.left,
                                                 symbol_alloc,
                                                 fn_alloc,
                                                 register_alloc,
                                                 statements);
            let left = destination_to_source(&destination);
            let right = compile_expr(&$node.inner.right,
                                     symbol_alloc,
                                     fn_alloc,
                                     register_alloc,
                                     statements);
            // free left and destination only (right is a copy of the former)
            free_source_registers(&right, register_alloc);
            free_destination_registers(&destination, register_alloc);
            statements.push(Statement::$var { left,
                                              right,
                                              destination });
        }};
    }

    use Expression as E;
    match expression {
        E::Assign(node) => {
            let source = compile_expr(&node.inner.right,
                                      symbol_alloc,
                                      fn_alloc,
                                      register_alloc,
                                      statements);
            let destination = assign_destination(&node.inner.left,
                                                 symbol_alloc,
                                                 fn_alloc,
                                                 register_alloc,
                                                 statements);
            free_source_registers(&source, register_alloc);
            free_destination_registers(&destination, register_alloc);
            statements.push(Statement::Ld { source,
                                            destination });
        }
        E::PlusAssign(node) => arithmetic_branch!(Add, node),
        E::MinusAssign(node) => arithmetic_branch!(Sub, node),
        E::MulAssign(node) => arithmetic_branch!(Mul, node),
        E::DivAssign(node) => arithmetic_branch!(Div, node),
        E::AndAssign(node) => arithmetic_branch!(And, node),
        E::OrAssign(node) => arithmetic_branch!(Or, node),
        E::XorAssign(node) => arithmetic_branch!(Xor, node),
        _ => unreachable!(),
    }
}

// compute the destination of an assignment expression
fn assign_destination<B: ByteOrder>(expression: &Expression<'_>,
                                    symbol_alloc: &SymbolAlloc<B>,
                                    fn_alloc: &FnAlloc,
                                    register_alloc: &mut RegisterAlloc,
                                    statements: &mut Vec<Statement>)
                                    -> Destination {
    use Expression as E;
    match expression {
        E::Path(path) => {
            let name = path_to_symbol_name(path);
            let symbol = symbol_alloc.get(&name);
            Destination::Pointer { base: symbol.pointer(),
                                   offset: None }
        }
        E::Index(index) => {
            let offset = compile_expr(&index.inner.left,
                                      symbol_alloc,
                                      fn_alloc,
                                      register_alloc,
                                      statements);
            let mut destination = assign_destination(&index.inner.right,
                                                     symbol_alloc,
                                                     fn_alloc,
                                                     register_alloc,
                                                     statements);
            match_expr!(&mut destination, Destination::Pointer, offset).replace(Box::new(offset));
            destination
        }
        _ => unreachable!(),
    }
}

/// compile a `Layout::U8` expression, and store the result in a `Source<u8>`
/// return this source.
///
/// # Note
/// It is possible that the `Source` variant will be a `Source::Register`, or a
/// `Source::Pointer` with the value of a register set as offset. In
/// those situations, the callee of the function is responsible for freeing this
/// register.
pub fn compile_expr<B: ByteOrder>(expression: &Expression<'_>,
                                  symbol_alloc: &SymbolAlloc<B>,
                                  fn_alloc: &FnAlloc,
                                  register_alloc: &mut RegisterAlloc,
                                  statements: &mut Vec<Statement>)
                                  -> Source<u8> {
    macro_rules! arithmetic_branch {
        ($var:ident, $node:expr) => {{
            let left = compile_expr(&$node.inner.left,
                                    symbol_alloc,
                                    fn_alloc,
                                    register_alloc,
                                    statements);
            let right = compile_expr(&$node.inner.right,
                                     symbol_alloc,
                                     fn_alloc,
                                     register_alloc,
                                     statements);
            free_source_registers(&left, register_alloc);
            free_source_registers(&right, register_alloc);
            // TODO for now, put it in a register, but it shpuld be possible to instruct the
            //  function to put it somewhere in memory instead.
            let store_register = register_alloc.alloc();
            statements.push(Statement::$var { left,
                                              right,
                                              destination:
                                                  Destination::Register(store_register) });
            Source::Register(store_register)
        }};
    }

    use Expression as E;

    // if the expression is a constant expression, return it as a literal.
    if let Some(n) = const_expr(expression, Some(symbol_alloc)) {
        assert!(n <= 0xff); // TODO wrap?
        return Source::Literal(n as u8);
    }

    match expression {
        // TODO numeric const expressions are handled by the above statement, but what if expression
        //  is a string literal?
        E::Lit(_) => todo!(),

        // symbol name
        E::Path(path) => {
            let symbol_name = path_to_symbol_name(path);
            let symbol = symbol_alloc.get(&symbol_name);
            assert!(matches!(&symbol.layout, Layout::U8));
            Source::Pointer { base: symbol.pointer(),
                              offset: None }
        }

        // 8bit arithmetic
        E::Add(node) => arithmetic_branch!(Add, node),
        E::Sub(node) => arithmetic_branch!(Sub, node),
        E::Mul(node) => arithmetic_branch!(Mul, node),
        E::Div(node) => arithmetic_branch!(Div, node),
        // TODO modulo
        E::And(node) => arithmetic_branch!(And, node),
        E::Or(node) => arithmetic_branch!(Or, node),
        E::Xor(node) => arithmetic_branch!(Xor, node),
        E::LeftShift(node) => arithmetic_branch!(LeftShift, node),
        E::RightShift(node) => arithmetic_branch!(RightShift, node),

        // boolean
        E::Eq(node) => arithmetic_branch!(Eq, node),
        E::NotEq(node) => arithmetic_branch!(NotEq, node),
        E::Greater(node) => arithmetic_branch!(Greater, node),
        E::GreaterEq(node) => arithmetic_branch!(GreaterEq, node),
        E::Less(node) => arithmetic_branch!(Less, node),
        E::LessEq(node) => arithmetic_branch!(LessEq, node),

        // arrays
        E::Index(node) => {
            let right = match_expr!(&node.inner.right, E::Path);
            let name = path_to_symbol_name(&right);
            let symbol = symbol_alloc.get(&name);
            let offset = compile_expr(&node.inner.left,
                                      symbol_alloc,
                                      fn_alloc,
                                      register_alloc,
                                      statements);
            Source::Pointer { base: symbol.pointer(),
                              offset: Some(Box::new(offset)) }
        }

        // functions
        #[warn(unused)]
        E::Call(node) => todo!(),
        _ => unreachable!(),
    }
}

/// compiles the evaluation of an expression, but the result is not stored
/// anywhere.
#[warn(unused)]
pub fn compile_expr_void<B: ByteOrder>(expression: &Expression<'_>,
                                       symbol_alloc: &SymbolAlloc<B>,
                                       fn_alloc: &FnAlloc,
                                       register_alloc: &mut RegisterAlloc,
                                       statements: &mut Vec<Statement>) {
    use Expression as E;
    match expression {
        // superfluous expressions
        E::Lit(_) | E::Path(_) => { /* Nop */ }

        // assignments
        expression @ E::PlusAssign(_)
        | expression @ E::MinusAssign(_)
        | expression @ E::MulAssign(_)
        | expression @ E::DivAssign(_)
        | expression @ E::AndAssign(_)
        | expression @ E::OrAssign(_)
        | expression @ E::XorAssign(_)
        | expression @ E::Assign(_) => compile_assign(expression,
                                                      symbol_alloc,
                                                      fn_alloc,
                                                      register_alloc,
                                                      statements),

        // function call
        E::Call(node) => todo!(),
        _ => todo!(),
    }
}

// TODO remove/replace code below

#[deprecated]
fn path_to_symbol_name(path: &Path<'_>) -> String {
    let mut items = path.iter();
    let name = items.next().unwrap().to_string();
    items.fold(name, |mut o, ident| {
             o.push_str("::");
             o.push_str(&ident.to_string());
             o
         })
}

// compile computation of the given expression and store the result in the given
// stack address (it is assume that the expression fits).
#[deprecated]
pub fn compile_expression_into_pointer<B: ByteOrder>(expression: &Expression<'_>,
                                                     layout: &Layout,
                                                     symbol_alloc: &SymbolAlloc<B>,
                                                     fn_alloc: &FnAlloc,
                                                     dst_base: Pointer,
                                                     register_alloc: &mut RegisterAlloc,
                                                     statements: &mut Vec<Statement>) {
    macro_rules! arithmetic_match_branch {
        ($node:expr, $var:ident) => {{
            let left = compile_expr(&$node.inner.left,
                                    symbol_alloc,
                                    fn_alloc,
                                    register_alloc,
                                    statements);
            let right = compile_expr(&$node.inner.right,
                                     symbol_alloc,
                                     fn_alloc,
                                     register_alloc,
                                     statements);
            free_source_registers(&left, register_alloc);
            free_source_registers(&right, register_alloc);
            statements.push($var { left,
                                   right,
                                   destination: Destination::Pointer { base: dst_base,
                                                                       offset: None } });
        }};
    }

    use super::Statement::{
        Add, And, Div, Eq, Greater, GreaterEq, Ld, LdAddr, LdW, LeftShift, Less, LessEq, Mul,
        NotEq, Or, RightShift, Sub, Xor,
    };

    match expression {
        // compile literal expression by simply move a literal value unto the stack address.
        // the size must be either a u8 or a u16 at this point. Any other value is wrong and the
        // compiler frontend should've caught it by now, hence the panic.
        expr @ Expression::Lit(_) => {
            let lit = const_expr(expr, Some(symbol_alloc)).unwrap();
            match layout {
                Layout::U8 => {
                    assert!(lit <= 0xff);
                    statements.push(Ld { source: Source::Literal(lit as u8),
                                         destination: Destination::Pointer { base: dst_base,
                                                                             offset: None } });
                }
                Layout::I8 => unimplemented!("TODO i8"),
                Layout::Pointer(_) => statements.push(LdW { source: Source::Literal(lit),
                                                  destination:
                                                      Destination::Pointer { base: dst_base,
                                                                             offset: None } }),
                _ => panic!(),
            }
        }
        Expression::Path(path) => {
            let name = path_to_symbol_name(path);
            let symbol = symbol_alloc.get(&name);
            // fallibility should be implemented in the frontend. If it panics here, it has
            // to be a bug.
            assert_eq!(layout, &symbol.layout);

            // byte by byte copy
            // TODO consider using a loop if the type is too large later on if
            //  code size gets too large.
            let src_base = match symbol.memory_space {
                SymbolMemorySpace::Static => Pointer::Static(symbol.offset),
                SymbolMemorySpace::Const => Pointer::Const(symbol.offset),
                SymbolMemorySpace::Stack => Pointer::Stack(symbol.offset),
                SymbolMemorySpace::Absolute => Pointer::Absolute(symbol.offset),
            };
            for offset in 0..layout.size() {
                let source = Source::Pointer { base: src_base.offset(offset),
                                               offset: None };
                let destination = Destination::Pointer { base: dst_base.offset(offset),
                                                         offset: None };
                statements.push(Ld { source,
                                     destination });
            }
        }
        Expression::Array(value) => match layout {
            Layout::Array { inner, len } => {
                assert_eq!(*len as usize, value.inner.len());

                let array_type_size = inner.size();

                for (i, expr) in value.inner.iter().enumerate() {
                    let offset = array_type_size * (i as u16);
                    compile_expression_into_pointer(expr,
                                                    inner,
                                                    symbol_alloc,
                                                    fn_alloc,
                                                    dst_base.offset(offset),
                                                    register_alloc,
                                                    statements);
                }
            }
            _ => panic!(),
        },
        Expression::Minus(_) => {}
        Expression::AddressOf(address_of) => match layout {
            Layout::Pointer(ptr) => {
                match &address_of.inner {
                    Expression::Path(path) => {
                        let name = path_to_symbol_name(path);
                        let symbol = symbol_alloc.get(&name);

                        // check layouts
                        assert_eq!(ptr.as_ref(), &symbol.layout);

                        let source_ptr = match symbol.memory_space {
                            SymbolMemorySpace::Stack => Pointer::Stack(symbol.offset),
                            SymbolMemorySpace::Static => Pointer::Static(symbol.offset),
                            SymbolMemorySpace::Const => Pointer::Const(symbol.offset),
                            SymbolMemorySpace::Absolute => Pointer::Absolute(symbol.offset),
                        };
                        statements.push(LdAddr { source: Source::Pointer { base: source_ptr,
                                                                           offset: None },
                                                 destination:
                                                     Destination::Pointer { base: dst_base,
                                                                            offset: None } });
                    }
                    Expression::Index(index) => {
                        match &index.inner.right {
                            Expression::Path(path) => {
                                let name = path_to_symbol_name(path);
                                let symbol = symbol_alloc.get(&name);

                                // TODO fix lint
                                #[allow(unused)]
                                if let Layout::Array { inner, len } = &symbol.layout {
                                    assert_eq!(ptr, inner);

                                    // TODO extend to non-constant expression indices
                                    let type_size = inner.size();
                                    let offset_const_expr =
                                        const_expr(&index.inner.left, Some(symbol_alloc)).unwrap();
                                    let offset = symbol.offset + type_size * offset_const_expr;

                                    let source_ptr = match symbol.memory_space {
                                        SymbolMemorySpace::Static => Pointer::Stack(offset),
                                        SymbolMemorySpace::Const => Pointer::Const(offset),
                                        SymbolMemorySpace::Stack => Pointer::Stack(offset),
                                        SymbolMemorySpace::Absolute => Pointer::Absolute(offset),
                                    };
                                    statements.push(LdAddr { source: Source::Pointer { base:
                                                                                 source_ptr,
                                                                             offset: None },
                                                   destination:
                                                       Destination::Pointer { base: dst_base,
                                                                              offset: None } });
                                } else {
                                    panic!()
                                }
                            }
                            _ => unimplemented!(),
                        }
                    }
                    // TODO generalise (allow taking a pointer of something other than just a
                    // symbol)
                    _ => unimplemented!(),
                }
            }
            _ => panic!(),
        },
        Expression::Deref(_) => {}
        Expression::Not(_) => {}

        // binary expressions
        Expression::Add(node) => arithmetic_match_branch!(node, Add),
        Expression::Sub(node) => arithmetic_match_branch!(node, Sub),
        Expression::Mul(node) => arithmetic_match_branch!(node, Mul),
        Expression::Div(node) => arithmetic_match_branch!(node, Div),
        Expression::And(node) => arithmetic_match_branch!(node, And),
        Expression::Or(node) => arithmetic_match_branch!(node, Or),
        Expression::Xor(node) => arithmetic_match_branch!(node, Xor),
        Expression::LeftShift(node) => arithmetic_match_branch!(node, LeftShift),
        Expression::RightShift(node) => arithmetic_match_branch!(node, RightShift),

        // boolean
        Expression::Eq(node) => arithmetic_match_branch!(node, Eq),
        Expression::NotEq(node) => arithmetic_match_branch!(node, NotEq),
        Expression::Greater(node) => arithmetic_match_branch!(node, Greater),
        Expression::GreaterEq(node) => arithmetic_match_branch!(node, GreaterEq),
        Expression::Less(node) => arithmetic_match_branch!(node, Less),
        Expression::LessEq(node) => arithmetic_match_branch!(node, LessEq),

        // assignment (these return void, so panic)
        Expression::Assign(_)
        | Expression::PlusAssign(_)
        | Expression::MinusAssign(_)
        | Expression::MulAssign(_)
        | Expression::DivAssign(_)
        | Expression::AndAssign(_)
        | Expression::OrAssign(_)
        | Expression::XorAssign(_) => panic!(),

        // TODO reimplement (only works for [u8 N] atm)
        Expression::Index(index) => {
            match &index.inner.right {
                Expression::Path(path) => {
                    let name = path_to_symbol_name(path);
                    let symbol = symbol_alloc.get(&name);

                    //assert_eq!(&Layout::Array {}, &symbol.layout);

                    // compute offset
                    let offset = compile_expr(&index.inner.left,
                                              symbol_alloc,
                                              fn_alloc,
                                              register_alloc,
                                              statements);
                    free_source_registers(&offset, register_alloc);
                    let base = match symbol.memory_space {
                        SymbolMemorySpace::Static => Pointer::Static(symbol.offset),
                        SymbolMemorySpace::Const => Pointer::Const(symbol.offset),
                        SymbolMemorySpace::Stack => Pointer::Stack(symbol.offset),
                        SymbolMemorySpace::Absolute => Pointer::Absolute(symbol.offset),
                    };
                    statements.push(Ld { source: Source::Pointer { base,
                                                                   offset:
                                                                       Some(Box::new(offset)) },
                                         destination: Destination::Pointer { base: dst_base,
                                                                             offset: None } });
                }
                _ => unimplemented!(),
            }
        }
        Expression::Call(call) => match &call.inner.left {
            Expression::Path(ident) => {
                let (fn_, routine) = fn_alloc.get(&path_to_symbol_name(ident));

                // check that the function returns the type we're trying to compile!
                assert_eq!(fn_.ret_layout.as_ref(), Some(layout));

                let args_call = &call.inner.args;
                let args_layout = &fn_.arg_layout;

                // TODO implement functions
                #[warn(unused)]
                let _destination = Some(Destination::Pointer { base: dst_base,
                                                               offset: None });

                assert_eq!(args_call.len(), args_layout.len());

                let mut offset = 0;
                let start = symbol_alloc.stack_address() - 1;

                for (call_arg, arg_layout) in args_call.iter().zip(args_layout) {
                    compile_expression_into_pointer(call_arg,
                                                    arg_layout,
                                                    symbol_alloc,
                                                    fn_alloc,
                                                    dst_base.offset(offset),
                                                    register_alloc,
                                                    statements);
                    offset += arg_layout.size();
                }

                // call the function and place the results in the stack
                statements.push(Statement::Call { routine,
                                                  range: start..(start + offset) });
                for i in 0..layout.size() {
                    let source = Source::Pointer { base: Pointer::Return(i),
                                                   offset: None };
                    let destination = Destination::Pointer { base: Pointer::Stack(start + i),
                                                             offset: None };
                    statements.push(Statement::Ld { source,
                                                    destination });
                }
            }
            _ => panic!(),
        },
    }
}

#[cfg(test)]
mod test {
    use crate::{byteorder::NativeEndian, parser::ast};

    #[test]
    fn const_expr() {
        fn ast(input: &str) -> ast::Expression<'_> {
            let mut ctx = crate::parser::ContextBuilder::default().build();
            let mut tokens = crate::parser::lex::Tokens::new(input).peekable();
            crate::parser::ast::Grammar::parse(&mut ctx, &mut tokens).expect("Error parsing constant expression")
        }

        assert_eq!(Some(0x42),
                   super::const_expr::<NativeEndian>(&ast("(+ 0x40 2)"), None));
    }
}
