use crate::{
    byteorder::ByteOrder,
    ir::{
        alloc::{FnAlloc, RegisterAlloc, SymbolAlloc},
        layout::Layout,
        Destination, Location, Pointer, Routine, Source, Statement,
        Statement::{Inc, Jmp, JmpCmp, Ld, Ret, Stop, Sub},
        NOP_BREAK, NOP_CONTINUE, NOP_PERSIST,
    },
    parser::ast,
};
use Statement::{JmpCmpNot, Nop};

/// Ir compilation context.
#[derive(Default)]
pub struct Context<B: ByteOrder> {
    /// optimization flag.
    pub optimize: bool,

    /// Layout of the return type. This field will be some when compiling a
    /// function statement that returns some type.
    pub return_: Option<Layout>,

    /// Compiled routines.
    pub routines: Vec<Routine>,

    /// Static allocation.
    pub symbol_alloc: SymbolAlloc<B>,

    /// fn alloc
    pub fn_alloc: FnAlloc,

    /// Register allocation.
    pub register_alloc: RegisterAlloc,
}

pub trait Compile {
    fn compile<B: ByteOrder>(&self, context: &mut Context<B>, out: &mut Vec<Statement>);
}

impl Compile for Vec<ast::Statement<'_>> {
    fn compile<B: ByteOrder>(&self, context: &mut Context<B>, out: &mut Vec<Statement>) {
        for statement in self {
            match statement {
                ast::Statement::Write(_) => todo!(),
                ast::Statement::Read(_) => todo!(),
                ast::Statement::If(if_) => if_.compile(context, out),
                ast::Statement::IfElse(_if_else) => todo!(),
                ast::Statement::Scope(scope) => scope.compile(context, out),
                ast::Statement::Panic(panic) => panic.compile(context, out),
                ast::Statement::Mod(_) => todo!(),
                ast::Statement::Static(static_) => static_.compile(context, out),
                ast::Statement::Const(const_) => const_.compile(context, out),
                ast::Statement::Let(let_) => let_.compile(context, out),
                ast::Statement::For(for_) => for_.compile(context, out),
                ast::Statement::Loop(loop_) => loop_.compile(context, out),
                ast::Statement::Continue(continue_) => continue_.compile(context, out),
                ast::Statement::Break(break_) => break_.compile(context, out),
                ast::Statement::Inline(inline) => inline.compile(context, out),
                ast::Statement::Fn(fn_) => fn_.compile(context, out),
                ast::Statement::Return(return_) => return_.compile(context, out),
            }
        }
    }
}

impl Compile for ast::Ast<'_> {
    fn compile<B: ByteOrder>(&self, context: &mut Context<B>, out: &mut Vec<Statement>) {
        out.push(Nop(NOP_PERSIST));
        self.inner.compile(context, out);
        out.push(Stop);
        super::optimize::optimize(out);
    }
}

impl Compile for ast::Panic<'_> {
    fn compile<B: ByteOrder>(&self, _: &mut Context<B>, out: &mut Vec<Statement>) {
        out.push(Stop);
    }
}

impl Compile for ast::Scope<'_> {
    fn compile<B: ByteOrder>(&self, context: &mut Context<B>, out: &mut Vec<Statement>) {
        // push static symbols in the parent scope (to be restored later)
        // all symbols defined within the child scope aren't "visible" outside of it.
        let child = context.symbol_alloc.clone();
        let parent: SymbolAlloc<B> = std::mem::replace(&mut context.symbol_alloc, child);

        // compile statements normally.
        self.inner.compile(context, out);

        // restore symbols
        let _ = std::mem::replace(&mut context.symbol_alloc, parent);
    }
}

impl Compile for ast::Static<'_> {
    fn compile<B: ByteOrder>(&self, context: &mut Context<B>, _: &mut Vec<Statement>) {
        if let Some(offset) = &self.offset {
            // static memory with explicit offset means the memory is located at the
            // absolute location in memory.
            let symbol_alloc = &context.symbol_alloc;
            let offset = super::expression::const_expr(&offset.expression, Some(symbol_alloc)).expect("Not a constant expression offset!");
            context.symbol_alloc.alloc_absolute(&self.field, offset);
        } else {
            // otw the memory is allocated by the compiler in the static virtual memory
            // space.
            context.symbol_alloc.alloc_static(&self.field);
        }
    }
}

impl Compile for ast::Const<'_> {
    fn compile<B: ByteOrder>(&self, context: &mut Context<B>, _: &mut Vec<Statement>) {
        context.symbol_alloc
               .alloc_const(&self.field, &self.expression);
    }
}

impl Compile for ast::Let<'_> {
    fn compile<B: ByteOrder>(&self, context: &mut Context<B>, out: &mut Vec<Statement>) {
        // allocate memory on the stack for this field
        // the compiled expression should store the result on the stack
        let stack_address = context.symbol_alloc.alloc_stack_field(&self.field);
        let field_layout = Layout::new(&self.field.type_);
        super::expression::compile_expression_into_pointer(&self.expression,
                                                           &field_layout,
                                                           &context.symbol_alloc,
                                                           &context.fn_alloc,
                                                           Pointer::Stack(stack_address),
                                                           &mut context.register_alloc,
                                                           out);
    }
}

