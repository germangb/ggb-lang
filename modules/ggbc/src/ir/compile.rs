use crate::{
    ir::{
        alloc::{FnAlloc, RegisterAlloc, SymbolAlloc},
        utils, Interrupts, Ir, Routine, Statement,
    },
    parser::{
        ast,
        ast::{Const, Fn, Let, Scope, Static},
        Ast,
    },
};
use ggbc_parser::ast::{Expression, Field};

mod expression;

pub fn compile(ast: &Ast) -> Ir {
    use crate::parser::ast::Statement::*;

    let mut routines = Vec::new();
    let mut symbol_alloc = SymbolAlloc::default();
    let mut fn_alloc = FnAlloc::default();
    let mut statements = Vec::new();

    compile_statements(
        true,
        &ast.inner,
        &mut symbol_alloc,
        &mut fn_alloc,
        &mut statements,
        &mut routines,
    );

    let main = routines.len();
    routines.push(Routine { statements });

    Ir {
        const_: Vec::new(),
        routines,
        interrupts: Interrupts::default(),
        main,
    }
}

/// Compile vec of statements.
fn compile_statements<'a>(
    main: bool,
    ast_statements: &'a [ast::Statement],
    symbol_alloc: &mut SymbolAlloc<'a>,
    fn_alloc: &mut FnAlloc<'a>,
    statements: &mut Vec<Statement>,
    routines: &mut Vec<Routine>,
) {
    use ast::Statement::*;

    for statement in ast_statements {
        match statement {
            Static(static_) if main => {
                compile_static(static_, symbol_alloc);
            }
            Const(const_) if main => {
                compile_const(const_, symbol_alloc);
            }
            Fn(fn_) if main => {
                compile_fn(fn_, symbol_alloc, fn_alloc, routines);
            }
            Let(let_) => {
                compile_stack(
                    &let_.field,
                    &let_.expression,
                    symbol_alloc,
                    fn_alloc,
                    statements,
                );
            }
            Scope(scope) => {
                compile_scope(scope, symbol_alloc.clone(), fn_alloc, statements, routines);
            }
            _ => {}
        }
    }
}

/// Compile scope statement.
fn compile_scope<'a>(
    scope: &'a Scope<'a>,
    mut symbol_alloc: SymbolAlloc<'a>,
    fn_alloc: &mut FnAlloc<'a>,
    statements: &mut Vec<Statement>,
    routines: &mut Vec<Routine>,
) {
    // push stack frame.
    statements.push(Statement::Push);

    compile_statements(
        false,
        &scope.inner,
        &mut symbol_alloc,
        fn_alloc,
        statements,
        routines,
    );

    // pop stack frame.
    // clears all stack memory created within the scope.
    statements.push(Statement::Pop);
}

/// compile "const" statement
fn compile_const<'a>(const_: &'a Const<'a>, symbol_alloc: &mut SymbolAlloc<'a>) {
    symbol_alloc.alloc_const(&const_.field, &const_.expression);
}

/// compile "static" statement
fn compile_static<'a>(static_: &'a Static<'a>, symbol_alloc: &mut SymbolAlloc<'a>) {
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

/// Compile "let" statement
fn compile_stack<'a>(
    field: &'a Field<'a>,
    expression: &'a Expression<'a>,
    symbol_alloc: &mut SymbolAlloc<'a>,
    fn_alloc: &FnAlloc,
    statements: &mut Vec<Statement>,
) {
    // allocate memory on the stack for this field
    // the compiled expression should store the result on the stack
    let stack_address = symbol_alloc.alloc_stack_field(&field);
    let mut register_alloc = RegisterAlloc::default();
    expression::compile_expression_into_stack(
        expression,
        &field.type_,
        symbol_alloc,
        &mut register_alloc,
        fn_alloc,
        stack_address,
        statements,
    );

    assert_eq!(0, register_alloc.len())
}

/// Compile fn statement.
fn compile_fn<'a>(
    fn_: &'a Fn,
    symbol_alloc: &mut SymbolAlloc<'a>,
    fn_alloc: &mut FnAlloc<'a>,
    routines: &mut Vec<Routine>,
) {
    // compile function with an empty virtual stack.
    // static and consts and previously defined functions are still in scope.
    let mut symbol_alloc = symbol_alloc.clone();
    symbol_alloc.clear_stack();
    fn_alloc.alloc(fn_);

    // allocate function parameters in the current stack frame without init
    // (basically advance the stack pointer)
    // the Statement::Call instruction is responsible for filling in the memory.
    if let Some(args) = &fn_.fn_arg {
        args.inner.iter().for_each(|field| {
            symbol_alloc.alloc_stack_field(field);
        });
    }

    let mut statements = Vec::new();

    compile_statements(
        false,
        &fn_.inner,
        &mut symbol_alloc,
        fn_alloc,
        &mut statements,
        routines,
    );

    routines.push(Routine { statements })
}
