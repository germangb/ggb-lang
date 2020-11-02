//! Intermediate representation language.
use crate::{
    byteorder::{ByteOrder, NativeEndian},
    ir::expression::{
        compile_expr_register, compile_expr_void, compile_expression_into_pointer,
        compute_const_expr,
    },
    parser::ast,
};
use alloc::{FnAlloc, RegisterAlloc, SymbolAlloc};
use layout::Layout;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

mod alloc;
mod expression;
mod layout;

pub type Address = u16;
pub type Register = usize;
pub type Word = u16;

/// Intermediate representation of a program.
///
/// Generic over the byte ordering `B` of the bytes in `const_`.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub struct Ir<B: ByteOrder = NativeEndian> {
    const_: Vec<u8>,
    routines: Vec<Routine>,
    handlers: Handlers,
    _phantom: PhantomData<B>,
}

impl<B: ByteOrder> Ir<B> {
    /// Convert AST into IR intermediate code.
    pub fn new(ast: &ast::Ast) -> Self {
        use ast::Statement::*;
        use Statement::*;

        //let mut context = Context::default();
        let mut routines = Vec::new();
        let mut register_alloc = RegisterAlloc::default();
        let mut symbol_alloc: SymbolAlloc<B> = SymbolAlloc::default();
        let mut fn_alloc = FnAlloc::default();
        let mut statements = Vec::new();

        // wrap the statements in between Nop instructions (just in case the program
        // begins with a loop
        statements.push(Nop(0));
        compile_statements(&ast.inner,
                           true,
                           &mut register_alloc,
                           &mut symbol_alloc,
                           &mut fn_alloc,
                           &mut statements,
                           &mut routines);
        statements.push(Nop(0));
        statements.push(Stop);

        let main = routines.len();
        routines.push(Routine { debug_name: None,
                                statements });

        Self { const_: symbol_alloc.into_const_data(),
               routines,
               handlers: Handlers { main,
                                    ..Default::default() },
               _phantom: PhantomData }
    }

    /// Returns the memory corresponding to const statements (ROM).
    pub fn const_(&self) -> &[u8] {
        &self.const_
    }

    /// Returns the compiled IR routines.
    pub fn routines(&self) -> &[Routine] {
        &self.routines
    }

    /// Returns the main routine.
    pub fn main(&self) -> &Routine {
        &self.routines[self.handlers.main]
    }

    /// Return interrupt handlers.
    pub fn int(&self) -> Interrupts {
        Interrupts { routines: self.routines(),
                     handlers: &self.handlers }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default)]
struct Handlers {
    main: usize,
    vblank: Option<usize>,
    lcd_stat: Option<usize>,
    timer: Option<usize>,
    serial: Option<usize>,
    joypad: Option<usize>,
}

/// Routine handles for each type of interrupt.
#[derive(Debug)]
pub struct Interrupts<'a> {
    routines: &'a [Routine],
    handlers: &'a Handlers,
}

impl<'a> Interrupts<'a> {
    pub fn vblank(&self) -> Option<&Routine> {
        self.handlers.vblank.map(|i| &self.routines[i])
    }

    pub fn lcd_stat(&self) -> Option<&Routine> {
        self.handlers.lcd_stat.map(|i| &self.routines[i])
    }

    pub fn timer(&self) -> Option<&Routine> {
        self.handlers.timer.map(|i| &self.routines[i])
    }

    pub fn serial(&self) -> Option<&Routine> {
        self.handlers.serial.map(|i| &self.routines[i])
    }

    pub fn joypad(&self) -> Option<&Routine> {
        self.handlers.joypad.map(|i| &self.routines[i])
    }
}

/// Data associated with a compiled IR routine.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub struct Routine {
    /// Optional routine name (for debugging purposes).
    pub debug_name: Option<String>,
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

impl Pointer {
    pub fn offset(self, offset: Address) -> Self {
        use Pointer::*;
        match self {
            Absolute(a) => Absolute(a + offset),
            Static(a) => Static(a + offset),
            Const(a) => Const(a + offset),
            Stack(a) => Stack(a + offset),
        }
    }
}

