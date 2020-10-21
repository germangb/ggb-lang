use crate::{
    ir::{
        alloc::{FnAlloc, RegisterAlloc, Space, SymbolAlloc},
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

pub fn compile_expression_into_register8(
    expression: &Expression,
    type_: &Type,
    symbols: &mut SymbolAlloc,
    registers: &mut RegisterAlloc,
    functions: &FnAlloc,
    stack_address: u16,
    statements: &mut Vec<Statement>,
) -> usize {
    match expression {
        Expression::Lit(lit) => {
            let lit = utils::compute_literal_as_numeric(lit);
            match type_ {
                Type::U8(_) => {
                    assert!(lit <= 0xff);
                    let register = registers.alloc();
                    statements.push(Statement::Ld {
                        source: Source::Literal(lit as u8),
                        destination: Destination::Register(register),
                    });
                    register
                }
                Type::I8(_) => unimplemented!("TODO i8"),
                _ => panic!(),
            }
        }
        call @ Expression::Call(_) => {
            compile_expression_into_stack(
                call,
                type_,
                symbols,
                registers,
                functions,
                stack_address,
                statements,
            );
            let register = registers.alloc();
            statements.push(Statement::Ld {
                source: Source::Pointer(Pointer::Stack(stack_address)),
                destination: Destination::Register(register),
            });
            register
        }
        #[rustfmt::skip]
        Expression::Add(add) => {
            let left = compile_expression_into_register8(&add.inner.left, type_, symbols, registers, functions, stack_address, statements);
            let right = compile_expression_into_register8(&add.inner.right, type_, symbols, registers, functions, stack_address, statements);
            let free = left.max(right);
            let store = left + right - free;
            registers.free(free);
            statements.push(Statement::Add {
                left: Source::Register(left),
                right: Source::Register(right),
                destination: Destination::Register(store),
            });
            store
        }
        #[rustfmt::skip]
        Expression::Sub(add) => {
            let left = compile_expression_into_register8(&add.inner.left, type_, symbols, registers, functions, stack_address, statements);
            let right = compile_expression_into_register8(&add.inner.right, type_, symbols, registers, functions, stack_address, statements);
            let free = left.max(right);
            let store = left + right - free;
            registers.free(free);
            statements.push(Statement::Sub {
                left: Source::Register(left),
                right: Source::Register(right),
                destination: Destination::Register(store),
            });
            store
        }
        #[rustfmt::skip]
        Expression::And(and) => {
            let left = compile_expression_into_register8(&and.inner.left, type_, symbols, registers, functions, stack_address, statements);
            let right = compile_expression_into_register8(&and.inner.right, type_, symbols, registers, functions, stack_address, statements);
            let free = left.max(right);
            let store = left + right - free;
            registers.free(free);
            statements.push(Statement::And {
                left: Source::Register(left),
                right: Source::Register(right),
                destination: Destination::Register(store),
            });
            store
        }
        #[rustfmt::skip]
        Expression::Or(or) => {
            let left = compile_expression_into_register8(&or.inner.left, type_, symbols, registers, functions, stack_address, statements);
            let right = compile_expression_into_register8(&or.inner.right, type_, symbols, registers, functions, stack_address, statements);
            let free = left.max(right);
            let store = left + right - free;
            registers.free(free);
            statements.push(Statement::Or {
                left: Source::Register(left),
                right: Source::Register(right),
                destination: Destination::Register(store),
            });
            store
        }
        #[rustfmt::skip]
        Expression::Xor(xor) => {
            let left = compile_expression_into_register8(&xor.inner.left, type_, symbols, registers, functions, stack_address, statements);
            let right = compile_expression_into_register8(&xor.inner.right, type_, symbols, registers, functions, stack_address, statements);
            let free = left.max(right);
            let store = left + right - free;
            registers.free(free);
            statements.push(Statement::Xor {
                left: Source::Register(left),
                right: Source::Register(right),
                destination: Destination::Register(store),
            });
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
pub fn compile_expression_into_stack(
    expression: &Expression,
    type_: &Type,
    alloc: &mut SymbolAlloc,
    register_alloc: &mut RegisterAlloc,
    fn_alloc: &FnAlloc,
    stack_address: u16,
    statements: &mut Vec<Statement>,
) {
    match expression {
        // compile literal expression by simply move a literal value unto the stack address.
        // the size must be either a u8 or a u16 at this point. Any other value is wrong and the
        // compiler frontend should've caught it by now, hence the panic.
        Expression::Lit(lit) => {
            let lit = utils::compute_literal_as_numeric(lit);
            match type_ {
                Type::U8(_) => {
                    assert!(lit <= 0xff);
                    statements.push(Statement::Ld {
                        source: Source::Literal(lit as u8),
                        destination: Destination::Pointer(Pointer::Stack(stack_address)),
                    })
                }
                Type::I8(_) => unimplemented!("TODO i8"),
                Type::Pointer(_) => statements.push(Statement::Ld16 {
                    source: Source::Literal(lit),
                    destination: Destination::Pointer(Pointer::Stack(stack_address)),
                }),
                _ => panic!(),
            }
        }
        Expression::Path(path) => {
            let name = utils::path_to_symbol_name(path);
            let symbol = alloc.symbol(&name);
            // fallibility should be implemented in the frontend. If it panics here, it has
            // to be a bug.
            assert!(utils::equivalent_types(type_, &symbol.type_));

            // byte by byte copy
            // TODO consider using a loop if the type is too large later on if
            //  code size gets too large.
            let source_offset = symbol.offset;
            let target_offset = stack_address;
            for offset in 0..utils::size_of(type_) {
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
        Expression::Array(value) => match type_ {
            Type::Array(array) => {
                let array_type_size = utils::size_of(&array.type_);
                let array_len = utils::compute_const_expression(&array.len);

                assert_eq!(
                    array_len as usize,
                    value.inner.len(),
                    "array value length doesn't match the length in the type annotation"
                );

                for (i, expr) in value.inner.iter().enumerate() {
                    let stack_address = stack_address + array_type_size * (i as u16);
                    compile_expression_into_stack(
                        expr,
                        &array.type_,
                        alloc,
                        register_alloc,
                        fn_alloc,
                        stack_address,
                        statements,
                    );
                }
            }
            _ => panic!(),
        },
        Expression::Minus(_) => {}
        Expression::AddressOf(address_of) => {
            match &address_of.inner {
                Expression::Path(path) => match type_ {
                    Type::Pointer(ptr) => {
                        let name = utils::path_to_symbol_name(&path);
                        let symbol = alloc.symbol(&name);
                        assert!(utils::equivalent_types(&ptr.type_, &symbol.type_));
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
                    _ => panic!(),
                },
                // TODO generalise (allow taking a pointer of something other than just a symbol)
                _ => unimplemented!(),
            }
        }
        Expression::Deref(_) => {}
        Expression::Not(_) => {}
        #[rustfmt::skip]
        Expression::Add(add) => {
            let left = compile_expression_into_register8(&add.inner.left, type_, alloc, register_alloc, fn_alloc, stack_address, statements);
            let right = compile_expression_into_register8(&add.inner.right, type_, alloc, register_alloc, fn_alloc, stack_address, statements);
            statements.push(Statement::Add {
                left: Source::Register(left),
                right: Source::Register(right),
                destination: Destination::Pointer(Pointer::Stack(stack_address)),
            });
            register_alloc.free(left);
            register_alloc.free(right);
        }
        #[rustfmt::skip]
        Expression::Sub(sub) => {
            let left = compile_expression_into_register8(&sub.inner.left, type_, alloc, register_alloc, fn_alloc, stack_address, statements);
            let right = compile_expression_into_register8(&sub.inner.right, type_, alloc, register_alloc, fn_alloc, stack_address, statements);
            statements.push(Statement::Sub {
                left: Source::Register(left),
                right: Source::Register(right),
                destination: Destination::Pointer(Pointer::Stack(stack_address)),
            });
            register_alloc.free(left);
            register_alloc.free(right);
        }
        #[rustfmt::skip]
        Expression::And(and) => {
            let left = compile_expression_into_register8(&and.inner.left, type_, alloc, register_alloc, fn_alloc, stack_address, statements);
            let right = compile_expression_into_register8(&and.inner.right, type_, alloc, register_alloc, fn_alloc, stack_address, statements);
            statements.push(Statement::And {
                left: Source::Register(left),
                right: Source::Register(right),
                destination: Destination::Pointer(Pointer::Stack(stack_address)),
            });
            register_alloc.free(left);
            register_alloc.free(right);
        }
        #[rustfmt::skip]
        Expression::Or(or) => {
            let left = compile_expression_into_register8(&or.inner.left, type_, alloc, register_alloc, fn_alloc, stack_address, statements);
            let right = compile_expression_into_register8(&or.inner.right, type_, alloc, register_alloc, fn_alloc, stack_address, statements);
            statements.push(Statement::Or {
                left: Source::Register(left),
                right: Source::Register(right),
                destination: Destination::Pointer(Pointer::Stack(stack_address)),
            });
            register_alloc.free(left);
            register_alloc.free(right);
        }
        #[rustfmt::skip]
        Expression::Xor(xor) => {
            let left = compile_expression_into_register8(&xor.inner.left, type_, alloc, register_alloc, fn_alloc, stack_address, statements);
            let right = compile_expression_into_register8(&xor.inner.right, type_, alloc, register_alloc, fn_alloc, stack_address, statements);
            statements.push(Statement::Xor {
                left: Source::Register(left),
                right: Source::Register(right),
                destination: Destination::Pointer(Pointer::Stack(stack_address)),
            });
            register_alloc.free(left);
            register_alloc.free(right);
        }
        #[rustfmt::skip]
        Expression::Mul(mul) => {
            let left = compile_expression_into_register8(&mul.inner.left, type_, alloc, register_alloc, fn_alloc, stack_address, statements);
            let right = compile_expression_into_register8(&mul.inner.right, type_, alloc, register_alloc, fn_alloc, stack_address, statements);
            statements.push(Statement::Mul {
                left: Source::Register(left),
                right: Source::Register(right),
                destination: Destination::Pointer(Pointer::Stack(stack_address)),
            });
            register_alloc.free(left);
            register_alloc.free(right);
        }
        #[rustfmt::skip]
        Expression::Div(div) => {
            let left = compile_expression_into_register8(&div.inner.left, type_, alloc, register_alloc, fn_alloc, stack_address, statements);
            let right = compile_expression_into_register8(&div.inner.right, type_, alloc, register_alloc, fn_alloc, stack_address, statements);
            statements.push(Statement::Div {
                left: Source::Register(left),
                right: Source::Register(right),
                destination: Destination::Pointer(Pointer::Stack(stack_address)),
            });
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
                let fn_ = fn_alloc.get(&utils::path_to_symbol_name(&ident));

                // check that the function returns the type we're trying to compile!
                assert!(utils::equivalent_types(
                    &fn_.fn_return.as_ref().unwrap().type_,
                    type_
                ));

                let call_args = &call.inner.args;
                let destination = Some(Destination::Pointer(Pointer::Stack(stack_address)));

                if let Some(sign_args) = &fn_.fn_arg {
                    let sign_args = &sign_args.inner;

                    assert_eq!(call_args.len(), sign_args.len());

                    let mut args = Vec::with_capacity(sign_args.len());
                    let mut offset = stack_address;

                    // lay functions arguments in the stack
                    let mut alloc = alloc.clone();
                    for (call_arg, sign_arg) in call_args.iter().zip(sign_args) {
                        compile_expression_into_stack(
                            call_arg,
                            &sign_arg.type_,
                            &mut alloc,
                            register_alloc,
                            fn_alloc,
                            offset,
                            statements,
                        );
                        let type_size = utils::size_of(&sign_arg.type_);
                        args.push(offset);
                        offset += type_size;
                    }

                    statements.push(Statement::Call {
                        routine: 0,
                        args,
                        destination,
                    })
                } else {
                    assert!(call.inner.args.is_empty());
                    statements.push(Statement::Call {
                        routine: 0,
                        args: Vec::new(),
                        destination,
                    })
                }
            }
            _ => panic!(),
        },
    }
}