impl Compile for ast::Inline<'_> {
    fn compile<B: ByteOrder>(&self, context: &mut Context<B>, out: &mut Vec<Statement>) {
        // compile expression and drop the results.
        // the expression will be evaluated by the result is not stored anywhere.
        super::expression::compile_expr_void(&self.inner,
                                             &context.symbol_alloc,
                                             &context.fn_alloc,
                                             &mut context.register_alloc,
                                             out)
    }
}

impl Compile for ast::If<'_> {
    fn compile<B: ByteOrder>(&self, context: &mut Context<B>, out: &mut Vec<Statement>) {
        // push static symbols in the parent scope (to be restored later)
        // all symbols defined within the child scope aren't "visible" outside of it.
        let child = context.symbol_alloc.clone();
        let parent: SymbolAlloc<B> = std::mem::replace(&mut context.symbol_alloc, child);

        match super::expression::const_expr(&self.expression, Some(&context.symbol_alloc)) {
            Some(0) => { /* Optimization: don't compile if statement */ }
            Some(_) => {
                // Optimization: don't compile the if conditional
                self.inner.compile(context, out)
            }
            None => {
                // compile expression into an 8bit register
                let source = super::expression::compile_expr(&self.expression,
                                                             &context.symbol_alloc,
                                                             &context.fn_alloc,
                                                             &mut context.register_alloc,
                                                             out);
                super::expression::free_source_registers(&source, &mut context.register_alloc);

                // compile the block of statements inside the if block.
                // clone the symbol_alloc to free any symbols defined within the block.
                let mut inner = Vec::new();
                self.inner.compile(context, &mut inner);

                let jmp = inner.len() /*+ if has_else { 1 } else { 0 } */;
                out.push(JmpCmpNot { location: Location::Relative(jmp as _),
                                     source });
                out.extend(inner);
            }
        }

        // restore symbols
        let _ = std::mem::replace(&mut context.symbol_alloc, parent);
    }
}

/// Wrapper around inner loop statements, indicating that a list of statements
/// corresponds to the inner statements of a loop (be it a for loop, or a
/// canonical loop).
struct LoopInner<'a, 'b> {
    prefix: Vec<Statement>,
    inner: &'a Vec<ast::Statement<'b>>,
    suffix: Vec<Statement>,
}

impl Compile for LoopInner<'_, '_> {
    fn compile<B: ByteOrder>(&self, context: &mut Context<B>, out: &mut Vec<Statement>) {
        // compile statements inside the loop block
        // at the end, jump back to the first statement
        let mut inner = Vec::new();

        inner.extend_from_slice(&self.prefix);
        self.inner.compile(context, &mut inner);
        inner.extend_from_slice(&self.suffix);

        let loop_statements_signed = inner.len() as isize;
        inner.push(Jmp { location: Location::Relative(-(loop_statements_signed + 1) as i8) });

        // replace Nop statements (placeholders for break and continue) with the
        // appropriate Jmp statements. Don't worry about optimizing jumps for now. That
        // will be dealt with later...
        let statements_len = inner.len();
        for (i, statement) in inner.iter_mut().enumerate() {
            match statement {
                // break
                Nop(NOP_BREAK) => {
                    let relative = statements_len - i - 1;
                    *statement = Jmp { location: Location::Relative(relative as _) };
                }
                // continue
                Nop(NOP_CONTINUE) => {
                    let relative = i as isize + 1;
                    *statement = Jmp { location: Location::Relative(-relative as _) };
                }
                _ => {}
            }
        }
        out.extend(inner);
    }
}

impl Compile for ast::Loop<'_> {
    fn compile<B: ByteOrder>(&self, context: &mut Context<B>, out: &mut Vec<Statement>) {
        // push static symbols in the parent scope (to be restored later)
        // all symbols defined within the child scope aren't "visible" outside of it.
        let child = context.symbol_alloc.clone();
        let parent: SymbolAlloc<B> = std::mem::replace(&mut context.symbol_alloc, child);

        LoopInner { prefix: Vec::new(),
                    inner: &self.inner,
                    suffix: Vec::new() }.compile(context, out);

        // restore symbols
        let _ = std::mem::replace(&mut context.symbol_alloc, parent);
    }
}

