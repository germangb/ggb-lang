use crate::{
    byteorder::ByteOrder,
    ir::{
        alloc::{FnAlloc, RegisterAlloc, Space, SymbolAlloc},
        layout::Layout,
        Destination, Pointer, Register, Source, Statement,
    },
    parser::ast::{Expression, Path},
};

// TODO consider removing this hack.
fn path_to_symbol_name(path: &Path<'_>) -> String {
    let mut items = path.iter();
    let name = items.next().unwrap().to_string();
    items.fold(name, |mut o, ident| {
             o.push_str("::");
             o.push_str(ident.as_str());
             o
         })
}

// TODO refactor, there's a lot of code repetition in the below functions, and
//  the only differences between them seems to be the destination of the
//  evaluated results.
//
// BRACE! CRAPPY (UNTESTED) COMPILATION CODE BELOW!
//
//                         .i;;;;i.
//                       iYcviii;vXY:
//                     .YXi       .i1c.
//                    .YC.     .    in7.
//                   .vc.   ......   ;1c.
//                   i7,   ..        .;1;
//                  i7,   .. ...      .Y1i
//                 ,7v     .6MMM@;     .YX,
//                .7;.   ..IMMMMMM1     :t7.
//               .;Y.     ;$MMMMMM9.     :tc.
//               vY.   .. .nMMM@MMU.      ;1v.
//              i7i   ...  .#MM@M@C. .....:71i
//             it:   ....   $MMM@9;.,i;;;i,;tti
//            :t7.  .....   0MMMWv.,iii:::,,;St.
//           .nC.   .....   IMMMQ..,::::::,.,czX.
//          .ct:   ....... .ZMMMI..,:::::::,,:76Y.
//          c2:   ......,i..Y$M@t..:::::::,,..inZY
//         vov   ......:ii..c$MBc..,,,,,,,,,,..iI9i
//        i9Y   ......iii:..7@MA,..,,,,,,,,,....;AA:
//       iIS.  ......:ii::..;@MI....,............;Ez.
//      .I9.  ......:i::::...8M1..................C0z.
//     .z9;  ......:i::::,.. .i:...................zWX.
//     vbv  ......,i::::,,.      ................. :AQY
//    c6Y.  .,...,::::,,..:t0@@QY. ................ :8bi
//   :6S. ..,,...,:::,,,..EMMMMMMI. ............... .;bZ,
//  :6o,  .,,,,..:::,,,..i#MMMMMM#v.................  YW2.
// .n8i ..,,,,,,,::,,,,.. tMMMMM@C:.................. .1Wn
// 7Uc. .:::,,,,,::,,,,..   i1t;,..................... .UEi
// 7C...::::::::::::,,,,..        ....................  vSi.
// ;1;...,,::::::,.........       ..................    Yz:
//  v97,.........                                     .voC.
//   izAotX7777777777777777777777777777777777777777Y7n92:
//     .;CoIIIIIUAA666666699999ZZZZZZZZZZZZZZZZZZZZ6ov.

