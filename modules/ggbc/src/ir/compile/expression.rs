use crate::{
    ir::{
        alloc::{FnAlloc, RegisterAlloc, Space, SymbolAlloc},
        layout::Layout,
        utils, Destination, Pointer, Source, Statement,
    },
    parser::ast::{Expression, Type},
};

// TODO refactor
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

pub fn compile_expression_into_register8(expression: &Expression,
                                         layout: &Layout,
                                         symbols: &mut SymbolAlloc,
                                         registers: &mut RegisterAlloc,
                                         functions: &FnAlloc,
                                         stack_address: u16,
                                         statements: &mut Vec<Statement>)
                                         -> usize {
    match expression {
        Expression::Lit(lit) => {
            let lit = utils::compute_literal_as_numeric(lit);
            match layout {
                Layout::U8 => {
                    assert!(lit <= 0xff);
                    let register = registers.alloc();
                    statements.push(Statement::Ld { source: Source::Literal(lit as u8),
                                                    destination:
                                                        Destination::Register(register) });
                    register
                }
                Layout::I8 => {
                    assert!(lit <= i8::max_value() as _);
                    assert!(lit >= i8::min_value() as _);
                    unimplemented!("TODO i8")
                }
                _ => panic!(),
            }
        }
        call @ Expression::Call(_) => {
            compile_expression_into_stack(call,
                                          layout,
                                          symbols,
                                          registers,
                                          functions,
                                          stack_address,
                                          statements);
            let register = registers.alloc();
            statements.push(Statement::Ld { source:
                                                Source::Pointer(Pointer::Stack(stack_address)),
                                            destination: Destination::Register(register) });
            register
        }

        Expression::Add(add) => {
            let left = compile_expression_into_register8(&add.inner.left,
                                                         layout,
                                                         symbols,
                                                         registers,
                                                         functions,
                                                         stack_address,
                                                         statements);
            let right = compile_expression_into_register8(&add.inner.right,
                                                          layout,
                                                          symbols,
                                                          registers,
                                                          functions,
                                                          stack_address,
                                                          statements);
            let free = left.max(right);
            let store = left + right - free;
            registers.free(free);
            statements.push(Statement::Add { left: Source::Register(left),
                                             right: Source::Register(right),
                                             destination: Destination::Register(store) });
            store
        }

        Expression::Sub(add) => {
            let left = compile_expression_into_register8(&add.inner.left,
                                                         layout,
                                                         symbols,
                                                         registers,
                                                         functions,
                                                         stack_address,
                                                         statements);
            let right = compile_expression_into_register8(&add.inner.right,
                                                          layout,
                                                          symbols,
                                                          registers,
                                                          functions,
                                                          stack_address,
                                                          statements);
            let free = left.max(right);
            let store = left + right - free;
            registers.free(free);
            statements.push(Statement::Sub { left: Source::Register(left),
                                             right: Source::Register(right),
                                             destination: Destination::Register(store) });
            store
        }

        Expression::And(and) => {
            let left = compile_expression_into_register8(&and.inner.left,
                                                         layout,
                                                         symbols,
                                                         registers,
                                                         functions,
                                                         stack_address,
                                                         statements);
            let right = compile_expression_into_register8(&and.inner.right,
                                                          layout,
                                                          symbols,
                                                          registers,
                                                          functions,
                                                          stack_address,
                                                          statements);
            let free = left.max(right);
            let store = left + right - free;
            registers.free(free);
            statements.push(Statement::And { left: Source::Register(left),
                                             right: Source::Register(right),
                                             destination: Destination::Register(store) });
            store
        }

        Expression::Or(or) => {
            let left = compile_expression_into_register8(&or.inner.left,
                                                         layout,
                                                         symbols,
                                                         registers,
                                                         functions,
                                                         stack_address,
                                                         statements);
            let right = compile_expression_into_register8(&or.inner.right,
                                                          layout,
                                                          symbols,
                                                          registers,
                                                          functions,
                                                          stack_address,
                                                          statements);
            let free = left.max(right);
            let store = left + right - free;
            registers.free(free);
            statements.push(Statement::Or { left: Source::Register(left),
                                            right: Source::Register(right),
                                            destination: Destination::Register(store) });
            store
        }

        Expression::Xor(xor) => {
            let left = compile_expression_into_register8(&xor.inner.left,
                                                         layout,
                                                         symbols,
                                                         registers,
                                                         functions,
                                                         stack_address,
                                                         statements);
            let right = compile_expression_into_register8(&xor.inner.right,
                                                          layout,
                                                          symbols,
                                                          registers,
                                                          functions,
                                                          stack_address,
                                                          statements);
            let free = left.max(right);
            let store = left + right - free;
            registers.free(free);
            statements.push(Statement::Xor { left: Source::Register(left),
                                             right: Source::Register(right),
                                             destination: Destination::Register(store) });
            store
        }
        _ => unimplemented!(),
    }
}

