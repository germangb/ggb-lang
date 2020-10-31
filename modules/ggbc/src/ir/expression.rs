use crate::{
    byteorder::ByteOrder,
    ir::{
        alloc::{FnAlloc, RegisterAlloc, Space, SymbolAlloc},
        layout::Layout,
        Destination, Pointer, Register, Source, Statement,
    },
    parser::{
        ast::{Expression, Path, Type},
        lex::Lit,
    },
};

// TODO consider removing this hack. Converting from path to symbol name should
//  be agnostic of the program syntax.
fn path_to_symbol_name(path: &Path) -> String {
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

pub fn compile_expression_into_register<B: ByteOrder>(expression: &Expression,
                                                      layout: &Layout,
                                                      symbol_alloc: &mut SymbolAlloc<B>,
                                                      register_alloc: &mut RegisterAlloc,
                                                      fn_alloc: &FnAlloc,
                                                      stack_address: u16,
                                                      statements: &mut Vec<Statement>)
                                                      -> Register {
    match expression {
        Expression::Not(not) => {
            let register = compile_expression_into_register(&not.inner,
                                                            layout,
                                                            symbol_alloc,
                                                            register_alloc,
                                                            fn_alloc,
                                                            stack_address,
                                                            statements);
            match layout {
                Layout::U8 => {
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
                    statements.push(Statement::Ldw { source: Source::Pointer { base: pointer,
                                                                               offset: None },
                                                     destination:
                                                         Destination::Register(register) });
                }
                _ => panic!(),
            }
            register
        }

        Expression::Lit(lit) => {
            let lit = compute_literal_as_numeric(lit);
            let register = register_alloc.alloc();
            match layout {
                Layout::U8 => {
                    assert!(lit <= 0xff);
                    statements.push(Statement::Ld { source: Source::Literal(lit as u8),
                                                    destination:
                                                        Destination::Register(register) });
                }
                Layout::Pointer(_) => {
                    statements.push(Statement::Ldw { source: Source::Literal(lit),
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
            compile_expression_into_stack(call,
                                          layout,
                                          symbol_alloc,
                                          register_alloc,
                                          fn_alloc,
                                          stack_address,
                                          0,
                                          statements);
            let register = register_alloc.alloc();
            statements.push(Statement::Ld { source:
                                                Source::Pointer { base:
                                                                      Pointer::Stack(stack_address),
                                                                  offset: None },
                                            destination: Destination::Register(register) });
            register
        }

        Expression::Add(add) => {
            let left = compile_expression_into_register(&add.inner.left,
                                                        layout,
                                                        symbol_alloc,
                                                        register_alloc,
                                                        fn_alloc,
                                                        stack_address,
                                                        statements);
            let right = compile_expression_into_register(&add.inner.right,
                                                         layout,
                                                         symbol_alloc,
                                                         register_alloc,
                                                         fn_alloc,
                                                         stack_address,
                                                         statements);
            let free_register = left.max(right);
            let store_register = left + right - free_register;
            match layout {
                Layout::U8 => {
                    statements.push(Statement::Add { left: Source::Register(left),
                                                     right: Source::Register(right),
                                                     destination:
                                                         Destination::Register(store_register) });
                }
                Layout::Pointer(_) => {
                    statements.push(Statement::AddW { left: Source::Register(left),
                                                      right: Source::Register(right),
                                                      destination:
                                                          Destination::Register(store_register) });
                }
                _ => panic!(),
            }

            register_alloc.free(free_register);
            store_register
        }

        Expression::Sub(add) => {
            let left = compile_expression_into_register(&add.inner.left,
                                                        layout,
                                                        symbol_alloc,
                                                        register_alloc,
                                                        fn_alloc,
                                                        stack_address,
                                                        statements);
            let right = compile_expression_into_register(&add.inner.right,
                                                         layout,
                                                         symbol_alloc,
                                                         register_alloc,
                                                         fn_alloc,
                                                         stack_address,
                                                         statements);
            let free_register = left.max(right);
            let store_register = left + right - free_register;
            register_alloc.free(free_register);
            match layout {
                Layout::U8 => {
                    statements.push(Statement::Sub { left: Source::Register(left),
                                                     right: Source::Register(right),
                                                     destination:
                                                         Destination::Register(store_register) });
                }
                Layout::Pointer(_) => {
                    statements.push(Statement::SubW { left: Source::Register(left),
                                                      right: Source::Register(right),
                                                      destination:
                                                          Destination::Register(store_register) });
                }
                _ => panic!(),
            }
            store_register
        }

        Expression::And(and) => {
            let left = compile_expression_into_register(&and.inner.left,
                                                        layout,
                                                        symbol_alloc,
                                                        register_alloc,
                                                        fn_alloc,
                                                        stack_address,
                                                        statements);
            let right = compile_expression_into_register(&and.inner.right,
                                                         layout,
                                                         symbol_alloc,
                                                         register_alloc,
                                                         fn_alloc,
                                                         stack_address,
                                                         statements);

            let free_register = left.max(right);
            let store_register = left + right - free_register;
            register_alloc.free(free_register);
            match layout {
                Layout::U8 => {
                    statements.push(Statement::And { left: Source::Register(left),
                                                     right: Source::Register(right),
                                                     destination:
                                                         Destination::Register(store_register) });
                }
                Layout::Pointer(_) => {
                    statements.push(Statement::AndW { left: Source::Register(left),
                                                      right: Source::Register(right),
                                                      destination:
                                                          Destination::Register(store_register) });
                }
                _ => panic!(),
            }
            store_register
        }

        Expression::Or(or) => {
            let left = compile_expression_into_register(&or.inner.left,
                                                        layout,
                                                        symbol_alloc,
                                                        register_alloc,
                                                        fn_alloc,
                                                        stack_address,
                                                        statements);
            let right = compile_expression_into_register(&or.inner.right,
                                                         layout,
                                                         symbol_alloc,
                                                         register_alloc,
                                                         fn_alloc,
                                                         stack_address,
                                                         statements);
            let free_register = left.max(right);
            let store_register = left + right - free_register;
            register_alloc.free(free_register);
            match layout {
                Layout::U8 => {
                    statements.push(Statement::Or { left: Source::Register(left),
                                                    right: Source::Register(right),
                                                    destination:
                                                        Destination::Register(store_register) });
                }
                Layout::Pointer(_) => {
                    statements.push(Statement::OrW { left: Source::Register(left),
                                                     right: Source::Register(right),
                                                     destination:
                                                         Destination::Register(store_register) });
                }
                _ => panic!(),
            }
            store_register
        }

        Expression::Xor(xor) => {
            let left = compile_expression_into_register(&xor.inner.left,
                                                        layout,
                                                        symbol_alloc,
                                                        register_alloc,
                                                        fn_alloc,
                                                        stack_address,
                                                        statements);
            let right = compile_expression_into_register(&xor.inner.right,
                                                         layout,
                                                         symbol_alloc,
                                                         register_alloc,
                                                         fn_alloc,
                                                         stack_address,
                                                         statements);
            let free_register = left.max(right);
            let store_register = left + right - free_register;
            register_alloc.free(free_register);
            match layout {
                Layout::U8 => {
                    statements.push(Statement::Xor { left: Source::Register(left),
                                                     right: Source::Register(right),
                                                     destination:
                                                         Destination::Register(store_register) });
                }
                Layout::Pointer(_) => {
                    statements.push(Statement::XorW { left: Source::Register(left),
                                                      right: Source::Register(right),
                                                      destination:
                                                          Destination::Register(store_register) });
                }
                _ => panic!(),
            }
            store_register
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
                        compile_expression_into_register(&index.inner.left,
                                                         &Layout::Pointer(Box::new(Layout::U8)),
                                                         &mut symbol_alloc.clone(),
                                                         register_alloc,
                                                         fn_alloc,
                                                         symbol_alloc.stack_address(),
                                                         statements);
                    let base = match symbol.space {
                        Space::Static => Pointer::Static(symbol.offset),
                        Space::Const => Pointer::Const(symbol.offset),
                        Space::Stack => Pointer::Stack(symbol.offset),
                        Space::Absolute => Pointer::Absolute(symbol.offset),
                    };
                    let register = register_alloc.alloc();
                    statements.push(Statement::Ld { source: Source::Pointer { base,
                        offset: Some(Box::new(Source::Register(offset_register))) },
                        destination: Destination::Register(register),
                    });

                    register_alloc.free(offset_register);
                    register
                }
                _ => unimplemented!(),
            }
        }
        _ => unimplemented!(),
    }
}

// compile computation of the given expression and store the result in the given
// stack address (it is assume that the expression fits).
pub fn compile_expression_into_stack<B: ByteOrder>(expression: &Expression,
                                                   layout: &Layout,
                                                   symbol_alloc: &mut SymbolAlloc<B>,
                                                   register_alloc: &mut RegisterAlloc,
                                                   fn_alloc: &FnAlloc,
                                                   stack_address: u16,
                                                   offset_address: u16,
                                                   statements: &mut Vec<Statement>) {
    match expression {
        // compile literal expression by simply move a literal value unto the stack address.
        // the size must be either a u8 or a u16 at this point. Any other value is wrong and the
        // compiler frontend should've caught it by now, hence the panic.
        Expression::Lit(lit) => {
            let lit = compute_literal_as_numeric(lit);
            match layout {
                Layout::U8 => {
                    assert!(lit <= 0xff);
                    statements.push(Statement::Ld {
                        source: Source::Literal(lit as u8),
                        destination: Destination::Pointer { base: Pointer::Stack(stack_address),
                                                            offset: None } });
                }
                Layout::I8 => unimplemented!("TODO i8"),
                Layout::Pointer(_) => statements.push(Statement::Ldw {
                    source: Source::Literal(lit),
                    destination: Destination::Pointer { base: Pointer::Stack(stack_address),
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
            let source_offset = symbol.offset;
            let target_offset = stack_address;
            for offset in 0..layout.size() {
                let base = match symbol.space {
                    Space::Static => Pointer::Stack(source_offset + offset),
                    Space::Const => Pointer::Const(source_offset + offset),
                    Space::Stack => Pointer::Stack(source_offset + offset),
                    Space::Absolute => Pointer::Absolute(source_offset + offset),
                };
                statements.push(Statement::Ld {
                    source: Source::Pointer { base, offset: None },
                    destination: Destination::Pointer { base: Pointer::Stack(target_offset + offset),
                                                        offset: None } });
            }
        }
        Expression::Array(value) => match layout {
            Layout::Array { inner, len } => {
                assert_eq!(*len as usize, value.inner.len());

                let array_type_size = inner.size();

                for (i, expr) in value.inner.iter().enumerate() {
                    let stack_address = stack_address + array_type_size * (i as u16);
                    compile_expression_into_stack(expr,
                                                  inner,
                                                  symbol_alloc,
                                                  register_alloc,
                                                  fn_alloc,
                                                  stack_address,
                                                  0,
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

                        let base = match symbol.space {
                            Space::Stack => Pointer::Stack(symbol.offset),
                            Space::Static => Pointer::Static(symbol.offset),
                            Space::Const => Pointer::Const(symbol.offset),
                            Space::Absolute => Pointer::Absolute(symbol.offset),
                        };
                        statements.push(Statement::LdAddr {
                            source: Source::Pointer { base, offset: None },
                            destination: Destination::Pointer { base: Pointer::Stack(stack_address),
                                                                offset: None } });
                    }
                    Expression::Index(index) => match &index.inner.right {
                        Expression::Path(path) => {
                            let name = path_to_symbol_name(path);
                            let symbol = symbol_alloc.get(&name);

                            if let Layout::Array { inner, len } = &symbol.layout {
                                assert_eq!(ptr, inner);

                                // TODO extend to non-constant expression indices
                                let type_size = inner.size();
                                let offset_const_expr = compute_const_expression(&index.inner.left);
                                let offset = symbol.offset + type_size * offset_const_expr;

                                let base = match symbol.space {
                                    Space::Static => Pointer::Stack(offset),
                                    Space::Const => Pointer::Const(offset),
                                    Space::Stack => Pointer::Stack(offset),
                                    Space::Absolute => Pointer::Absolute(offset),
                                };
                                statements.push(Statement::LdAddr {
                                    source: Source::Pointer { base, offset: None },
                                    destination: Destination::Pointer { base: Pointer::Stack(stack_address),
                                                                        offset: None } });
                            } else {
                                panic!()
                            }
                        }
                        _ => unimplemented!(),
                    },
                    // TODO generalise (allow taking a pointer of something other than just a
                    // symbol)
                    _ => unimplemented!(),
                }
            }
            _ => panic!(),
        },
        Expression::Deref(_) => {}
        Expression::Not(_) => {}

        Expression::Add(add) => {
            let left = compile_expression_into_register(&add.inner.left,
                                                        layout,
                                                        symbol_alloc,
                                                        register_alloc,
                                                        fn_alloc,
                                                        stack_address,
                                                        statements);
            let right = compile_expression_into_register(&add.inner.right,
                                                         layout,
                                                         symbol_alloc,
                                                         register_alloc,
                                                         fn_alloc,
                                                         stack_address,
                                                         statements);
            statements.push(Statement::Add { left: Source::Register(left),
                                   right: Source::Register(right),
                                   destination:
                                       Destination::Pointer { base: Pointer::Stack(stack_address),
                                                              offset: None } });
            register_alloc.free(left);
            register_alloc.free(right);
        }

        Expression::Sub(sub) => {
            let left = compile_expression_into_register(&sub.inner.left,
                                                        layout,
                                                        symbol_alloc,
                                                        register_alloc,
                                                        fn_alloc,
                                                        stack_address,
                                                        statements);
            let right = compile_expression_into_register(&sub.inner.right,
                                                         layout,
                                                         symbol_alloc,
                                                         register_alloc,
                                                         fn_alloc,
                                                         stack_address,
                                                         statements);
            statements.push(Statement::Sub { left: Source::Register(left),
                                   right: Source::Register(right),
                                   destination:
                                       Destination::Pointer { base: Pointer::Stack(stack_address),
                                                              offset: None } });
            register_alloc.free(left);
            register_alloc.free(right);
        }

        Expression::And(and) => {
            let left = compile_expression_into_register(&and.inner.left,
                                                        layout,
                                                        symbol_alloc,
                                                        register_alloc,
                                                        fn_alloc,
                                                        stack_address,
                                                        statements);
            let right = compile_expression_into_register(&and.inner.right,
                                                         layout,
                                                         symbol_alloc,
                                                         register_alloc,
                                                         fn_alloc,
                                                         stack_address,
                                                         statements);
            statements.push(Statement::And { left: Source::Register(left),
                                   right: Source::Register(right),
                                   destination:
                                       Destination::Pointer { base: Pointer::Stack(stack_address),
                                                              offset: None } });
            register_alloc.free(left);
            register_alloc.free(right);
        }

        Expression::Or(or) => {
            let left = compile_expression_into_register(&or.inner.left,
                                                        layout,
                                                        symbol_alloc,
                                                        register_alloc,
                                                        fn_alloc,
                                                        stack_address,
                                                        statements);
            let right = compile_expression_into_register(&or.inner.right,
                                                         layout,
                                                         symbol_alloc,
                                                         register_alloc,
                                                         fn_alloc,
                                                         stack_address,
                                                         statements);
            statements.push(Statement::Or { left: Source::Register(left),
                                  right: Source::Register(right),
                                  destination:
                                      Destination::Pointer { base: Pointer::Stack(stack_address),
                                                             offset: None } });
            register_alloc.free(left);
            register_alloc.free(right);
        }

        Expression::Xor(xor) => {
            let left = compile_expression_into_register(&xor.inner.left,
                                                        layout,
                                                        symbol_alloc,
                                                        register_alloc,
                                                        fn_alloc,
                                                        stack_address,
                                                        statements);
            let right = compile_expression_into_register(&xor.inner.right,
                                                         layout,
                                                         symbol_alloc,
                                                         register_alloc,
                                                         fn_alloc,
                                                         stack_address,
                                                         statements);
            statements.push(Statement::Xor { left: Source::Register(left),
                                   right: Source::Register(right),
                                   destination:
                                       Destination::Pointer { base: Pointer::Stack(stack_address),
                                                              offset: None } });
            register_alloc.free(left);
            register_alloc.free(right);
        }

        Expression::Mul(mul) => {
            let left = compile_expression_into_register(&mul.inner.left,
                                                        layout,
                                                        symbol_alloc,
                                                        register_alloc,
                                                        fn_alloc,
                                                        stack_address,
                                                        statements);
            let right = compile_expression_into_register(&mul.inner.right,
                                                         layout,
                                                         symbol_alloc,
                                                         register_alloc,
                                                         fn_alloc,
                                                         stack_address,
                                                         statements);
            statements.push(Statement::Mul { left: Source::Register(left),
                                   right: Source::Register(right),
                                   destination:
                                       Destination::Pointer { base: Pointer::Stack(stack_address),
                                                              offset: None } });
            register_alloc.free(left);
            register_alloc.free(right);
        }

        Expression::Div(div) => {
            let left = compile_expression_into_register(&div.inner.left,
                                                        layout,
                                                        symbol_alloc,
                                                        register_alloc,
                                                        fn_alloc,
                                                        stack_address,
                                                        statements);
            let right = compile_expression_into_register(&div.inner.right,
                                                         layout,
                                                         symbol_alloc,
                                                         register_alloc,
                                                         fn_alloc,
                                                         stack_address,
                                                         statements);
            statements.push(Statement::Div { left: Source::Register(left),
                                   right: Source::Register(right),
                                   destination:
                                       Destination::Pointer { base: Pointer::Stack(stack_address),
                                                              offset: None } });
            register_alloc.free(left);
            register_alloc.free(right);
        }
        Expression::Assign(_)
        | Expression::PlusAssign(_)
        | Expression::MinusAssign(_)
        | Expression::MulAssign(_)
        | Expression::DivAssign(_)
        | Expression::AndAssign(_)
        | Expression::OrAssign(_)
        | Expression::XorAssign(_) => panic!(),

        // TODO reimplement (only works for [u8 N]
        Expression::Index(index) => {
            match &index.inner.right {
                Expression::Path(path) => {
                    let name = path_to_symbol_name(path);
                    let symbol = symbol_alloc.get(&name);

                    //assert_eq!(&Layout::Array {}, &symbol.layout);

                    // compute offset
                    let offset_register =
                        compile_expression_into_register(&index.inner.left,
                                                         &Layout::Pointer(Box::new(Layout::U8)),
                                                         &mut symbol_alloc.clone(),
                                                         register_alloc,
                                                         fn_alloc,
                                                         symbol_alloc.stack_address(),
                                                         statements);
                    let base = match symbol.space {
                        Space::Static => Pointer::Static(symbol.offset),
                        Space::Const => Pointer::Const(symbol.offset),
                        Space::Stack => Pointer::Stack(symbol.offset),
                        Space::Absolute => Pointer::Absolute(symbol.offset),
                    };
                    statements.push(Statement::Ld { source: Source::Pointer { base,
                                                                                    offset: Some(Box::new(Source::Register(offset_register))) },
                        destination: Destination::Pointer { base: Pointer::Stack(stack_address),
                                                            offset: None }
                    });

                    register_alloc.free(offset_register);
                }
                _ => unimplemented!(),
            }
        }
        Expression::LeftShift(_) => unimplemented!(),
        Expression::RightShift(_) => unimplemented!(),
        Expression::Eq(_) => unimplemented!(),
        Expression::NotEq(_) => unimplemented!(),
        Expression::LessEq(_) => unimplemented!(),
        Expression::GreaterEq(_) => unimplemented!(),
        Expression::Less(_) => unimplemented!(),
        Expression::Greater(_) => unimplemented!(),
        Expression::Call(call) => match &call.inner.left {
            Expression::Path(ident) => {
                let (fn_, routine) = fn_alloc.get(&path_to_symbol_name(&ident));

                // check that the function returns the type we're trying to compile!
                assert_eq!(fn_.ret_layout.as_ref(), Some(layout));

                let args_call = &call.inner.args;
                let args_layout = &fn_.arg_layout;

                let destination =
                    Some(Destination::Pointer { base: Pointer::Stack(stack_address),
                                                offset:
                                                    Some(Box::new(Source::Literal(offset_address))) });

                assert_eq!(args_call.len(), args_layout.len());

                let mut args = Vec::new();
                let mut offset = stack_address;

                // lay functions arguments in the stack
                let mut alloc = symbol_alloc.clone();
                for (call_arg, arg_layout) in args_call.iter().zip(args_layout) {
                    compile_expression_into_stack(call_arg,
                                                  &arg_layout,
                                                  &mut alloc,
                                                  register_alloc,
                                                  fn_alloc,
                                                  offset,
                                                  0,
                                                  statements);
                    args.push(offset);
                    offset += arg_layout.size();
                }

                statements.push(Statement::Call { routine,
                                                  args,
                                                  destination })
            }
            _ => panic!(),
        },
    }
}

// compile the evaluation of an expression, but discard the result (don't store
// it anywhere)
pub fn compile_expression_into_void<B: ByteOrder>(expression: &Expression,
                                                  register_alloc: &mut RegisterAlloc,
                                                  symbol_alloc: &mut SymbolAlloc<B>,
                                                  fn_alloc: &FnAlloc,
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
        E::PlusAssign(node) => arithmetic_assign_match_branch!(node, Add),
        E::MinusAssign(node) => arithmetic_assign_match_branch!(node, Sub),
        E::MulAssign(node) => arithmetic_assign_match_branch!(node, Mul),
        E::DivAssign(node) => arithmetic_assign_match_branch!(node, Div),
        E::AndAssign(node) => arithmetic_assign_match_branch!(node, And),
        E::OrAssign(node) => arithmetic_assign_match_branch!(node, Or),
        E::XorAssign(node) => arithmetic_assign_match_branch!(node, Xor),
        E::Assign(node) => {
            // TODO rework this workaround (using the arithmeric_assign function)
            let (_, source, destination) = arithmetic_assign(&node.inner.left,
                                                             &node.inner.right,
                                                             symbol_alloc,
                                                             register_alloc,
                                                             fn_alloc,
                                                             statements);
            statements.push(Ld { source,
                                 destination });
        }
        E::Call(node) => unimplemented!(),
        _ => unimplemented!(),
    }

    // function to generate the arithmetic+assign expression
    // foo += bar
    // foo -= bar
    // foo &= bar
    // and so on...
    fn arithmetic_assign<B: ByteOrder>(left: &Expression,
                                       right: &Expression,
                                       symbol_alloc: &mut SymbolAlloc<B>,
                                       register_alloc: &mut RegisterAlloc,
                                       fn_alloc: &FnAlloc,
                                       statements: &mut Vec<Statement>)
                                       -> (Source<u8>, Source<u8>, Destination) {
        use Expression as E;

        let register = compile_expression_into_register(right,
                                                        &Layout::U8,
                                                        &mut symbol_alloc.clone(),
                                                        register_alloc,
                                                        fn_alloc,
                                                        symbol_alloc.stack_address(),
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
                        let offset_register = compile_expression_into_register(&index.inner.left, &Layout::Pointer(Box::new(Layout::U8)), &mut symbol_alloc.clone(), register_alloc, fn_alloc, symbol_alloc.stack_address(), statements);
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

pub fn compute_literal_as_numeric(lit: &Lit) -> u16 {
    let num = lit.to_string();
    if num.starts_with("0x") {
        u16::from_str_radix(&num[2..], 16).expect("Not a hex number")
    } else if num.as_bytes()[0].is_ascii_digit() {
        num.parse().expect("Not a number")
    } else {
        panic!("Not a number literal")
    }
}

/// Compute the size of a given constant (numeric) expression.
/// Panics if the expression is not a constant expression nor numeric.
pub fn compute_const_expression(expr: &Expression) -> u16 {
    use Expression::*;

    match expr {
        Lit(e) => compute_literal_as_numeric(e),
        Minus(_e) => unimplemented!("TODO"),
        Not(e) => !compute_const_expression(&e.inner),
        Add(e) => {
            compute_const_expression(&e.inner.left) + compute_const_expression(&e.inner.right)
        }
        Sub(e) => {
            compute_const_expression(&e.inner.left) - compute_const_expression(&e.inner.right)
        }
        Mul(e) => {
            compute_const_expression(&e.inner.left) * compute_const_expression(&e.inner.right)
        }
        Div(e) => {
            compute_const_expression(&e.inner.left) / compute_const_expression(&e.inner.right)
        }
        And(e) => {
            compute_const_expression(&e.inner.left) & compute_const_expression(&e.inner.right)
        }
        Or(e) => compute_const_expression(&e.inner.left) | compute_const_expression(&e.inner.right),
        Xor(e) => {
            compute_const_expression(&e.inner.left) ^ compute_const_expression(&e.inner.right)
        }
        LeftShift(e) => {
            compute_const_expression(&e.inner.left) << compute_const_expression(&e.inner.right)
        }
        RightShift(e) => {
            compute_const_expression(&e.inner.left) >> compute_const_expression(&e.inner.right)
        }
        _ => panic!("Not a constant expression"),
    }
}