// compile expression, and palces the result in a new register.
// the register is allocated using the passed "register_alloc", so it must be
// freed afterwards by the callee of the function.
pub fn compile_expr_register<B: ByteOrder>(expression: &Expression<'_>,
                                           layout: &Layout,
                                           symbol_alloc: &SymbolAlloc<B>,
                                           fn_alloc: &FnAlloc,
                                           register_alloc: &mut RegisterAlloc,
                                           statements: &mut Vec<Statement>)
                                           -> Register {
    // compile binary arithmetic expression.
    // returns the left, right and store registers.
    fn arithmetic_branch_match<B: ByteOrder>(left: &Expression<'_>,
                                             right: &Expression<'_>,
                                             layout: &Layout,
                                             symbol_alloc: &SymbolAlloc<B>,
                                             register_alloc: &mut RegisterAlloc,
                                             fn_alloc: &FnAlloc,
                                             statements: &mut Vec<Statement>)
                                             -> (Register, Register, Register) {
        let left = compile_expr_register(left,
                                         layout,
                                         symbol_alloc,
                                         fn_alloc,
                                         register_alloc,
                                         statements);
        let right = compile_expr_register(right,
                                          layout,
                                          symbol_alloc,
                                          fn_alloc,
                                          register_alloc,
                                          statements);
        // one of the two above registers must be freed (free),
        // the other one will hold the arithmetic operation result (store).
        let free = left.max(right);
        let store = left + right - free;
        register_alloc.free(free);
        (left, right, store)
    }

    macro_rules! arithmetic_binary_match_branch {
        ($node:expr, $var:ident, $var_w:ident) => {{
            let (left, right, store) = arithmetic_branch_match(&$node.inner.left,
                                                               &$node.inner.right,
                                                               layout,
                                                               symbol_alloc,
                                                               register_alloc,
                                                               fn_alloc,
                                                               statements);
            match layout {
                Layout::U8 | Layout::I8 => {
                    statements.push(Statement::$var { left: Source::Register(left),
                                                      right: Source::Register(right),
                                                      destination: Destination::Register(store) })
                }
                Layout::Pointer(_) => {
                    statements.push(Statement::$var_w { left: Source::Register(left),
                                                        right: Source::Register(right),
                                                        destination:
                                                            Destination::Register(store) });
                }
                _ => panic!(),
            }
            // return the store register, which must be freed later by the callee of the
            // function
            store
        }};
    }

    match expression {
        Expression::Add(node) => arithmetic_binary_match_branch!(node, Add, AddW),
        Expression::Sub(node) => arithmetic_binary_match_branch!(node, Sub, SubW),
        Expression::And(node) => arithmetic_binary_match_branch!(node, And, AndW),
        Expression::Or(node) => arithmetic_binary_match_branch!(node, Or, OrW),
        Expression::Xor(node) => arithmetic_binary_match_branch!(node, Xor, XorW),
        Expression::Mul(node) => arithmetic_binary_match_branch!(node, Mul, MulW),
        Expression::Div(node) => arithmetic_binary_match_branch!(node, Div, DivW),
        Expression::LeftShift(node) => arithmetic_binary_match_branch!(node, LeftShift, LeftShiftW),
        Expression::RightShift(node) => {
            arithmetic_binary_match_branch!(node, RightShift, RightShiftW)
        }
        Expression::Not(not) => {
            let register = compile_expr_register(&not.inner,
                                                 layout,
                                                 symbol_alloc,
                                                 fn_alloc,
                                                 register_alloc,
                                                 statements);
            match layout {
                Layout::U8 | Layout::I8 => {
                    statements.push(Statement::Xor { left: Source::Register(register),
                                                     right: Source::Literal(0xff),
                                                     destination:
                                                         Destination::Register(register) });
                }
                Layout::Pointer(_) => {
                    statements.push(Statement::XorW { left: Source::Register(register),
                                                      right: Source::Literal(0xffff),
                                                      destination:
                                                          Destination::Register(register) });
                }
                _ => panic!(),
            }

            register
        }
        Expression::Path(path) => {
            let symbol_name = path_to_symbol_name(path);
            let symbol = symbol_alloc.get(&symbol_name);
            let pointer = match symbol.space {
                Space::Static => Pointer::Static(symbol.offset),
                Space::Const => Pointer::Const(symbol.offset),
                Space::Stack => Pointer::Stack(symbol.offset),
                Space::Absolute => Pointer::Absolute(symbol.offset),
            };
            let register = register_alloc.alloc();
            match layout {
                Layout::U8 => {
                    statements.push(Statement::Ld { source: Source::Pointer { base: pointer,
                                                                              offset: None },
                                                    destination:
                                                        Destination::Register(register) });
                }
                Layout::Pointer(_) => {
                    statements.push(Statement::LdW { source: Source::Pointer { base: pointer,
                                                                               offset: None },
                                                     destination:
                                                         Destination::Register(register) });
                }
                _ => panic!(),
            }
            register
        }
        expr @ Expression::Lit(_) => {
            let lit = compute_const_expr(expr);
            let register = register_alloc.alloc();
            match layout {
                Layout::U8 => {
                    assert!(lit <= 0xff);
                    statements.push(Statement::Ld { source: Source::Literal(lit as u8),
                                                    destination:
                                                        Destination::Register(register) });
                }
                Layout::Pointer(_) => {
                    statements.push(Statement::LdW { source: Source::Literal(lit),
                                                     destination:
                                                         Destination::Register(register) });
                }
                Layout::I8 => {
                    assert!(lit <= i8::max_value() as _);
                    assert!(lit >= i8::min_value() as _);
                    unimplemented!("TODO i8")
                }
                _ => panic!(),
            }
            register
        }
        call @ Expression::Call(_) => {
            let stack_address = symbol_alloc.stack_address();
            compile_expression_into_pointer(call,
                                            layout,
                                            symbol_alloc,
                                            fn_alloc,
                                            Pointer::Stack(stack_address),
                                            register_alloc,
                                            statements);
            let register = register_alloc.alloc();
            statements.push(Statement::Ld { source:
                                                Source::Pointer { base:
                                                                      Pointer::Stack(stack_address),
                                                                  offset: None },
                                            destination: Destination::Register(register) });
            register
        }
        // FIXME assuming foo[n] where foo is a byte array and n a word
        Expression::Index(index) => {
            match &index.inner.right {
                Expression::Path(path) => {
                    let name = path_to_symbol_name(path);
                    let symbol = symbol_alloc.get(&name);
                    //assert_eq!(&Layout::Array {}, &symbol.layout);
                    // compute offset
                    let offset_register =
                        compile_expr_register(&index.inner.left,
                                              &Layout::Pointer(Box::new(Layout::U8)),
                                              symbol_alloc,
                                              fn_alloc,
                                              register_alloc,
                                              statements);
                    let base = match symbol.space {
                        Space::Static => Pointer::Static(symbol.offset),
                        Space::Const => Pointer::Const(symbol.offset),
                        Space::Stack => Pointer::Stack(symbol.offset),
                        Space::Absolute => Pointer::Absolute(symbol.offset),
                    };
                    let register = register_alloc.alloc();
                    statements.push(Statement::Ld {
                        source: Source::Pointer {
                            base,
                            offset: Some(Box::new(Source::Register(offset_register))),
                        },
                        destination: Destination::Register(register),
                    });

                    register_alloc.free(offset_register);
                    register
                }
                _ => unimplemented!(),
            }
        }
        _ => panic!(),
    }
}

