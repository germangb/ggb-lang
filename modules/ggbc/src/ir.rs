//! Definition and compilation of IR.
use crate::{
    ir::{
        alloc::{Alloc, FnAlloc, Space},
        alloc_register::RegisterAlloc,
    },
    parser::{
        ast,
        ast::{Const, Expression, Fn, Let, Scope, Static, Type},
        Ast,
    },
};

mod alloc;
mod alloc_register;
mod utils;

pub type Address = u16;
pub type Register = usize;

/// Intermediate representation of a program.
pub struct Ir {
    /// Const memory space.
    pub const_: Vec<u8>,
    /// Compiled program routines.
    pub routines: Vec<Routine>,
    /// Interrupt handler routines.
    pub interrupts: Interrupts,
    /// Program entry point index.
    pub main: usize,
}

/// Routine handles for each type of interrupt.
#[derive(Default)]
pub struct Interrupts {
    /// VBlank interrupt.
    pub vblank: Option<usize>,
    /// LCD Status interrupt.
    pub lcd_stat: Option<usize>,
    /// Timer interrupt.
    pub timer: Option<usize>,
    /// Serial interrupt handler.
    pub serial: Option<usize>,
    /// Joypad interrupt.
    pub joypad: Option<usize>,
}

/// Data associated with a compiled IR routine.
pub struct Routine {
    /// Instructions of the routine.
    pub block: Vec<Statement>,
}

/// Virtual memory pointers.
#[derive(Debug)]
pub enum Pointer {
    Absolute(Address),
    Static(Address),
    Const(Address),
    Stack(Address),
}

/// Source from where to pull a value.
#[derive(Debug)]
pub enum Source<T> {
    /// Data at the given address.
    At(Pointer),
    /// Data at the given register.
    Register(Register),
    /// Literal data.
    Literal(T),
}

/// Destination where to store a value.
#[derive(Debug)]
pub enum Destination {
    /// Store at the given address
    Into(Pointer),
    /// Store at the given register.
    Register(Register),
}

/// Jump location of `Jmp` and `Cmp` statements.
#[derive(Debug)]
pub enum Jump {
    /// Jump relative to the current program pointer.
    Relative(i8),
}

/// The statements, or instruction set of the IR.
#[rustfmt::skip]
#[derive(Debug)]
pub enum Statement {
    // move data
    Ld     { source: Source<u8>,  destination: Destination },
    Ld16   { source: Source<u16>, destination: Destination },
    // move pointer address
    LdAddr { source: Source<u16>, destination: Destination },

    // arithmetic (unary)
    Inc   { destination: Destination },
    Inc16 { destination: Destination },
    Dec   { destination: Destination },
    Dec16 { destination: Destination },

    // arithmetic (binary)
    Add { left: Source<u8>, right: Source<u8>, destination: Destination },
    Sub { left: Source<u8>, right: Source<u8>, destination: Destination },
    And { left: Source<u8>, right: Source<u8>, destination: Destination },
    Xor { left: Source<u8>, right: Source<u8>, destination: Destination },
    Or  { left: Source<u8>, right: Source<u8>, destination: Destination },

    // flow control
    Jmp { target: Jump },
    Cmp { target: Jump, source: Source<u8> },

    // routines
    Call { routine: usize, args: Vec<Address>, destination: Option<Destination> },
    Ret,
}

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

// TODO
pub fn compile(ast: &Ast) -> Ir {
    use ast::Statement::*;

    let mut alloc = Alloc::default();
    let mut routines = Vec::new();
    let mut statements = Vec::new();
    let mut fn_alloc = FnAlloc::default();

    for statement in &ast.inner {
        match statement {
            Static(static_) => compile_static(static_, &mut alloc),
            Const(const_) => compile_const(const_, &mut alloc),
            Let(let_) => compile_let(let_, &mut alloc, &fn_alloc, &mut statements),
            Fn(fn_) => compile_fn(fn_, &mut alloc, &mut fn_alloc),
            _ => {}
        }
    }

    for s in statements.iter() {
        println!("{:x?}", s);
    }
    println!("---");
    println!("{:?}", alloc);
    Ir {
        const_: Vec::new(),
        routines,
        interrupts: Interrupts::default(),
        main: 0,
    }
}

