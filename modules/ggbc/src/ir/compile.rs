use crate::{
    ir::{
        alloc::{FnAlloc, RegisterAlloc, SymbolAlloc},
        layout::Layout,
        utils, Interrupts, Ir, Routine, Source, Statement,
        Statement::Nop,
        Target,
    },
    parser::ast,
};

mod expression;

pub fn compile(ast: &ast::Ast) -> Ir {
    use ast::Statement::*;

    let mut routines = Vec::new();
    let mut symbol_alloc = SymbolAlloc::default();
    let mut fn_alloc = FnAlloc::default();
    let mut statements = Vec::new();

    compile_statements(true,
                       &ast.inner,
                       &mut symbol_alloc,
                       &mut fn_alloc,
                       &mut statements,
                       &mut routines);

    let main = routines.len();
    routines.push(Routine { statements });

    Ir { const_: Vec::new(),
         routines,
         interrupts: Interrupts::default(),
         main }
}

/// Compile vec of statements.
fn compile_statements(main: bool,
                      ast_statements: &[ast::Statement],
                      symbol_alloc: &mut SymbolAlloc,
                      fn_alloc: &mut FnAlloc,
                      statements: &mut Vec<Statement>,
                      routines: &mut Vec<Routine>) {
    use ast::Statement::*;

    for statement in ast_statements {
        match statement {
            Static(static_) if main => compile_static(static_, symbol_alloc),
            Const(const_) if main => compile_const(const_, symbol_alloc),
            Fn(fn_) if main => compile_fn(fn_, symbol_alloc, fn_alloc, routines),
            Let(let_) => compile_let(let_, symbol_alloc, fn_alloc, statements),
            Scope(scope) => {
                compile_scope(scope, symbol_alloc.clone(), fn_alloc, statements, routines)
            }
            Inline(inline) => compile_inline(inline, symbol_alloc, fn_alloc, statements),
            If(if_) => compile_if(if_, symbol_alloc, fn_alloc, statements, routines),
            IfElse(if_else) => {
                compile_if_else(if_else, symbol_alloc, fn_alloc, statements, routines)
            }
            Loop(loop_) => compile_loop(loop_, symbol_alloc, fn_alloc, statements, routines),
            // in order to compile the Break statement, the compiler needs to know how many
            // instructions there are ahead of it. add placeholder Nop statement, which
            // should be replaced inside the compile_loop compile_for functions.
            Break(_) => statements.push(Nop),
            Continue(_) => compile_continue(statements),
            _ => {}
        }
    }
}

/// Compile continue statement
fn compile_continue(statements: &mut Vec<Statement>) {
    use Statement::*;

    let relative = -(statements.len() as isize);
    statements.push(Jmp { target: Target::Relative(relative as _) });
}

/// Compile loop statement
fn compile_loop(loop_: &ast::Loop,
                symbol_alloc: &mut SymbolAlloc,
                fn_alloc: &mut FnAlloc,
                statements: &mut Vec<Statement>,
                routines: &mut Vec<Routine>) {
    use Statement::*;

    // compile statements inside the loop block
    // at the end, jump back to the first statement
    let mut loop_statements = Vec::new();
    compile_statements(false,
                       &loop_.inner,
                       &mut symbol_alloc.clone(),
                       fn_alloc,
                       &mut loop_statements,
                       routines);
    loop_statements.push(Jmp { target: Target::Relative(-(loop_statements.len() as i8) as _) });

    // replace Nop statements (placeholders for break) with Jmp statements
    let statements_len = loop_statements.len();
    for (i, statement) in loop_statements.iter_mut().enumerate() {
        if *statement == Nop {
            let relative = statements_len - i;
            *statement = Jmp { target: Target::Relative(relative as _) }
        }
    }
    statements.extend(loop_statements);
}

/// Compile if statement.
fn compile_if(if_: &ast::If,
              symbol_alloc: &mut SymbolAlloc,
              fn_alloc: &mut FnAlloc,
              statements: &mut Vec<Statement>,
              routines: &mut Vec<Routine>) {
    use Statement::*;

    // compile expression into an 8bit register
    let layout = Layout::U8;
    let mut register_alloc = RegisterAlloc::default();
    let register = expression::compile_expression_into_register8(&if_.expression,
                                                                 &layout,
                                                                 symbol_alloc,
                                                                 &mut register_alloc,
                                                                 fn_alloc,
                                                                 symbol_alloc.stack_address(),
                                                                 statements);
    // compile the block of statements inside the if block.
    // clone the symbol_alloc to free any symbols defined within the block.
    let mut if_statements = Vec::new();
    compile_statements(false,
                       &if_.inner,
                       &mut symbol_alloc.clone(),
                       fn_alloc,
                       &mut if_statements,
                       routines);

    // TODO what if if_statements.len() is > i8::max_value() ?
    statements.push(CmpNot { target: Target::Relative((if_statements.len() + 1) as _),
                             source: Source::Register(register) });
    statements.extend(if_statements);

    // cleanup register, which is a bit superfluous to do this here, but just to
    // make sure the above fn is correct...
    register_alloc.free(register);
    assert_eq!(0, register_alloc.len());
}

