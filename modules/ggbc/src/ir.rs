//! Definition and compilation of IR.
use crate::{ir::alloc::Space, parser::ast};
use alloc::{FnAlloc, RegisterAlloc, SymbolAlloc};
use layout::Layout;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

mod alloc;
mod expression;
mod layout;
mod utils;

pub type Address = u16;
pub type Register = usize;

/// Intermediate representation of a program.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default)]
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub struct Routine {
    /// Instructions of the routine.
    pub statements: Vec<Statement>,
}

/// Virtual memory pointers.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Pointer {
    /// Absolute pointer.
    Absolute(Address),
    /// Pointer in virtual static memory.
    Static(Address),
    /// Pointer in virtual const (ROM) memory.
    Const(Address),
    /// Pointer in virtual stack memory.
    Stack(Address),
}

/// Source from where to pull a value.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Source<T> {
    /// Data at the given address.
    Pointer(Pointer),
    /// Data at the given register.
    Register(Register),
    /// Literal data.
    Literal(T),
}

/// Destination where to store a value.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Destination {
    /// Store at the given address
    Pointer(Pointer),
    /// Store at the given register.
    Register(Register),
}

/// Jump location of `Jmp` and `Cmp` statements.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Location {
    /// Jump relative to the current program pointer.
    Relative(i8),
}

/// The statements, or instruction set of the IR.
#[rustfmt::skip]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Statement {
    /// Placeholder instruction.
    Nop,
    /// Statement to stop execution (end program)
    Stop,

    // move/load instructions
    Ld     { source: Source<u8>,  destination: Destination },
    Ld16   { source: Source<u16>, destination: Destination },
    // move/load of memory addresses
    LdAddr { source: Source<u16>, destination: Destination },

    // arithmetic (unary)
    Inc   { source: Source<u8>, destination: Destination },
    Dec   { source: Source<u8>, destination: Destination },
    Inc16 { source: Source<u16>, destination: Destination },
    Dec16 { source: Source<u16>, destination: Destination },

    // arithmetic (binary)
    Add { left: Source<u8>, right: Source<u8>, destination: Destination },
    Sub { left: Source<u8>, right: Source<u8>, destination: Destination },
    And { left: Source<u8>, right: Source<u8>, destination: Destination },
    Xor { left: Source<u8>, right: Source<u8>, destination: Destination },
    Or  { left: Source<u8>, right: Source<u8>, destination: Destination },

    // The following instructions may not be available in target architectures through a native instruction
    // In those cases, codegen must decompose to the corresponding equivalent bitwise ops (shifts & ands)
    // I haven't decided yet, but I may just reject the statement if none of the factors is a constexpr...
    Mul { left: Source<u8>, right: Source<u8>, destination: Destination },  // multiply
    Div { left: Source<u8>, right: Source<u8>, destination: Destination },  // division
    Rem { left: Source<u8>, right: Source<u8>, destination: Destination },  // Remainder

    // flow control
    Jmp { location: Location },
    /// jump to target if source == 0
    Cmp    { location: Location, source: Source<u8> },
    /// jump to target if source != 0
    CmpNot { location: Location, source: Source<u8> },

    // routines
    Call { routine: usize, args: Vec<Address>, destination: Option<Destination> },
    Ret,
}

pub fn compile(ast: &ast::Ast) -> Ir {
    use ast::Statement::*;
    use Statement::*;

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
    statements.push(Stop);

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
            Break(_) => compile_break(statements),
            Continue(_) => compile_continue(statements),
            _ => {}
        }
    }
}

/// Compile inline expression.
/// Compiles an expression which drops the result.
fn compile_inline(inline: &ast::Inline,
                  symbol_alloc: &mut SymbolAlloc,
                  fn_alloc: &FnAlloc,
                  statements: &mut Vec<Statement>) {
    // TODO placeholder implementation to begin texting the VM
    // assume "symbol = <byte>" statement
    use Statement::*;

    match &inline.inner {
        ast::Expression::Assign(node) => {
            // place value in a register.
            let mut register_alloc = RegisterAlloc::default();
            let register =
                expression::compile_expression_into_register8(&node.inner.right,
                                                              &Layout::U8,
                                                              symbol_alloc,
                                                              &mut register_alloc,
                                                              fn_alloc,
                                                              symbol_alloc.stack_address(),
                                                              statements);
            match &node.inner.left {
                ast::Expression::Path(path) => {
                    let name = utils::path_to_symbol_name(path);
                    let symbol = symbol_alloc.get(&name);
                    let destination = match symbol.space {
                        Space::Static => Destination::Pointer(Pointer::Static(symbol.offset)),
                        Space::Const => Destination::Pointer(Pointer::Const(symbol.offset)),
                        Space::Stack => Destination::Pointer(Pointer::Stack(symbol.offset)),
                        Space::Absolute => Destination::Pointer(Pointer::Absolute(symbol.offset)),
                    };
                    statements.push(Ld { source: Source::Register(register),
                                         destination });
                }
                _ => unimplemented!(),
            };
        }
        _ => unimplemented!(),
    }
}

/// Compile break statement
fn compile_break(statements: &mut Vec<Statement>) {
    use Statement::*;

    // in order to compile the Break statement, the compiler needs to know how many
    // instructions there are ahead of it. add placeholder Nop statement, which
    // should be replaced inside the compile_loop compile_for functions.
    statements.push(Nop);
}

/// Compile continue statement
fn compile_continue(statements: &mut Vec<Statement>) {
    use Statement::*;

    let relative = -(statements.len() as isize);
    statements.push(Jmp { location: Location::Relative(relative as _) });
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
    loop_statements.push(Jmp { location: Location::Relative(-(loop_statements.len() as i8)
                                                            as _) });

    // replace Nop statements (placeholders for break) with Jmp statements
    let statements_len = loop_statements.len();
    for (i, statement) in loop_statements.iter_mut().enumerate() {
        if *statement == Nop {
            let relative = statements_len - i;
            *statement = Jmp { location: Location::Relative(relative as _) }
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
    statements.push(CmpNot { location: Location::Relative((if_statements.len() + 1) as _),
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
    if_statements.push(Jmp { location: Location::Relative(else_statements.len() as _) });

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