// compile "static" statement
fn compile_static<'a>(static_: &'a Static<'a>, alloc: &mut Alloc<'a>) {
    if let Some(offset) = &static_.offset {
        let offset = utils::compute_const_expression(&offset.expression);
        alloc.alloc_absolute(&static_.field, offset);
    } else {
        alloc.alloc_static(&static_.field);
    }
}

// compile "const" statement
fn compile_const<'a>(const_: &'a Const<'a>, alloc: &mut Alloc<'a>) {
    alloc.alloc_const(&const_.field, &const_.expr);
}

// compile "let" statement
fn compile_let<'a>(
    let_: &'a Let<'a>,
    alloc: &mut Alloc<'a>,
    fn_alloc: &FnAlloc,
    statements: &mut Vec<Statement>,
) {
    // allocate memory on the stack for this field
    // the compiled expression should store the result on the stack
    let stack_address = alloc.alloc_stack_field(&let_.field);
    let mut register_alloc = RegisterAlloc::default();
    compile_expression_into_stack(
        &let_.expr,
        &let_.field.type_,
        alloc,
        &mut register_alloc,
        fn_alloc,
        stack_address,
        statements,
    );

    assert_eq!(0, register_alloc.len())
}

fn compile_expression_into_register8(
    expression: &Expression,
    type_: &Type,
    alloc: &mut Alloc,
    register_alloc: &mut RegisterAlloc,
    fn_alloc: &FnAlloc,
    statements: &mut Vec<Statement>,
) -> usize {
    match expression {
        Expression::Lit(lit) => {
            let lit = utils::compute_literal_as_numeric(lit);
            match type_ {
                Type::U8(_) => {
                    assert!(lit <= 0xff);
                    let register = register_alloc.alloc();
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
        #[rustfmt::skip]
        Expression::Add(add) => {
            let left = compile_expression_into_register8(&add.inner.left, type_, alloc, register_alloc, fn_alloc, statements);
            let right = compile_expression_into_register8(&add.inner.right, type_, alloc, register_alloc, fn_alloc, statements);
            let free = left.max(right);
            let store = left + right - free;
            register_alloc.free(free);
            statements.push(Statement::Add {
                left: Source::Register(left),
                right: Source::Register(right),
                destination: Destination::Register(store),
            });
            store
        }
        #[rustfmt::skip]
        Expression::Sub(add) => {
            let left = compile_expression_into_register8(&add.inner.left, type_, alloc, register_alloc, fn_alloc, statements);
            let right = compile_expression_into_register8(&add.inner.right, type_, alloc, register_alloc, fn_alloc, statements);
            let free = left.max(right);
            let store = left + right - free;
            register_alloc.free(free);
            statements.push(Statement::Sub {
                left: Source::Register(left),
                right: Source::Register(right),
                destination: Destination::Register(store),
            });
            store
        }
        #[rustfmt::skip]
        Expression::And(and) => {
            let left = compile_expression_into_register8(&and.inner.left, type_, alloc, register_alloc, fn_alloc, statements);
            let right = compile_expression_into_register8(&and.inner.right, type_, alloc, register_alloc, fn_alloc, statements);
            let free = left.max(right);
            let store = left + right - free;
            register_alloc.free(free);
            statements.push(Statement::And {
                left: Source::Register(left),
                right: Source::Register(right),
                destination: Destination::Register(store),
            });
            store
        }
        #[rustfmt::skip]
        Expression::Or(or) => {
            let left = compile_expression_into_register8(&or.inner.left, type_, alloc, register_alloc, fn_alloc, statements);
            let right = compile_expression_into_register8(&or.inner.right, type_, alloc, register_alloc, fn_alloc, statements);
            let free = left.max(right);
            let store = left + right - free;
            register_alloc.free(free);
            statements.push(Statement::Or {
                left: Source::Register(left),
                right: Source::Register(right),
                destination: Destination::Register(store),
            });
            store
        }
        #[rustfmt::skip]
        Expression::Xor(xor) => {
            let left = compile_expression_into_register8(&xor.inner.left, type_, alloc, register_alloc, fn_alloc, statements);
            let right = compile_expression_into_register8(&xor.inner.right, type_, alloc, register_alloc, fn_alloc, statements);
            let free = left.max(right);
            let store = left + right - free;
            register_alloc.free(free);
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
fn compile_expression_into_stack(
    expression: &Expression,
    type_: &Type,
    alloc: &mut Alloc,
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
                        destination: Destination::Into(Pointer::Stack(stack_address)),
                    })
                }
                Type::I8(_) => unimplemented!("TODO i8"),
                Type::Pointer(_) => statements.push(Statement::Ld16 {
                    source: Source::Literal(lit),
                    destination: Destination::Into(Pointer::Stack(stack_address)),
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
            let mut source_offset = symbol.offset;
            let mut target_offset = stack_address;
            for offset in 0..utils::size_of(type_) {
                let source = match symbol.space {
                    Space::Static => Source::At(Pointer::Stack(source_offset + offset)),
                    Space::Const => Source::At(Pointer::Const(source_offset + offset)),
                    Space::Stack => Source::At(Pointer::Stack(source_offset + offset)),
                    Space::Absolute => Source::At(Pointer::Absolute(source_offset + offset)),
                };
                statements.push(Statement::Ld {
                    source,
                    destination: Destination::Into(Pointer::Stack(target_offset + offset)),
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
                            Space::Stack => Source::At(Pointer::Stack(symbol.offset)),
                            Space::Static => Source::At(Pointer::Static(symbol.offset)),
                            Space::Const => Source::At(Pointer::Const(symbol.offset)),
                            Space::Absolute => Source::At(Pointer::Absolute(symbol.offset)),
                        };
                        statements.push(Statement::LdAddr {
                            destination: Destination::Into(Pointer::Stack(stack_address)),
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
            let left = compile_expression_into_register8(&add.inner.left, type_, alloc, register_alloc, fn_alloc, statements);
            let right = compile_expression_into_register8(&add.inner.right, type_, alloc, register_alloc, fn_alloc, statements);
            statements.push(Statement::Add {
                left: Source::Register(left),
                right: Source::Register(right),
                destination: Destination::Into(Pointer::Stack(stack_address)),
            });
            register_alloc.free(left);
            register_alloc.free(right);
        }
        #[rustfmt::skip]
        Expression::Sub(sub) => {
            let left = compile_expression_into_register8(&sub.inner.left, type_, alloc, register_alloc, fn_alloc, statements);
            let right = compile_expression_into_register8(&sub.inner.right, type_, alloc, register_alloc, fn_alloc, statements);
            statements.push(Statement::Sub {
                left: Source::Register(left),
                right: Source::Register(right),
                destination: Destination::Into(Pointer::Stack(stack_address)),
            });
            register_alloc.free(left);
            register_alloc.free(right);
        }
        #[rustfmt::skip]
        Expression::And(and) => {
            let left = compile_expression_into_register8(&and.inner.left, type_, alloc, register_alloc, fn_alloc, statements);
            let right = compile_expression_into_register8(&and.inner.right, type_, alloc, register_alloc, fn_alloc, statements);
            statements.push(Statement::And {
                left: Source::Register(left),
                right: Source::Register(right),
                destination: Destination::Into(Pointer::Stack(stack_address)),
            });
            register_alloc.free(left);
            register_alloc.free(right);
        }
        #[rustfmt::skip]
        Expression::Or(or) => {
            let left = compile_expression_into_register8(&or.inner.left, type_, alloc, register_alloc, fn_alloc, statements);
            let right = compile_expression_into_register8(&or.inner.right, type_, alloc, register_alloc, fn_alloc, statements);
            statements.push(Statement::Or {
                left: Source::Register(left),
                right: Source::Register(right),
                destination: Destination::Into(Pointer::Stack(stack_address)),
            });
            register_alloc.free(left);
            register_alloc.free(right);
        }
        #[rustfmt::skip]
        Expression::Xor(xor) => {
            let left = compile_expression_into_register8(&xor.inner.left, type_, alloc, register_alloc, fn_alloc, statements);
            let right = compile_expression_into_register8(&xor.inner.right, type_, alloc, register_alloc, fn_alloc, statements);
            statements.push(Statement::Xor {
                left: Source::Register(left),
                right: Source::Register(right),
                destination: Destination::Into(Pointer::Stack(stack_address)),
            });
            register_alloc.free(left);
            register_alloc.free(right);
        }
        Expression::Mul(_) => {}
        Expression::Div(_) => {}
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
                let destination = Some(Destination::Into(Pointer::Stack(stack_address)));

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

fn compile_fn<'a>(fn_: &'a Fn, alloc: &mut Alloc, fn_alloc: &mut FnAlloc<'a>) {
    fn_alloc.alloc(fn_);
}