/// Source from where to pull a value.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Source<T> {
    /// Data at the given address.
    Pointer {
        /// The base pointer itself.
        base: Pointer,
        /// Dynamic applied to the address of the pointer.
        offset: Option<Box<Source<Address>>>,
    },
    /// Data at the given register.
    Register(Register),
    /// Literal data.
    Literal(T),
}

/// Destination where to store a value.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Destination {
    /// Store at the given address
    Pointer {
        /// The base pointer itself.
        base: Pointer,
        /// Dynamic applied to the address of the pointer.
        offset: Option<Box<Source<Address>>>,
    },
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Statement {
    /// Placeholder instructions.
    Nop(usize),
    /// Statement to stop execution (end program)
    Stop,

    // move/load instructions
    Ld {
        source: Source<u8>,
        destination: Destination,
    },
    LdW {
        source: Source<Word>,
        destination: Destination,
    },
    // move/load of memory addresses
    LdAddr {
        source: Source<Address>,
        destination: Destination,
    },

    // arithmetic (unary)
    Inc {
        source: Source<u8>,
        destination: Destination,
    },
    Dec {
        source: Source<u8>,
        destination: Destination,
    },
    IncW {
        source: Source<Word>,
        destination: Destination,
    },
    DecW {
        source: Source<Word>,
        destination: Destination,
    },

    // arithmetic (binary)
    Add {
        left: Source<u8>,
        right: Source<u8>,
        destination: Destination,
    },
    Sub {
        left: Source<u8>,
        right: Source<u8>,
        destination: Destination,
    },
    And {
        left: Source<u8>,
        right: Source<u8>,
        destination: Destination,
    },
    Xor {
        left: Source<u8>,
        right: Source<u8>,
        destination: Destination,
    },
    Or {
        left: Source<u8>,
        right: Source<u8>,
        destination: Destination,
    },

    AddW {
        left: Source<Word>,
        right: Source<Word>,
        destination: Destination,
    },
    SubW {
        left: Source<Word>,
        right: Source<Word>,
        destination: Destination,
    },
    AndW {
        left: Source<Word>,
        right: Source<Word>,
        destination: Destination,
    },
    XorW {
        left: Source<Word>,
        right: Source<Word>,
        destination: Destination,
    },
    OrW {
        left: Source<Word>,
        right: Source<Word>,
        destination: Destination,
    },

    // The following instructions may not be available in target architectures through a native
    // instruction. In those cases, codegen might decompose to the corresponding equivalent
    // bitwise operations.
    Mul {
        left: Source<u8>,
        right: Source<u8>,
        destination: Destination,
    }, // multiply
    Div {
        left: Source<u8>,
        right: Source<u8>,
        destination: Destination,
    },
    MulW {
        left: Source<Word>,
        right: Source<Word>,
        destination: Destination,
    }, // multiply
    DivW {
        left: Source<Word>,
        right: Source<Word>,
        destination: Destination,
    },
    Rem {
        left: Source<u8>,
        right: Source<u8>,
        destination: Destination,
    },

    // flow control
    Jmp {
        /// Jump location.
        location: Location,
    },
    /// jump to target if source == 0
    JmpCmp {
        /// Jump location
        location: Location,
        source: Source<u8>,
    },
    /// jump to target if source != 0
    JmpCmpNot {
        /// Jump location.
        location: Location,
        source: Source<u8>,
    },

    // routines
    Call {
        /// Routine index in `Ir::routines`
        routine: usize,
        /// Address of the beginning of the function stack frame.
        address: Address,
        destination: Option<Destination>,
    },
    // TODO return value
    Ret,
}