/// Compile if else statement.
fn compile_if_else(if_else: &ast::IfElse,
                   symbol_alloc: &mut SymbolAlloc,
                   fn_alloc: &mut FnAlloc,
                   statements: &mut Vec<Statement>,
                   routines: &mut Vec<Routine>) {
    use Statement::*;

    // compile else block statements
    let mut else_statements = Vec::new();
    compile_statements(false,
                       &if_else.else_.inner,
                       &mut symbol_alloc.clone(),
                       fn_alloc,
                       &mut else_statements,
                       routines);
    // compile if statement
    // append a jump statement at the end.
    let mut if_statements = Vec::new();
    compile_if(&if_else.if_,
               &mut symbol_alloc.clone(),
               fn_alloc,
               &mut if_statements,
               routines);
    if_statements.push(Jmp { target: Target::Relative(else_statements.len() as _) });

    statements.extend(if_statements);
    statements.extend(else_statements);
}

/// Compile let statement.
/// Compiles the expression which places the result at the current SP.
fn compile_let(let_: &ast::Let,
               symbol_alloc: &mut SymbolAlloc,
               fn_alloc: &FnAlloc,
               statements: &mut Vec<Statement>) {
    compile_stack(&let_.field,
                  &let_.expression,
                  symbol_alloc,
                  fn_alloc,
                  statements)
}

fn compile_stack(field: &ast::Field,
                 expression: &ast::Expression,
                 symbol_alloc: &mut SymbolAlloc,
                 fn_alloc: &FnAlloc,
                 statements: &mut Vec<Statement>) {
    use Statement::*;

    // allocate memory on the stack for this field
    // the compiled expression should store the result on the stack
    let stack_address = symbol_alloc.alloc_stack_field(&field);
    let mut register_alloc = RegisterAlloc::default();
    let field_layout = Layout::from_type(&field.type_);
    expression::compile_expression_into_stack(expression,
                                              &field_layout,
                                              symbol_alloc,
                                              &mut register_alloc,
                                              fn_alloc,
                                              stack_address,
                                              statements);

    assert_eq!(0, register_alloc.len())
}

/// Compile inline expression.
/// Compiles an expression which drops the result.
fn compile_inline(inline: &ast::Inline,
                  symbol_alloc: &SymbolAlloc,
                  fn_alloc: &FnAlloc,
                  statements: &mut Vec<Statement>) {
    // TODO properly implment expressions :(
}

/// Compile scope statement (or block statement).
fn compile_scope(scope: &ast::Scope,
                 mut symbol_alloc: SymbolAlloc,
                 fn_alloc: &mut FnAlloc,
                 statements: &mut Vec<Statement>,
                 routines: &mut Vec<Routine>) {
    use Statement::*;

    // clone symbols so that any symbols created within the scope (in the stack)
    // will be freed at the end of the scope.
    let mut symbol_alloc = symbol_alloc.clone();

    compile_statements(false,
                       &scope.inner,
                       &mut symbol_alloc,
                       fn_alloc,
                       statements,
                       routines);
}

/// Compile const statement, which represent a symbol in ROM memory.
/// Declares the symbol as a constant symbol, to be used by later expressions.
/// Evaluates the expression (must be constexpr) and puts it into ROM.
fn compile_const(const_: &ast::Const, symbol_alloc: &mut SymbolAlloc) {
    symbol_alloc.alloc_const(&const_.field, &const_.expression);
}

/// Compile static statement, which represents a symbol in RAM (mutable).
/// Same as the `compile_const` but the memory is not init, not in ROM.
fn compile_static(static_: &ast::Static, symbol_alloc: &mut SymbolAlloc) {
    if let Some(offset) = &static_.offset {
        // static memory with explicit offset means the memory is located at the
        // absolute location in memory.
        let offset = utils::compute_const_expression(&offset.expression);
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
fn compile_fn(fn_: &ast::Fn,
              symbol_alloc: &mut SymbolAlloc,
              fn_alloc: &mut FnAlloc,
              routines: &mut Vec<Routine>) {
    use Statement::*;

    // compile function with an empty stack (to represent the new stack frame).
    // static and consts and previously defined functions are still in scope.
    let mut symbol_alloc = symbol_alloc.clone();
    symbol_alloc.clear_stack();
    // allocate a new routine index/handle (used by the Call statement).
    // this is the index where the routine must be stored in Ir::routines.
    let handle = fn_alloc.alloc(fn_);

    // allocate function parameters in the current stack frame without init.
    // the Statement::Call instruction should take care of init the values.
    if let Some(args) = &fn_.fn_arg {
        args.inner
            .iter()
            .map(|field| symbol_alloc.alloc_stack_field(field))
            .for_each(drop);
    }

    let mut statements = Vec::new();
    compile_statements(false,
                       &fn_.inner,
                       &mut symbol_alloc,
                       fn_alloc,
                       &mut statements,
                       routines);

    assert_eq!(handle, routines.len());
    routines.push(Routine { statements })
}
