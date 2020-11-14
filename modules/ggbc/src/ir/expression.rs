use crate::{
    byteorder::ByteOrder,
    ir::{
        alloc::{FnAlloc, RegisterAlloc, Space, SymbolAlloc},
        layout::Layout,
        Destination, Offset, Pointer, Register, Source, Statement,
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
#[warn(unused)]
pub fn free_source_registers(source: &Source<u8>, register_alloc: &mut RegisterAlloc) {
    use Offset::*;
    use Source::*;
    match source {
        Register(r) => register_alloc.free(*r),
        Pointer { offset: Some(o), .. } => match o.as_ref() {
            U8(o) => free_source_registers(o, register_alloc),
            _ => {}
        },
        _ => {}
    }
}

/// Utility function to free any registers referenced within a given
/// `Destination`.
#[warn(unused)]
pub fn free_destination_registers(destination: &Destination, register_alloc: &mut RegisterAlloc) {
    use Destination::*;
    use Offset::*;
    match destination {
        Pointer { offset: Some(offset),
                  .. } => match offset.as_ref() {
            U8(source) => free_source_registers(source, register_alloc),
            U16(_source) => todo!(),
        },
        _ => {}
    }
}

#[warn(unused)]
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
#[warn(unused)]
pub fn const_expr<B: ByteOrder>(expression: &Expression<'_>,
                                symbol_alloc: &SymbolAlloc<B>)
                                -> Option<u16> {
    use Expression as E;
    match expression {
        E::Path(path) => {
            let name = path_to_symbol_name(path);
            let symbol = symbol_alloc.get(&name);
            match symbol.space {
                Space::Const => Some(symbol_alloc.const_data()[symbol.offset as usize] as _),
                _ => None,
            }
        }
        E::Lit(lit) => {
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
        E::Minus(_e) => todo!(),
        E::Not(e) => Some(!const_expr(&e.inner, symbol_alloc)?),
        E::Add(e) => Some(const_expr(&e.inner.left, symbol_alloc)?
                          + const_expr(&e.inner.right, symbol_alloc)?),
        E::Sub(e) => Some(const_expr(&e.inner.left, symbol_alloc)?
                          - const_expr(&e.inner.right, symbol_alloc)?),
        E::Mul(e) => Some(const_expr(&e.inner.left, symbol_alloc)?
                          * const_expr(&e.inner.right, symbol_alloc)?),
        E::Div(e) => Some(const_expr(&e.inner.left, symbol_alloc)?
                          / const_expr(&e.inner.right, symbol_alloc)?),
        E::And(e) => Some(const_expr(&e.inner.left, symbol_alloc)?
                          & const_expr(&e.inner.right, symbol_alloc)?),
        E::Or(e) => Some(const_expr(&e.inner.left, symbol_alloc)?
                         | const_expr(&e.inner.right, symbol_alloc)?),
        E::Xor(e) => Some(const_expr(&e.inner.left, symbol_alloc)?
                          ^ const_expr(&e.inner.right, symbol_alloc)?),
        E::LeftShift(e) => Some(const_expr(&e.inner.left, symbol_alloc)?
                                << const_expr(&e.inner.right, symbol_alloc)?),
        E::RightShift(e) => Some(const_expr(&e.inner.left, symbol_alloc)?
                                 >> const_expr(&e.inner.right, symbol_alloc)?),
        _ => None,
    }
}

/// Compile assignment statement/expression.
/// These expressions evaluate to no value in particular.
#[warn(unused)]
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
#[warn(unused)]
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
            match_expr!(&mut destination, Destination::Pointer, offset).replace(Box::new(Offset::U8(offset)));
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
#[warn(unused)]
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
    if let Some(n) = const_expr(expression, symbol_alloc) {
        assert!(n < 0xff); // TODO wrap?
        return Source::Literal(n as u8);
    }

    match expression {
        // TODO numeric const expressions are handled by the above statement, but what if expression
        //  is a string literal?
        E::Lit(_) => todo!(),
        // If the symbol resolves to an U8...
        E::Path(path) => {
            let symbol_name = path_to_symbol_name(path);
            let symbol = symbol_alloc.get(&symbol_name);
            assert!(matches!(&symbol.layout, Layout::U8));
            Source::Pointer { base: symbol.pointer(),
                              offset: None }
        }
        // 8bit binary expressions
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
        E::Eq(node) => arithmetic_branch!(Eq, node),
        E::NotEq(node) => arithmetic_branch!(NotEq, node),
        E::Greater(node) => arithmetic_branch!(Greater, node),
        E::GreaterEq(node) => arithmetic_branch!(GreaterEq, node),
        E::Less(node) => arithmetic_branch!(Less, node),
        E::LessEq(node) => arithmetic_branch!(LessEq, node),
        // Arrays ([left]right)
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
                              offset: Some(Box::new(Offset::U8(offset))) }
        }
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

// compile expression, and place the result in a new register.
// the register is allocated using the passed "register_alloc", so it must be
// freed afterwards by the callee of the function.
#[deprecated]
#[warn(unused)]
pub fn compile_expr_register<B: ByteOrder>(expression: &Expression<'_>,
                                           layout: &Layout,
                                           symbol_alloc: &SymbolAlloc<B>,
                                           fn_alloc: &FnAlloc,
                                           register_alloc: &mut RegisterAlloc,
                                           statements: &mut Vec<Statement>)
                                           -> Register {
    let source = compile_expr(expression,
                              symbol_alloc,
                              fn_alloc,
                              register_alloc,
                              statements);
    free_source_registers(&source, register_alloc);
    let register = register_alloc.alloc();
    statements.push(Statement::Ld { source,
                                    destination: Destination::Register(register) });
    register
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
            let left = compile_expr_register(&$node.inner.left,
                                             layout,
                                             symbol_alloc,
                                             fn_alloc,
                                             register_alloc,
                                             statements);
            let right = compile_expr_register(&$node.inner.right,
                                              layout,
                                              symbol_alloc,
                                              fn_alloc,
                                              register_alloc,
                                              statements);
            statements.push($var { left: Source::Register(left),
                                   right: Source::Register(right),
                                   destination: Destination::Pointer { base: dst_base,
                                                                       offset: None } });
            register_alloc.free(left);
            register_alloc.free(right);
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
            let lit = compute_const_expr(expr);
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
            let src_base = match symbol.space {
                Space::Static => Pointer::Stack(symbol.offset),
                Space::Const => Pointer::Const(symbol.offset),
                Space::Stack => Pointer::Stack(symbol.offset),
                Space::Absolute => Pointer::Absolute(symbol.offset),
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

                        let source_ptr = match symbol.space {
                            Space::Stack => Pointer::Stack(symbol.offset),
                            Space::Static => Pointer::Static(symbol.offset),
                            Space::Const => Pointer::Const(symbol.offset),
                            Space::Absolute => Pointer::Absolute(symbol.offset),
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
                                    let offset_const_expr = compute_const_expr(&index.inner.left);
                                    let offset = symbol.offset + type_size * offset_const_expr;

                                    let source_ptr = match symbol.space {
                                        Space::Static => Pointer::Stack(offset),
                                        Space::Const => Pointer::Const(offset),
                                        Space::Stack => Pointer::Stack(offset),
                                        Space::Absolute => Pointer::Absolute(offset),
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
                    let offset_register = compile_expr_register(&index.inner.left,
                                                                &Layout::U8,
                                                                symbol_alloc,
                                                                fn_alloc,
                                                                register_alloc,
                                                                statements);
                    let base_ptr = match symbol.space {
                        Space::Static => Pointer::Static(symbol.offset),
                        Space::Const => Pointer::Const(symbol.offset),
                        Space::Stack => Pointer::Stack(symbol.offset),
                        Space::Absolute => Pointer::Absolute(symbol.offset),
                    };
                    statements.push(Ld {
                        source: Source::Pointer {
                            base: base_ptr,
                            offset: Some(Box::new(Offset::U8(Source::Register(offset_register)))),
                        },
                        destination: Destination::Pointer {
                            base: dst_base,
                            offset: None,
                        },
                    });

                    register_alloc.free(offset_register);
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

                // layout functions arguments in the stack
                let mut alloc = symbol_alloc.clone();
                alloc.clear_stack();

                for (call_arg, arg_layout) in args_call.iter().zip(args_layout) {
                    compile_expression_into_pointer(call_arg,
                                                    arg_layout,
                                                    &alloc,
                                                    fn_alloc,
                                                    dst_base.offset(offset),
                                                    register_alloc,
                                                    statements);
                    offset += arg_layout.size();
                }

                statements.push(Statement::Call { routine,
                                                  address: offset })
            }
            _ => panic!(),
        },
    }
}

/// Compute the size of a given constant (numeric) expression.
/// Panics if the expression is not a constant expression nor numeric.
#[deprecated]
pub fn compute_const_expr(expr: &Expression<'_>) -> u16 {
    use Expression::{Add, And, Div, LeftShift, Lit, Minus, Mul, Not, Or, RightShift, Sub, Xor};

    match expr {
        Lit(lit) => {
            let num = lit.to_string();
            if num.starts_with("0x") {
                u16::from_str_radix(&num[2..], 16).expect("Not a hex number")
            } else if num.starts_with('0') && num.len() > 1 {
                u16::from_str_radix(&num[1..], 8).expect("Not an octal number")
            } else if num.starts_with("0b") {
                u16::from_str_radix(&num[2..], 2).expect("Not a bin number")
            } else {
                u16::from_str_radix(&num[..], 10).expect("Not a decimal number")
            }
        }
        Minus(_e) => unimplemented!("TODO"),
        Not(e) => !compute_const_expr(&e.inner),
        Add(e) => compute_const_expr(&e.inner.left) + compute_const_expr(&e.inner.right),
        Sub(e) => compute_const_expr(&e.inner.left) - compute_const_expr(&e.inner.right),
        Mul(e) => compute_const_expr(&e.inner.left) * compute_const_expr(&e.inner.right),
        Div(e) => compute_const_expr(&e.inner.left) / compute_const_expr(&e.inner.right),
        And(e) => compute_const_expr(&e.inner.left) & compute_const_expr(&e.inner.right),
        Or(e) => compute_const_expr(&e.inner.left) | compute_const_expr(&e.inner.right),
        Xor(e) => compute_const_expr(&e.inner.left) ^ compute_const_expr(&e.inner.right),
        LeftShift(e) => compute_const_expr(&e.inner.left) << compute_const_expr(&e.inner.right),
        RightShift(e) => compute_const_expr(&e.inner.left) >> compute_const_expr(&e.inner.right),
        _ => panic!("Not a constant expression"),
    }
}

#[deprecated]
pub fn compute_const_expr_into_vec<B: ByteOrder>(layout: &Layout,
                                                 expression: &Expression<'_>,
                                                 out: &mut Vec<u8>) {
    match (layout, expression) {
        (Layout::U8, expression) => {
            let lit = super::expression::compute_const_expr(expression);
            assert!(lit <= 0xff);
            out.push(lit as u8);
        }
        (Layout::I8, expression) => {
            let lit = super::expression::compute_const_expr(expression);
            assert!(lit <= i8::max_value() as u16 && lit >= i8::min_value() as u16);
            out.push(unsafe { std::mem::transmute(lit as i8) });
        }
        (Layout::Pointer(_), expr @ Expression::Lit(_)) => {
            let lit = super::expression::compute_const_expr(expr);
            let offset = out.len();
            // append value with the correct endianness
            out.push(0);
            out.push(0);
            B::write_u16(&mut out[offset..], lit);
        }
        (Layout::Array { inner, len }, Expression::Array(array)) => {
            assert_eq!(*len as usize, array.inner.len());
            for item in &array.inner {
                compute_const_expr_into_vec::<B>(inner, item, out);
            }
        }
        _ => panic!(),
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn const_expr() {
        let mut ctx = crate::parser::ContextBuilder::default().build();
        let mut tokens = crate::parser::lex::Tokens::new("(+ 0x0505 0xfafa)").peekable();
        let expr = crate::parser::ast::Grammar::parse(&mut ctx, &mut tokens).unwrap();

        assert_eq!(0xffff, super::compute_const_expr(&expr));
    }
}
