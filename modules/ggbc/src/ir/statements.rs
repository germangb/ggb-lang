use crate::{
    byteorder::ByteOrder,
    ir::{
        alloc::{FnAlloc, RegisterAlloc, SymbolAlloc},
        expression,
        layout::Layout,
        Destination, Location, Pointer, Routine, Source, Statement, NOP_BREAK, NOP_CONTINUE,
    },
    parser::ast,
};

/// Compile vec of statements.
pub(super) fn compile_statements<B: ByteOrder>(ast_statements: &[ast::Statement<'_>],
                                               main: bool,
                                               register_alloc: &mut RegisterAlloc,
                                               symbol_alloc: &mut SymbolAlloc<B>,
                                               fn_alloc: &mut FnAlloc,
                                               statements: &mut Vec<Statement>,
                                               routines: &mut Vec<Routine>) {
    use ast::Statement::{
        Break, Const, Continue, Fn, For, If, IfElse, Inline, Let, Loop, Panic, Return, Scope,
        Static,
    };
    use Statement::Stop;

    for statement in ast_statements {
        match statement {
            Panic(_) => {
                // ignore everything after a panic
                statements.push(Stop);
                break;
            }
            Static(static_) if main => compile_static(static_, symbol_alloc),
            Const(const_) if main => compile_const(const_, symbol_alloc),
            Fn(fn_) if main => {
                compile_fn(fn_, main, register_alloc, symbol_alloc, fn_alloc, routines)
            }
            Let(let_) => compile_let(let_, register_alloc, symbol_alloc, fn_alloc, statements),
            Scope(scope) => compile_scope(scope,
                                          main,
                                          register_alloc,
                                          symbol_alloc.clone(),
                                          fn_alloc,
                                          statements,
                                          routines),
            Inline(inline) => {
                compile_inline(inline, register_alloc, symbol_alloc, fn_alloc, statements)
            }
            If(if_) => compile_if(if_,
                                  false,
                                  main,
                                  register_alloc,
                                  symbol_alloc,
                                  fn_alloc,
                                  statements,
                                  routines),
            IfElse(if_else) => compile_if_else(if_else,
                                               main,
                                               register_alloc,
                                               symbol_alloc,
                                               fn_alloc,
                                               statements,
                                               routines),
            Loop(loop_) => compile_loop(loop_,
                                        main,
                                        register_alloc,
                                        symbol_alloc,
                                        fn_alloc,
                                        statements,
                                        routines),
            For(for_) => compile_for(for_,
                                     main,
                                     register_alloc,
                                     symbol_alloc,
                                     fn_alloc,
                                     statements,
                                     routines),
            Break(_) => {
                // ignore everything after a break statement
                compile_break(statements);
                break;
            }
            Continue(_) => {
                // ignore everything after a continue statement
                compile_continue(statements);
                break;
            }
            #[allow(unused)]
            Return(return_) => {
                // TODO
                break;
            }
            _ => {}
        }
    }
}

/// Compile inline expression.
/// Compiles an expression which drops the result.
fn compile_inline<B: ByteOrder>(inline: &ast::Inline<'_>,
                                register_alloc: &mut RegisterAlloc,
                                symbol_alloc: &mut SymbolAlloc<B>,
                                fn_alloc: &FnAlloc,
                                statements: &mut Vec<Statement>) {
    // compile expression and drop the results.
    // the expression will be evaluated by the result is not stored anywhere.
    expression::compile_expr_void(&inline.inner,
                                  symbol_alloc,
                                  fn_alloc,
                                  register_alloc,
                                  statements)
}

/// Compile break statement
fn compile_break(statements: &mut Vec<Statement>) {
    use Statement::Nop;

    // in order to compile the Break statement, the compiler needs to know how many
    // instructions there are ahead of it. add placeholder Nop statement, which
    // should be replaced inside the compile_loop compile_for functions.
    statements.push(Nop(NOP_BREAK));
}

/// Compile continue statement
fn compile_continue(statements: &mut Vec<Statement>) {
    use Statement::Nop;

    // same deal as with the break statement.
    // use a different Nop to differentiate it.
    statements.push(Nop(NOP_CONTINUE));
}

/// Compile loop statement
fn compile_for<B: ByteOrder>(for_: &ast::For<'_>,
                             main: bool,
                             register_alloc: &mut RegisterAlloc,
                             symbol_alloc: &mut SymbolAlloc<B>,
                             fn_alloc: &mut FnAlloc,
                             statements: &mut Vec<Statement>,
                             routines: &mut Vec<Routine>) {
    use Statement::{Inc, JmpCmp, Ld, Nop, Sub};

    // for loop is computed as a special case of the generic loop
    // initializes the
    let mut symbol_alloc = symbol_alloc.clone();
    let mut for_statements = Vec::new();
    let stack_address = symbol_alloc.alloc_stack_field(&for_.field);

    // init for variable with the lhs side of the range
    // TODO non-U8 variables
    let init = expression::compile_expr(&for_.range.left,
                                        &symbol_alloc,
                                        fn_alloc,
                                        register_alloc,
                                        statements);
    expression::free_source_registers(&init, register_alloc);
    statements.push(Ld { source: init,
                         destination: Destination::Pointer { base:
                                                                 Pointer::Stack(stack_address),
                                                             offset: None } });

    // compute end index of the for loop with the rhs of the range
    // increment if it's an inclusive range
    let end = expression::compile_expr(&for_.range.right,
                                       &symbol_alloc,
                                       fn_alloc,
                                       register_alloc,
                                       statements);
    let end_register = register_alloc.alloc();
    statements.push(Statement::Ld { source: end.clone(),
                                    destination: Destination::Register(end_register) });
    expression::free_source_registers(&end, register_alloc);
    if for_.range.eq.is_some() {
        statements.push(Inc { source: Source::Register(end_register),
                              destination: Destination::Register(end_register) });
    }

    // begin compiling the inner for loop statements.
    // check if for loop variable has reached the limit.
    let cmp_register = register_alloc.alloc();
    let prefix = vec![Sub { left: Source::Register(end_register),
                            right: Source::Pointer { base: Pointer::Stack(stack_address),
                                                     offset: None },
                            destination: Destination::Register(cmp_register) },
                      // TODO optimize away, as this is equivalent to: if foo { break }
                      JmpCmp { location: Location::Relative(1),
                               source: Source::Register(cmp_register) },
                      Nop(NOP_BREAK),];
    register_alloc.free(cmp_register);
    // increment the for loop variable
    let suffix = vec![Inc { source: Source::Pointer { base: Pointer::Stack(stack_address),
                                                      offset: None },
                            destination: Destination::Pointer { base:
                                                                    Pointer::Stack(stack_address),
                                                                offset: None } }];
    compile_loop_statements(&for_.inner,
                            main,
                            register_alloc,
                            &mut symbol_alloc,
                            fn_alloc,
                            prefix,
                            suffix,
                            &mut for_statements,
                            routines);
    statements.extend(for_statements);

    // free register holding the last index of the for loop
    register_alloc.free(end_register);
}

/// compile loop statements
fn compile_loop<B: ByteOrder>(loop_: &ast::Loop<'_>,
                              main: bool,
                              register_alloc: &mut RegisterAlloc,
                              symbol_alloc: &mut SymbolAlloc<B>,
                              fn_alloc: &mut FnAlloc,
                              statements: &mut Vec<Statement>,
                              routines: &mut Vec<Routine>) {
    compile_loop_statements(&loop_.inner,
                            main,
                            register_alloc,
                            symbol_alloc,
                            fn_alloc,
                            Vec::new(),
                            Vec::new(),
                            statements,
                            routines);
}

/// Compile inner loop statement
#[warn(clippy::too_many_arguments)]
fn compile_loop_statements<B: ByteOrder>(loop_: &[ast::Statement<'_>],
                                         main: bool,
                                         register_alloc: &mut RegisterAlloc,
                                         symbol_alloc: &mut SymbolAlloc<B>,
                                         fn_alloc: &mut FnAlloc,
                                         // TODO here to inject for-loop code
                                         //    consider doing this differently...
                                         prefix_statements: Vec<Statement>,
                                         suffix_statements: Vec<Statement>,
                                         statements: &mut Vec<Statement>,
                                         routines: &mut Vec<Routine>) {
    use Location::Relative;
    use Statement::{Jmp, Nop};

    // compile statements inside the loop block
    // at the end, jump back to the first statement
    let mut loop_statements = prefix_statements;
    compile_statements(loop_,
                       main,
                       register_alloc,
                       &mut symbol_alloc.clone(),
                       fn_alloc,
                       &mut loop_statements,
                       routines);
    loop_statements.extend(suffix_statements);
    let loop_statements_signed = loop_statements.len() as isize;
    loop_statements.push(Jmp { location: Relative(-(loop_statements_signed + 1) as i8) });
    // replace Nop(1|2) statements (placeholders for break and continue) with the
    // corresponding Jmp statements.
    let statements_len = loop_statements.len();
    for (i, statement) in loop_statements.iter_mut().enumerate() {
        match statement {
            // break
            Nop(NOP_BREAK) => {
                let relative = statements_len - i - 1;
                *statement = Jmp { location: Relative(relative as _) };
            }
            // continue
            Nop(NOP_CONTINUE) => {
                let relative = i as isize + 1;
                *statement = Jmp { location: Relative(-relative as _) };
            }
            _ => {}
        }
    }
    statements.extend(loop_statements);
}

/// Compile if statement.
#[warn(clippy::too_many_arguments)]
fn compile_if<B: ByteOrder>(if_: &ast::If<'_>,
                            has_else: bool,
                            main: bool,
                            register_alloc: &mut RegisterAlloc,
                            symbol_alloc: &mut SymbolAlloc<B>,
                            fn_alloc: &mut FnAlloc,
                            statements: &mut Vec<Statement>,
                            routines: &mut Vec<Routine>) {
    use Statement::JmpCmpNot;

    // compile expression into an 8bit register
    let source = expression::compile_expr(&if_.expression,
                                          symbol_alloc,
                                          fn_alloc,
                                          register_alloc,
                                          statements);
    expression::free_source_registers(&source, register_alloc);

    // compile the block of statements inside the if block.
    // clone the symbol_alloc to free any symbols defined within the block.
    let mut if_statements = Vec::new();
    compile_statements(&if_.inner,
                       main,
                       register_alloc,
                       &mut symbol_alloc.clone(),
                       fn_alloc,
                       &mut if_statements,
                       routines);

    let jmp = if_statements.len() + if has_else { 1 } else { 0 };
    statements.push(JmpCmpNot { location: Location::Relative(jmp as _),
                                source });
    statements.extend(if_statements);
}

/// Compile if else statement.
fn compile_if_else<B: ByteOrder>(if_else: &ast::IfElse<'_>,
                                 main: bool,
                                 register_alloc: &mut RegisterAlloc,
                                 symbol_alloc: &mut SymbolAlloc<B>,
                                 fn_alloc: &mut FnAlloc,
                                 statements: &mut Vec<Statement>,
                                 routines: &mut Vec<Routine>) {
    use Statement::Jmp;

    // compile else block statements
    let mut else_statements = Vec::new();
    compile_statements(&if_else.else_.inner,
                       main,
                       register_alloc,
                       &mut symbol_alloc.clone(),
                       fn_alloc,
                       &mut else_statements,
                       routines);
    // compile if statement
    // append a jump statement at the end.
    let mut if_statements = Vec::new();
    compile_if(&if_else.if_,
               true,
               main,
               register_alloc,
               &mut symbol_alloc.clone(),
               fn_alloc,
               &mut if_statements,
               routines);
    if_statements.push(Jmp { location: Location::Relative(else_statements.len() as _) });

    statements.extend(if_statements);
    statements.extend(else_statements);
}

/// Compile let statement.
/// Compiles the expression which places the result at the current SP.
fn compile_let<B: ByteOrder>(let_: &ast::Let<'_>,
                             register_alloc: &mut RegisterAlloc,
                             symbol_alloc: &mut SymbolAlloc<B>,
                             fn_alloc: &FnAlloc,
                             statements: &mut Vec<Statement>) {
    compile_stack(&let_.field,
                  &let_.expression,
                  register_alloc,
                  symbol_alloc,
                  fn_alloc,
                  statements)
}

fn compile_stack<B: ByteOrder>(field: &ast::Field<'_>,
                               expression: &ast::Expression<'_>,
                               register_alloc: &mut RegisterAlloc,
                               symbol_alloc: &mut SymbolAlloc<B>,
                               fn_alloc: &FnAlloc,
                               statements: &mut Vec<Statement>) {
    // allocate memory on the stack for this field
    // the compiled expression should store the result on the stack
    let stack_address = symbol_alloc.alloc_stack_field(field);
    let field_layout = Layout::new(&field.type_);
    expression::compile_expression_into_pointer(expression,
                                                &field_layout,
                                                symbol_alloc,
                                                fn_alloc,
                                                Pointer::Stack(stack_address),
                                                register_alloc,
                                                statements);
}

/// Compile scope statement (or block statement).
fn compile_scope<B: ByteOrder>(scope: &ast::Scope<'_>,
                               main: bool,
                               register_alloc: &mut RegisterAlloc,
                               symbol_alloc: SymbolAlloc<B>,
                               fn_alloc: &mut FnAlloc,
                               statements: &mut Vec<Statement>,
                               routines: &mut Vec<Routine>) {
    // clone symbols so that any symbols created within the scope (in the stack)
    // will be freed at the end of the scope.
    let mut symbol_alloc = symbol_alloc;

    compile_statements(&scope.inner,
                       main,
                       register_alloc,
                       &mut symbol_alloc,
                       fn_alloc,
                       statements,
                       routines);
}

/// Compile const statement, which represent a symbol in ROM memory.
/// Declares the symbol as a constant symbol, to be used by later expressions.
/// Evaluates the expression (must be constexpr) and puts it into ROM.
fn compile_const<B: ByteOrder>(const_: &ast::Const<'_>, symbol_alloc: &mut SymbolAlloc<B>) {
    symbol_alloc.alloc_const(&const_.field, &const_.expression);
}

/// Compile static statement, which represents a symbol in RAM (mutable).
/// Same as the `compile_const` but the memory is not init, not in ROM.
fn compile_static<B: ByteOrder>(static_: &ast::Static<'_>, symbol_alloc: &mut SymbolAlloc<B>) {
    if let Some(offset) = &static_.offset {
        // static memory with explicit offset means the memory is located at the
        // absolute location in memory.
        let offset = expression::const_expr(&offset.expression, Some(symbol_alloc)).expect("Not a constant expression offset!");
        symbol_alloc.alloc_absolute(&static_.field, offset);
    } else {
        // otw the memory is allocated by the compiler in the static virtual memory
        // space.
        symbol_alloc.alloc_static(&static_.field);
    }
}

/// Compile fn statement into routines containing more statements.
/// Each routine is assigned an index given by the `FnAlloc`, and stored into
/// `routines` Vec at that index.
fn compile_fn<B: ByteOrder>(fn_: &ast::Fn<'_>,
                            main: bool,
                            register_alloc: &mut RegisterAlloc,
                            symbol_alloc: &mut SymbolAlloc<B>,
                            fn_alloc: &mut FnAlloc,
                            routines: &mut Vec<Routine>) {
    use Statement::Ret;

    // compile function with an empty stack (to represent the new stack frame).
    // static and consts and previously defined functions are still in scope.
    let mut symbol_alloc = symbol_alloc.clone();
    symbol_alloc.clear_stack();
    // allocate a new routine index/handle (used by the Call statement).
    // this is the index where the routine must be stored in Ir::routines.
    let _handle = fn_alloc.alloc(fn_);

    // allocate function parameters in the current stack frame without init.
    // the Statement::Call instruction should take care of init the values.
    if let Some(args) = &fn_.fn_arg {
        args.inner
            .iter()
            .map(|field| symbol_alloc.alloc_stack_field(field))
            .for_each(drop);
    }

    let mut statements = Vec::new();
    compile_statements(&fn_.inner,
                       main,
                       register_alloc,
                       &mut symbol_alloc,
                       fn_alloc,
                       &mut statements,
                       routines);

    statements.push(Ret);

    let name = Some(fn_.ident.to_string());
    routines.push(Routine { debug_name: name,
                            statements });
}