impl Compile for ast::For<'_> {
    fn compile<B: ByteOrder>(&self, context: &mut Context<B>, out: &mut Vec<Statement>) {
        // push static symbols in the parent scope (to be restored later)
        // all symbols defined within the child scope aren't "visible" outside of it.
        let child = context.symbol_alloc.clone();
        let parent: SymbolAlloc<B> = std::mem::replace(&mut context.symbol_alloc, child);

        {
            let mut for_statements = Vec::new();
            let stack_address = context.symbol_alloc.alloc_stack_field(&self.field);

            // init for variable with the lhs side of the range
            // TODO non-U8 variables
            let init = super::expression::compile_expr(&self.range.left,
                                                       &context.symbol_alloc,
                                                       &context.fn_alloc,
                                                       &mut context.register_alloc,
                                                       out);
            super::expression::free_source_registers(&init, &mut context.register_alloc);
            out.push(Ld { source: init,
                          destination: Destination::Pointer { base:
                                                                  Pointer::Stack(stack_address),
                                                              offset: None } });

            // compute end index of the for loop with the rhs of the range
            // increment if it's an inclusive range
            let end = super::expression::compile_expr(&self.range.right,
                                                      &context.symbol_alloc,
                                                      &context.fn_alloc,
                                                      &mut context.register_alloc,
                                                      out);
            let end_register = context.register_alloc.alloc();
            out.push(Statement::Ld { source: end.clone(),
                                     destination: Destination::Register(end_register) });
            super::expression::free_source_registers(&end, &mut context.register_alloc);
            if self.range.eq.is_some() {
                out.push(Inc { source: Source::Register(end_register),
                               destination: Destination::Register(end_register) });
            }

            // begin compiling the inner for loop statements.
            // check if for loop variable has reached the limit.
            let cmp_register = context.register_alloc.alloc();
            let prefix = vec![Sub { left: Source::Register(end_register),
                                    right: Source::Pointer { base:
                                                                 Pointer::Stack(stack_address),
                                                             offset: None },
                                    destination: Destination::Register(cmp_register) },
                              // TODO optimize away, as this is equivalent to: if foo { break }
                              JmpCmp { location: Location::Relative(1),
                                       source: Source::Register(cmp_register) },
                              Nop(NOP_BREAK),];
            context.register_alloc.free(cmp_register);
            // increment the for loop variable
            let suffix =
                vec![Inc { source: Source::Pointer { base: Pointer::Stack(stack_address),
                                                     offset: None },
                           destination: Destination::Pointer { base:
                                                                   Pointer::Stack(stack_address),
                                                               offset: None } }];

            // parse inner loop statements
            LoopInner { prefix,
                        inner: &self.inner,
                        suffix }.compile(context, &mut for_statements);

            out.extend(for_statements);

            // free register holding the last index of the for loop
            context.register_alloc.free(end_register);
        }

        // restore symbols
        let _ = std::mem::replace(&mut context.symbol_alloc, parent);
    }
}

impl Compile for ast::Break<'_> {
    fn compile<B: ByteOrder>(&self, _: &mut Context<B>, out: &mut Vec<Statement>) {
        // in order to compile the Break statement, the compiler needs to know how many
        // instructions there are ahead of it. add placeholder Nop statement, which
        // should be replaced inside the compile_loop compile_for functions.
        out.push(Nop(NOP_BREAK));
    }
}

impl Compile for ast::Continue<'_> {
    fn compile<B: ByteOrder>(&self, _: &mut Context<B>, out: &mut Vec<Statement>) {
        // same deal as with the break statement.
        // use a different Nop to differentiate it.
        out.push(Nop(NOP_CONTINUE));
    }
}

impl Compile for ast::Fn<'_> {
    fn compile<B: ByteOrder>(&self, context: &mut Context<B>, _: &mut Vec<Statement>) {
        // push static symbols in the parent scope (to be restored later)
        // all symbols defined within the child scope aren't "visible" outside of it.
        let child = context.symbol_alloc.clone();
        let parent: SymbolAlloc<B> = std::mem::replace(&mut context.symbol_alloc, child);

        // this is a function so only const and static symbols are visible
        context.symbol_alloc.free_stack();

        {
            // allocate a new routine index/handle (used by the Call statement).
            // this is the index where the routine must be stored in Ir::routines.
            let _handle = context.fn_alloc.alloc(self);

            // allocate function parameters in the new stack frame.
            if let Some(args) = &self.fn_arg {
                for field in &args.inner {
                    context.symbol_alloc.alloc_stack_field(field);
                }
            }

            // like with main, start the routine with a Nop instruction
            let mut out = vec![Nop(NOP_PERSIST)];
            let return_layout = self.fn_return.as_ref().map(|r| Layout::new(&r.type_));

            context.return_ = return_layout;
            self.inner.compile(context, &mut out);
            context.return_ = None;

            out.push(Ret);

            // optimize routine statements
            super::optimize::optimize(&mut out);

            let name = Some(self.ident.to_string());
            context.routines.push(Routine { debug_name: name,
                                            statements: out });
        }

        // restore symbols
        let _ = std::mem::replace(&mut context.symbol_alloc, parent);
    }
}

impl Compile for ast::Return<'_> {
    fn compile<B: ByteOrder>(&self, context: &mut Context<B>, out: &mut Vec<Statement>) {
        if let Some(return_layout) = &context.return_ {
            super::expression::compile_expression_into_pointer::<B>(self.expression
                                                                        .as_ref()
                                                                        .unwrap(),
                                                                    return_layout,
                                                                    &context.symbol_alloc,
                                                                    &context.fn_alloc,
                                                                    Pointer::Return(0),
                                                                    &mut context.register_alloc,
                                                                    out);
        }
        out.push(Statement::Ret);
    }
}