// compile computation of the given expression and store the result in the given
// stack address (it is assume that the expression fits).
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

    use super::Statement::*;

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
                        let name = path_to_symbol_name(&path);
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
        Expression::Eq(_) => unimplemented!(),
        Expression::NotEq(_) => unimplemented!(),
        Expression::LessEq(_) => unimplemented!(),
        Expression::GreaterEq(_) => unimplemented!(),
        Expression::Less(_) => unimplemented!(),
        Expression::Greater(_) => unimplemented!(),

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
                    let offset_register =
                        compile_expr_register(&index.inner.left,
                                              &Layout::Pointer(Box::new(Layout::U8)),
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
                            offset: Some(Box::new(Source::Register(offset_register))),
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
                let (fn_, routine) = fn_alloc.get(&path_to_symbol_name(&ident));

                // check that the function returns the type we're trying to compile!
                assert_eq!(fn_.ret_layout.as_ref(), Some(layout));

                let args_call = &call.inner.args;
                let args_layout = &fn_.arg_layout;

                // TODO implement functions
                #[warn(unused)]
                let destination = Some(Destination::Pointer { base: dst_base,
                                                              offset: None });

                assert_eq!(args_call.len(), args_layout.len());

                let mut offset = 0;

                // layout functions arguments in the stack
                let mut alloc = symbol_alloc.clone();
                alloc.clear_stack();

                for (call_arg, arg_layout) in args_call.iter().zip(args_layout) {
                    compile_expression_into_pointer(call_arg,
                                                    &arg_layout,
                                                    &mut alloc,
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

// compiles the evaluation of an expression, throwing away the result.
// examples: function calls, assignments, ...
pub fn compile_expr_void<B: ByteOrder>(expression: &Expression<'_>,
                                       symbol_alloc: &SymbolAlloc<B>,
                                       fn_alloc: &FnAlloc,
                                       register_alloc: &mut RegisterAlloc,
                                       statements: &mut Vec<Statement>) {
    // TODO placeholder implementation to begin running programs in the VM...
    use Expression as E;
    use Statement::*;

    macro_rules! arithmetic_assign_match_branch {
        ($node:expr, $var:ident) => {{
            let (left, right, destination) = arithmetic_assign(&$node.inner.left,
                                                               &$node.inner.right,
                                                               symbol_alloc,
                                                               register_alloc,
                                                               fn_alloc,
                                                               statements);
            statements.push($var { left,
                                   right,
                                   destination });
        }};
    }

    match expression {
        // superfluous expressions
        E::Lit(_) | E::Path(_) => {}

        // arithmetic op + assign expressions
        // (these return "void", so no value)
        E::PlusAssign(node) => arithmetic_assign_match_branch!(node, Add),
        E::MinusAssign(node) => arithmetic_assign_match_branch!(node, Sub),
        E::MulAssign(node) => arithmetic_assign_match_branch!(node, Mul),
        E::DivAssign(node) => arithmetic_assign_match_branch!(node, Div),
        E::AndAssign(node) => arithmetic_assign_match_branch!(node, And),
        E::OrAssign(node) => arithmetic_assign_match_branch!(node, Or),
        E::XorAssign(node) => arithmetic_assign_match_branch!(node, Xor),
        E::Assign(node) => {
            // value can't be freed before the call to destination, because that function
            // allocated a new register! The register must be freed at the end of this match
            // arm.
            let value = compile_expr_register(&node.inner.right,
                                              &Layout::U8,
                                              symbol_alloc,
                                              fn_alloc,
                                              register_alloc,
                                              statements);
            // compute the destination of the assignment.
            #[warn(unused)]
            let (destination, layout) = compute_destination_and_layout(&node.inner.left,
                                                                       symbol_alloc,
                                                                       fn_alloc,
                                                                       register_alloc,
                                                                       statements);
            statements.push(Ld { source: Source::Register(value),
                                 destination });
            register_alloc.free(value);
        }
        #[warn(unused)]
        E::Call(node) => unimplemented!(),
        _ => unimplemented!(),
    }

    // function to generate the arithmetic+assign expression
    // foo += bar
    // foo -= bar
    // foo &= bar
    // and so on...
    fn arithmetic_assign<B: ByteOrder>(left: &Expression<'_>,
                                       right: &Expression<'_>,
                                       symbol_alloc: &SymbolAlloc<B>,
                                       register_alloc: &mut RegisterAlloc,
                                       fn_alloc: &FnAlloc,
                                       statements: &mut Vec<Statement>)
                                       -> (Source<u8>, Source<u8>, Destination) {
        let register = compile_expr_register(right,
                                             &Layout::U8,
                                             symbol_alloc,
                                             fn_alloc,
                                             register_alloc,
                                             statements);
        register_alloc.free(register);
        match left {
            E::Path(path) => {
                let name = path_to_symbol_name(path);
                let symbol = symbol_alloc.get(&name);
                let base = match symbol.space {
                    Space::Static => Pointer::Static(symbol.offset),
                    Space::Const => Pointer::Const(symbol.offset),
                    Space::Stack => Pointer::Stack(symbol.offset),
                    Space::Absolute => Pointer::Absolute(symbol.offset),
                };
                // return the lhs and rhs of the expression
                (Source::Pointer { base, offset: None },
                 Source::Register(register),
                 Destination::Pointer { base, offset: None })
            }
            E::Index(index) => {
                match &index.inner.right {
                    // <path>[E] = E
                    E::Path(path) => {
                        let name = path_to_symbol_name(path);
                        let symbol = symbol_alloc.get(&name);

                        //assert_eq!(&Layout::Array {}, &symbol.layout);

                        // compute offset
                        let offset_register =
                            compile_expr_register(&index.inner.left,
                                                  &Layout::Pointer(Box::new(Layout::U8)),
                                                  symbol_alloc,
                                                  fn_alloc,
                                                  register_alloc,
                                                  statements);
                        let base = match symbol.space {
                            Space::Static => Pointer::Static(symbol.offset),
                            Space::Const => Pointer::Const(symbol.offset),
                            Space::Stack => Pointer::Stack(symbol.offset),
                            Space::Absolute => Pointer::Absolute(symbol.offset),
                        };
                        register_alloc.free(offset_register);
                        let offset = Box::new(Source::Register(offset_register));
                        (Source::Pointer { base,
                                           offset: Some(offset.clone()) },
                         Source::Register(register),
                         Destination::Pointer { base,
                                                offset: Some(offset) })
                    }
                    _ => unimplemented!(),
                }
            }
            _ => unimplemented!(),
        }
    }
}

// Compute the destination corresponding to a lhs assignment expression.
// Also returns the layout of the data to be assigned.
fn compute_destination_and_layout<B: ByteOrder>(expression: &Expression<'_>,
                                                symbol_alloc: &SymbolAlloc<B>,
                                                fn_alloc: &FnAlloc,
                                                register_alloc: &mut RegisterAlloc,
                                                statements: &mut Vec<Statement>)
                                                -> (Destination, Layout) {
    use Expression as E;
    match expression {
        E::Path(path) => {
            let symbol_name = path_to_symbol_name(path);
            let symbol = symbol_alloc.get(&symbol_name);
            let base = match symbol.space {
                Space::Static => Pointer::Static(symbol.offset),
                Space::Const => Pointer::Const(symbol.offset),
                Space::Stack => Pointer::Stack(symbol.offset),
                Space::Absolute => Pointer::Absolute(symbol.offset),
            };

            (Destination::Pointer { base, offset: None }, symbol.layout.clone())
        }
        #[warn(unused)]
        E::Deref(deref) => unimplemented!(),
        E::Index(index) => {
            let offset = compile_expr_register(&index.inner.left,
                                               &Layout::Pointer(Box::new(Layout::U8)),
                                               symbol_alloc,
                                               fn_alloc,
                                               register_alloc,
                                               statements);
            register_alloc.free(offset);
            match &index.inner.right {
                E::Path(path) => {
                    let symbol_name = path_to_symbol_name(path);
                    let symbol = symbol_alloc.get(&symbol_name);
                    let base = match symbol.space {
                        Space::Static => Pointer::Static(symbol.offset),
                        Space::Const => Pointer::Const(symbol.offset),
                        Space::Stack => Pointer::Stack(symbol.offset),
                        Space::Absolute => Pointer::Absolute(symbol.offset),
                    };

                    let layout = match &symbol.layout {
                        Layout::Array { inner, .. } => *inner.clone(),
                        _ => panic!(),
                    };

                    (Destination::Pointer { base,
                                            offset: Some(Box::new(Source::Register(offset))) },
                     layout)
                }
                _ => panic!(),
            }
        }
        _ => panic!(),
    }
}

/// Compute the size of a given constant (numeric) expression.
/// Panics if the expression is not a constant expression nor numeric.
pub fn compute_const_expr(expr: &Expression<'_>) -> u16 {
    use Expression::*;

    match expr {
        Lit(lit) => {
            let num = lit.to_string();
            if num.starts_with("0x") {
                u16::from_str_radix(&num[2..], 16).expect("Not a hex number")
            } else if num.starts_with("0") && num.len() > 1 {
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