// compile computation of expression represented by `expression` and store the
// result in the given stack address. Assume that the expression fits the
// allocated space in `stack`, and the size of the type is exactly `size` bytes.
//
// The memory space in the stack was allocated with the `alloc` allocator. In
// order to perform any intermediate expression compilations that require usage
// of the stack (such as a function call), you must crate a new allocator by
// cloning it, perform the compilation, then drop it, so the original stack
// pointer is preserved.
pub fn compile_expression_into_stack(expression: &Expression,
                                     layout: &Layout,
                                     symbol_alloc: &mut SymbolAlloc,
                                     register_alloc: &mut RegisterAlloc,
                                     fn_alloc: &FnAlloc,
                                     stack_address: u16,
                                     statements: &mut Vec<Statement>) {
    match expression {
        // compile literal expression by simply move a literal value unto the stack address.
        // the size must be either a u8 or a u16 at this point. Any other value is wrong and the
        // compiler frontend should've caught it by now, hence the panic.
        Expression::Lit(lit) => {
            let lit = utils::compute_literal_as_numeric(lit);
            match layout {
                Layout::U8 => {
                    assert!(lit <= 0xff);
                    statements.push(Statement::Ld {
                        source: Source::Literal(lit as u8),
                        destination: Destination::Pointer(Pointer::Stack(stack_address)),
                    })
                }
                Layout::I8 => unimplemented!("TODO i8"),
                Layout::Pointer(_) => statements.push(Statement::Ld16 {
                    source: Source::Literal(lit),
                    destination: Destination::Pointer(Pointer::Stack(stack_address)),
                }),
                _ => panic!(),
            }
        }
        Expression::Path(path) => {
            let name = utils::path_to_symbol_name(path);
            let symbol = symbol_alloc.get(&name);
            // fallibility should be implemented in the frontend. If it panics here, it has
            // to be a bug.
            assert_eq!(layout, &symbol.layout);

            // byte by byte copy
            // TODO consider using a loop if the type is too large later on if
            //  code size gets too large.
            let source_offset = symbol.offset;
            let target_offset = stack_address;
            for offset in 0..layout.compute_size() {
                let source = match symbol.space {
                    Space::Static => Source::Pointer(Pointer::Stack(source_offset + offset)),
                    Space::Const => Source::Pointer(Pointer::Const(source_offset + offset)),
                    Space::Stack => Source::Pointer(Pointer::Stack(source_offset + offset)),
                    Space::Absolute => Source::Pointer(Pointer::Absolute(source_offset + offset)),
                };
                statements.push(Statement::Ld {
                    source,
                    destination: Destination::Pointer(Pointer::Stack(target_offset + offset)),
                });
            }
        }
        Expression::Array(value) => match layout {
            Layout::Array { inner, len } => {
                assert_eq!(*len as usize, value.inner.len());

                let array_type_size = inner.compute_size();

                for (i, expr) in value.inner.iter().enumerate() {
                    let stack_address = stack_address + array_type_size * (i as u16);
                    compile_expression_into_stack(expr,
                                                  inner,
                                                  symbol_alloc,
                                                  register_alloc,
                                                  fn_alloc,
                                                  stack_address,
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
                        let name = utils::path_to_symbol_name(&path);
                        let symbol = symbol_alloc.get(&name);

                        // check layouts
                        assert_eq!(ptr.as_ref(), &symbol.layout);

                        let source = match symbol.space {
                            Space::Stack => Source::Pointer(Pointer::Stack(symbol.offset)),
                            Space::Static => Source::Pointer(Pointer::Static(symbol.offset)),
                            Space::Const => Source::Pointer(Pointer::Const(symbol.offset)),
                            Space::Absolute => Source::Pointer(Pointer::Absolute(symbol.offset)),
                        };
                        statements.push(Statement::LdAddr {
                            destination: Destination::Pointer(Pointer::Stack(stack_address)),
                            source,
                        });
                    }
                    Expression::Index(index) => match &index.inner.right {
                        Expression::Path(path) => {
                            let name = utils::path_to_symbol_name(path);
                            let symbol = symbol_alloc.get(&name);

                            if let Layout::Array { inner, len } = &symbol.layout {
                                assert_eq!(ptr, inner);

                                // TODO extend to non-constant expression indices
                                let type_size = inner.compute_size();
                                let offset_const_expr =
                                    utils::compute_const_expression(&index.inner.left);
                                let offset = symbol.offset + type_size * offset_const_expr;

                                let source = match symbol.space {
                                    Space::Static => Source::Pointer(Pointer::Stack(offset)),
                                    Space::Const => Source::Pointer(Pointer::Const(offset)),
                                    Space::Stack => Source::Pointer(Pointer::Stack(offset)),
                                    Space::Absolute => Source::Pointer(Pointer::Absolute(offset)),
                                };
                                statements.push(Statement::LdAddr {
                                    source,
                                    destination: Destination::Pointer(Pointer::Stack(
                                        stack_address,
                                    )),
                                })
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
            let left = compile_expression_into_register8(&add.inner.left,
                                                         layout,
                                                         symbol_alloc,
                                                         register_alloc,
                                                         fn_alloc,
                                                         stack_address,
                                                         statements);
            let right = compile_expression_into_register8(&add.inner.right,
                                                          layout,
                                                          symbol_alloc,
                                                          register_alloc,
                                                          fn_alloc,
                                                          stack_address,
                                                          statements);
            statements.push(Statement::Add { left: Source::Register(left),
                                   right: Source::Register(right),
                                   destination:
                                       Destination::Pointer(Pointer::Stack(stack_address)) });
            register_alloc.free(left);
            register_alloc.free(right);
        }

        Expression::Sub(sub) => {
            let left = compile_expression_into_register8(&sub.inner.left,
                                                         layout,
                                                         symbol_alloc,
                                                         register_alloc,
                                                         fn_alloc,
                                                         stack_address,
                                                         statements);
            let right = compile_expression_into_register8(&sub.inner.right,
                                                          layout,
                                                          symbol_alloc,
                                                          register_alloc,
                                                          fn_alloc,
                                                          stack_address,
                                                          statements);
            statements.push(Statement::Sub { left: Source::Register(left),
                                   right: Source::Register(right),
                                   destination:
                                       Destination::Pointer(Pointer::Stack(stack_address)) });
            register_alloc.free(left);
            register_alloc.free(right);
        }

        Expression::And(and) => {
            let left = compile_expression_into_register8(&and.inner.left,
                                                         layout,
                                                         symbol_alloc,
                                                         register_alloc,
                                                         fn_alloc,
                                                         stack_address,
                                                         statements);
            let right = compile_expression_into_register8(&and.inner.right,
                                                          layout,
                                                          symbol_alloc,
                                                          register_alloc,
                                                          fn_alloc,
                                                          stack_address,
                                                          statements);
            statements.push(Statement::And { left: Source::Register(left),
                                   right: Source::Register(right),
                                   destination:
                                       Destination::Pointer(Pointer::Stack(stack_address)) });
            register_alloc.free(left);
            register_alloc.free(right);
        }

        Expression::Or(or) => {
            let left = compile_expression_into_register8(&or.inner.left,
                                                         layout,
                                                         symbol_alloc,
                                                         register_alloc,
                                                         fn_alloc,
                                                         stack_address,
                                                         statements);
            let right = compile_expression_into_register8(&or.inner.right,
                                                          layout,
                                                          symbol_alloc,
                                                          register_alloc,
                                                          fn_alloc,
                                                          stack_address,
                                                          statements);
            statements.push(Statement::Or { left: Source::Register(left),
                                  right: Source::Register(right),
                                  destination:
                                      Destination::Pointer(Pointer::Stack(stack_address)) });
            register_alloc.free(left);
            register_alloc.free(right);
        }

        Expression::Xor(xor) => {
            let left = compile_expression_into_register8(&xor.inner.left,
                                                         layout,
                                                         symbol_alloc,
                                                         register_alloc,
                                                         fn_alloc,
                                                         stack_address,
                                                         statements);
            let right = compile_expression_into_register8(&xor.inner.right,
                                                          layout,
                                                          symbol_alloc,
                                                          register_alloc,
                                                          fn_alloc,
                                                          stack_address,
                                                          statements);
            statements.push(Statement::Xor { left: Source::Register(left),
                                   right: Source::Register(right),
                                   destination:
                                       Destination::Pointer(Pointer::Stack(stack_address)) });
            register_alloc.free(left);
            register_alloc.free(right);
        }

        Expression::Mul(mul) => {
            let left = compile_expression_into_register8(&mul.inner.left,
                                                         layout,
                                                         symbol_alloc,
                                                         register_alloc,
                                                         fn_alloc,
                                                         stack_address,
                                                         statements);
            let right = compile_expression_into_register8(&mul.inner.right,
                                                          layout,
                                                          symbol_alloc,
                                                          register_alloc,
                                                          fn_alloc,
                                                          stack_address,
                                                          statements);
            statements.push(Statement::Mul { left: Source::Register(left),
                                   right: Source::Register(right),
                                   destination:
                                       Destination::Pointer(Pointer::Stack(stack_address)) });
            register_alloc.free(left);
            register_alloc.free(right);
        }

        Expression::Div(div) => {
            let left = compile_expression_into_register8(&div.inner.left,
                                                         layout,
                                                         symbol_alloc,
                                                         register_alloc,
                                                         fn_alloc,
                                                         stack_address,
                                                         statements);
            let right = compile_expression_into_register8(&div.inner.right,
                                                          layout,
                                                          symbol_alloc,
                                                          register_alloc,
                                                          fn_alloc,
                                                          stack_address,
                                                          statements);
            statements.push(Statement::Div { left: Source::Register(left),
                                   right: Source::Register(right),
                                   destination:
                                       Destination::Pointer(Pointer::Stack(stack_address)) });
            register_alloc.free(left);
            register_alloc.free(right);
        }
        Expression::Assign(_) => {}
        Expression::PlusAssign(_) => {}
        Expression::MinusAssign(_) => {}
        Expression::MulAssign(_) => {}
        Expression::DivAssign(_) => {}
        Expression::AndAssign(_) => {}
        Expression::OrAssign(_) => {}
        Expression::XorAssign(_) => {}
        Expression::Index(_) => {}
        Expression::LeftShift(_) => {}
        Expression::RightShift(_) => {}
        Expression::Eq(_) => {}
        Expression::NotEq(_) => {}
        Expression::LessEq(_) => {}
        Expression::GreaterEq(_) => {}
        Expression::Less(_) => {}
        Expression::Greater(_) => {}
        Expression::Call(call) => match &call.inner.left {
            Expression::Path(ident) => {
                let (fn_, routine) = fn_alloc.get(&utils::path_to_symbol_name(&ident));

                // check that the function returns the type we're trying to compile!
                assert_eq!(fn_.ret_layout.as_ref(), Some(layout));

                let args_call = &call.inner.args;
                let args_layout = &fn_.arg_layout;

                let destination = Some(Destination::Pointer(Pointer::Stack(stack_address)));

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
                                                  statements);
                    args.push(offset);
                    offset += arg_layout.compute_size();
                }

                statements.push(Statement::Call { routine,
                                                  args,
                                                  destination })
            }
            _ => panic!(),
        },
    }
}
