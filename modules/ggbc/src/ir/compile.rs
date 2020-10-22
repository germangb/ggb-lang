use crate::{
    ir::{
        alloc::{FnAlloc, RegisterAlloc, SymbolAlloc},
        layout::Layout,
        utils, Interrupts, Ir, Routine, Statement,
    },
    parser::{
        ast,
        ast::{Const, Fn, Let, Scope, Static},
        Ast,
    },
};
use ggbc_parser::ast::{Expression, Field, Inline};

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
fn compile_statements(
    main: bool,
    ast_statements: &[ast::Statement],
    symbol_alloc: &mut SymbolAlloc,
    fn_alloc: &mut FnAlloc,
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
            Inline(inline) => compile_inline(inline, symbol_alloc, fn_alloc, statements),
            _ => {}
        }
    }
}

/// Compile inline expression
fn compile_inline(
    inline: &Inline,
    symbol_alloc: &SymbolAlloc,
    fn_alloc: &FnAlloc,
    statements: &mut Vec<Statement>,
) {
    // TODO properly implment expressions :(
}

/// Compile scope statement.
fn compile_scope(
    scope: &Scope,
    mut symbol_alloc: SymbolAlloc,
    fn_alloc: &mut FnAlloc,
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
fn compile_const(const_: &Const, symbol_alloc: &mut SymbolAlloc) {
    symbol_alloc.alloc_const(&const_.field, &const_.expression);
}

/// compile "static" statement
fn compile_static(static_: &Static, symbol_alloc: &mut SymbolAlloc) {
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
fn compile_stack(
    field: &Field,
    expression: &Expression,
    symbol_alloc: &mut SymbolAlloc,
    fn_alloc: &FnAlloc,
    statements: &mut Vec<Statement>,
) {
    // allocate memory on the stack for this field
    // the compiled expression should store the result on the stack
    let stack_address = symbol_alloc.alloc_stack_field(&field);
    let mut register_alloc = RegisterAlloc::default();
    let field_layout = Layout::from_type(&field.type_);
    expression::compile_expression_into_stack(
        expression,
        &field_layout,
        symbol_alloc,
        &mut register_alloc,
        fn_alloc,
        stack_address,
        statements,
    );

    assert_eq!(0, register_alloc.len())
}

/// Compile fn statement.
fn compile_fn(
    fn_: &Fn,
    symbol_alloc: &mut SymbolAlloc,
    fn_alloc: &mut FnAlloc,
    routines: &mut Vec<Routine>,
) {
    // compile function with an empty virtual stack.
    // static and consts and previously defined functions are still in scope.
    let mut symbol_alloc = symbol_alloc.clone();
    symbol_alloc.clear_stack();
    let fn_handle = fn_alloc.alloc(fn_);

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

    assert_eq!(fn_handle, routines.len());
    routines.push(Routine { statements })
}