/// Compile vec of statements.
fn compile_statements<B: ByteOrder>(ast_statements: &[ast::Statement],
                                    main: bool,
                                    register_alloc: &mut RegisterAlloc,
                                    symbol_alloc: &mut SymbolAlloc<B>,
                                    fn_alloc: &mut FnAlloc,
                                    statements: &mut Vec<Statement>,
                                    routines: &mut Vec<Routine>) {
    use ast::Statement::*;
    use Statement::*;

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
            Loop(loop_) => compile_loop(&loop_,
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
fn compile_inline<B: ByteOrder>(inline: &ast::Inline,
                                register_alloc: &mut RegisterAlloc,
                                symbol_alloc: &mut SymbolAlloc<B>,
                                fn_alloc: &FnAlloc,
                                statements: &mut Vec<Statement>) {
    // compile expression and drop the results.
    // the expression will be evaluated by the result is not stored anywhere.
    compile_expr_void(&inline.inner,
                      symbol_alloc,
                      fn_alloc,
                      register_alloc,
                      statements)
}

/// Compile break statement
fn compile_break(statements: &mut Vec<Statement>) {
    use Statement::*;

    // in order to compile the Break statement, the compiler needs to know how many
    // instructions there are ahead of it. add placeholder Nop statement, which
    // should be replaced inside the compile_loop compile_for functions.
    statements.push(Nop(1));
}

/// Compile continue statement
fn compile_continue(statements: &mut Vec<Statement>) {
    use Statement::*;

    // same deal as with the break statement.
    // use a different Nop to differentiate it.
    statements.push(Nop(2));
}

/// Compile loop statement
fn compile_for<B: ByteOrder>(for_: &ast::For,
                             main: bool,
                             register_alloc: &mut RegisterAlloc,
                             symbol_alloc: &mut SymbolAlloc<B>,
                             fn_alloc: &mut FnAlloc,
                             statements: &mut Vec<Statement>,
                             routines: &mut Vec<Routine>) {
    use Statement::*;

    // for loop is computed as a special case of the generic loop
    // initializes the
    let mut symbol_alloc = symbol_alloc.clone();
    let mut for_statements = Vec::new();
    let stack_address = symbol_alloc.alloc_stack_field(&for_.field);

    // init for variable with the lhs side of the range
    // TODO non-U8 variables
    let init_register = compile_expr_register(&for_.range.left,
                                              &Layout::U8,
                                              &mut symbol_alloc.clone(),
                                              fn_alloc,
                                              register_alloc,
                                              statements);
    statements.push(Ld { source: Source::Register(init_register),
                         destination: Destination::Pointer { base:
                                                                 Pointer::Stack(stack_address),
                                                             offset: None } });
    register_alloc.free(init_register);

    // compute end index of the for loop with the rhs of the range
    // increment if it's an inclusive range
    let end_register = compile_expr_register(&for_.range.right,
                                             &Layout::U8,
                                             &mut symbol_alloc.clone(),
                                             fn_alloc,
                                             register_alloc,
                                             statements);
    if for_.range.eq.is_some() {
        statements.push(Inc { source: Source::Register(end_register),
                              destination: Destination::Register(end_register) });
    }

    // begin compiling the inner for loop statements.
    // check if for loop variable has reached the limit.
    let cmp_register = register_alloc.alloc();
    let mut prefix = vec![Sub { left: Source::Register(end_register),
                                right: Source::Pointer { base: Pointer::Stack(stack_address),
                                                         offset: None },
                                destination: Destination::Register(cmp_register) },
                          // TODO optimize away, as this is equivalent to: if foo { break }
                          JmpCmp { location: Location::Relative(1),
                                   source: Source::Register(cmp_register) },
                          Nop(1),];
    register_alloc.free(cmp_register);
    // increment the for loop variable
    let mut suffix =
        vec![Inc { source: Source::Pointer { base: Pointer::Stack(stack_address),
                                             offset: None },
                   destination: Destination::Pointer { base: Pointer::Stack(stack_address),
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
fn compile_loop<B: ByteOrder>(loop_: &ast::Loop,
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
fn compile_loop_statements<B: ByteOrder>(loop_: &[ast::Statement],
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
    use Location::*;
    use Statement::*;

    // compile statements inside the loop block
    // at the end, jump back to the first statement
    let mut loop_statements = prefix_statements;
    compile_statements(&loop_,
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
            Nop(1) => {
                let relative = statements_len - i - 1;
                *statement = Jmp { location: Location::Relative(relative as _) };
            }
            // continue
            Nop(2) => {
                let relative = i as isize + 1;
                *statement = Jmp { location: Location::Relative(-relative as _) };
            }
            _ => {}
        }
    }
    statements.extend(loop_statements);
}

/// Compile if statement.
fn compile_if<B: ByteOrder>(if_: &ast::If,
                            main: bool,
                            register_alloc: &mut RegisterAlloc,
                            symbol_alloc: &mut SymbolAlloc<B>,
                            fn_alloc: &mut FnAlloc,
                            statements: &mut Vec<Statement>,
                            routines: &mut Vec<Routine>) {
    use Statement::*;

    // compile expression into an 8bit register
    let layout = Layout::U8;
    let register = compile_expr_register(&if_.expression,
                                         &layout,
                                         symbol_alloc,
                                         fn_alloc,
                                         register_alloc,
                                         statements);
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

    // TODO what if if_statements.len() is > i8::max_value() ?
    let jmp = if_statements.len() + 1 + 2; // FIXME if i remove this +3, the loop example doesn't work. Fix this please :(
    statements.push(JmpCmpNot { location: Location::Relative(jmp as _),
                                source: Source::Register(register) });
    statements.extend(if_statements);
    register_alloc.free(register);
}

/// Compile if else statement.
fn compile_if_else<B: ByteOrder>(if_else: &ast::IfElse,
                                 main: bool,
                                 register_alloc: &mut RegisterAlloc,
                                 symbol_alloc: &mut SymbolAlloc<B>,
                                 fn_alloc: &mut FnAlloc,
                                 statements: &mut Vec<Statement>,
                                 routines: &mut Vec<Routine>) {
    use Statement::*;

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
fn compile_let<B: ByteOrder>(let_: &ast::Let,
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

fn compile_stack<B: ByteOrder>(field: &ast::Field,
                               expression: &ast::Expression,
                               register_alloc: &mut RegisterAlloc,
                               symbol_alloc: &mut SymbolAlloc<B>,
                               fn_alloc: &FnAlloc,
                               statements: &mut Vec<Statement>) {
    use Statement::*;

    // allocate memory on the stack for this field
    // the compiled expression should store the result on the stack
    let stack_address = symbol_alloc.alloc_stack_field(&field);
    let field_layout = Layout::new(&field.type_);
    compile_expression_into_pointer(expression,
                                    &field_layout,
                                    symbol_alloc,
                                    fn_alloc,
                                    Pointer::Stack(stack_address),
                                    register_alloc,
                                    statements);
}

/// Compile scope statement (or block statement).
fn compile_scope<B: ByteOrder>(scope: &ast::Scope,
                               main: bool,
                               register_alloc: &mut RegisterAlloc,
                               mut symbol_alloc: SymbolAlloc<B>,
                               fn_alloc: &mut FnAlloc,
                               statements: &mut Vec<Statement>,
                               routines: &mut Vec<Routine>) {
    use Statement::*;

    // clone symbols so that any symbols created within the scope (in the stack)
    // will be freed at the end of the scope.
    let mut symbol_alloc = symbol_alloc.clone();

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
fn compile_const<B: ByteOrder>(const_: &ast::Const, symbol_alloc: &mut SymbolAlloc<B>) {
    symbol_alloc.alloc_const(&const_.field, &const_.expression);
}

/// Compile static statement, which represents a symbol in RAM (mutable).
/// Same as the `compile_const` but the memory is not init, not in ROM.
fn compile_static<B: ByteOrder>(static_: &ast::Static, symbol_alloc: &mut SymbolAlloc<B>) {
    if let Some(offset) = &static_.offset {
        // static memory with explicit offset means the memory is located at the
        // absolute location in memory.
        let offset = compute_const_expr(&offset.expression);
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
fn compile_fn<B: ByteOrder>(fn_: &ast::Fn,
                            main: bool,
                            register_alloc: &mut RegisterAlloc,
                            symbol_alloc: &mut SymbolAlloc<B>,
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
