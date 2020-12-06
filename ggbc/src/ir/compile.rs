use crate::{
    byteorder::ByteOrder,
    ir::{
        opcodes::{
            Destination, Location, Pointer, Source, Statement,
            Statement::{Inc, Jmp, JmpCmp, JmpCmpNot, Ld, Nop, Ret, Stop, Sub},
            StopStatus,
        },
        Routine,
    },
    parser::ast,
};
use alloc::{FnAlloc, RegisterAlloc, SymbolAlloc};
use layout::Layout;

mod alloc;
pub(crate) mod expression;
mod layout;
pub(crate) mod optimize;

// placeholder NOPs
pub(crate) const NOP_PERSIST: usize = 0;
pub(crate) const NOP_CONTINUE: usize = 1;
pub(crate) const NOP_BREAK: usize = 2;
pub(crate) const NOP_UNREACHABLE: usize = 3;

fn compile_scope<B: ByteOrder, F: FnOnce(&mut Context<B>)>(context: &mut Context<B>, fun: F) {
    // push static symbols from the parent scope (to be restored later)
    // all symbols defined within the child scope will be freed by the end.
    let child = context.symbol_alloc.clone();
    let parent: SymbolAlloc<B> = std::mem::replace(&mut context.symbol_alloc, child);
    //let parent_stack_usage = context.symbol_alloc.stack_usage();

    fun(context);

    // restore symbols
    let child_static_usage = context.symbol_alloc.static_usage();
    let child_stack_usage = context.symbol_alloc.stack_usage();
    let child_const = std::mem::replace(&mut context.symbol_alloc, parent).into_const_data();
    context.stack_size = context.stack_size.max(child_stack_usage);
    context.symbol_alloc.set_static_usage(child_static_usage);
    let _ = context.symbol_alloc.set_const(child_const);
}

/// Ir compilation context.
#[derive(Default)]
pub struct Context<B: ByteOrder> {
    pub(super) routines: Vec<Routine>,
    pub(super) symbol_alloc: SymbolAlloc<B>,
    pub(super) stack_size: u16,
    return_: Option<Layout>,
    fn_alloc: FnAlloc,
    register_alloc: RegisterAlloc,
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
                ast::Statement::IfElse(if_else) => if_else.compile(context, out),
                ast::Statement::Scope(scope) => scope.compile(context, out),
                ast::Statement::Mod(_) => todo!(),
                ast::Statement::Static(static_) => static_.compile(context, out),
                ast::Statement::Const(const_) => const_.compile(context, out),
                ast::Statement::Let(let_) => let_.compile(context, out),
                ast::Statement::For(for_) => for_.compile(context, out),
                ast::Statement::Loop(loop_) => loop_.compile(context, out),
                ast::Statement::Inline(inline) => inline.compile(context, out),
                ast::Statement::Fn(fn_) => fn_.compile(context, out),

                // everything after these statements will be unreachable by the program flow.
                // therefore, this marks the end of the loop.
                // possible way to test this later:
                // ```
                // loop {
                //     static A:u8
                //     break
                //     static B:u8 // (B should not be allocated)
                // }
                //
                // if 1 {
                //     !!
                // }
                //
                // static C:u8
                // const D:u8   // (neither C nor D should be allocated)
                // ``
                ast::Statement::Panic(panic) => {
                    panic.compile(context, out);
                    break;
                }
                ast::Statement::Continue(continue_) => {
                    continue_.compile(context, out);
                    break;
                }
                ast::Statement::Break(break_) => {
                    break_.compile(context, out);
                    break;
                }
                ast::Statement::Return(return_) => {
                    return_.compile(context, out);
                    break;
                }
            }
        }
    }
}

impl Compile for ast::Ast<'_> {
    fn compile<B: ByteOrder>(&self, context: &mut Context<B>, out: &mut Vec<Statement>) {
        out.push(Nop(NOP_PERSIST));
        self.inner.compile(context, out);
        out.push(Stop(StopStatus::Success));
        let stack_size = context.symbol_alloc.stack_usage();
        context.stack_size = context.stack_size.max(stack_size);
    }
}

impl Compile for ast::Panic<'_> {
    fn compile<B: ByteOrder>(&self, _: &mut Context<B>, out: &mut Vec<Statement>) {
        out.push(Stop(StopStatus::Error));
    }
}

impl Compile for ast::Scope<'_> {
    fn compile<B: ByteOrder>(&self, context: &mut Context<B>, out: &mut Vec<Statement>) {
        compile_scope(context, |ctx| self.inner.compile(ctx, out));
    }
}

impl Compile for ast::Static<'_> {
    fn compile<B: ByteOrder>(&self, context: &mut Context<B>, _: &mut Vec<Statement>) {
        if let Some(offset) = &self.offset {
            // static memory with explicit offset means the memory is located at the
            // absolute location in memory.
            let symbol_alloc = &context.symbol_alloc;
            let offset = expression::const_expr(&offset.expression, Some(symbol_alloc))
                .expect("Not a constant expression offset!");
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
        context
            .symbol_alloc
            .alloc_const(&self.field, &self.expression);
    }
}

impl Compile for ast::Let<'_> {
    fn compile<B: ByteOrder>(&self, context: &mut Context<B>, out: &mut Vec<Statement>) {
        // allocate memory on the stack for this field
        // the compiled expression should store the result on the stack
        let stack_address = context.symbol_alloc.alloc_stack_field(&self.field);
        let field_layout = Layout::new(&self.field.type_);
        expression::compile_expression_into_pointer(
            &self.expression,
            &field_layout,
            &context.symbol_alloc,
            &context.fn_alloc,
            Pointer::Stack(stack_address),
            &mut context.register_alloc,
            out,
        );
    }
}

impl Compile for ast::Inline<'_> {
    fn compile<B: ByteOrder>(&self, context: &mut Context<B>, out: &mut Vec<Statement>) {
        // compile expression and drop the results.
        // the expression will be evaluated by the result is not stored anywhere.
        expression::compile_expr_void(
            &self.inner,
            &context.symbol_alloc,
            &context.fn_alloc,
            &mut context.register_alloc,
            out,
        )
    }
}

struct IfStatements<'a, 'b> {
    expression: &'a ast::Expression<'b>,
    inner: &'a Vec<ast::Statement<'b>>,
    has_else: bool,
}

impl Compile for ast::If<'_> {
    fn compile<B: ByteOrder>(&self, context: &mut Context<B>, out: &mut Vec<Statement>) {
        let const_expr = expression::const_expr(&self.expression, Some(&context.symbol_alloc));

        match const_expr {
            Some(0) => {}
            Some(_) => compile_scope(context, |ctx| self.inner.compile(ctx, out)),
            None => compile_scope(context, |ctx| {
                IfStatements {
                    expression: &self.expression,
                    inner: &self.inner,
                    has_else: false,
                }
                .compile(ctx, out)
            }),
        }
    }
}

impl Compile for ast::IfElse<'_> {
    fn compile<B: ByteOrder>(&self, context: &mut Context<B>, out: &mut Vec<Statement>) {
        let const_expr = expression::const_expr(&self.if_.expression, Some(&context.symbol_alloc));

        match const_expr {
            Some(0) => compile_scope(context, |ctx| self.else_.inner.compile(ctx, out)),
            Some(_) => compile_scope(context, |ctx| self.if_.inner.compile(ctx, out)),
            None => {
                // compiled else_ block
                let mut else_ = Vec::new();

                compile_scope(context, |ctx| self.else_.inner.compile(ctx, &mut else_));
                compile_scope(context, |ctx| {
                    IfStatements {
                        expression: &self.if_.expression,
                        inner: &self.if_.inner,
                        has_else: true,
                    }
                    .compile(ctx, out)
                });

                out.push(Jmp {
                    location: Location::Relative(else_.len() as _),
                });
                out.extend(else_);
            }
        }
    }
}

impl Compile for IfStatements<'_, '_> {
    fn compile<B: ByteOrder>(&self, context: &mut Context<B>, out: &mut Vec<Statement>) {
        // compile expression into an 8bit register
        let source = expression::compile_expr_u8(
            &self.expression,
            &context.symbol_alloc,
            &context.fn_alloc,
            &mut context.register_alloc,
            out,
        );
        expression::free_source_registers(&source, &mut context.register_alloc);

        // compile the block of statements inside the if block.
        // clone the symbol_alloc to free any symbols defined within the block.
        let mut inner = Vec::new();
        self.inner.compile(context, &mut inner);

        let jmp = inner.len() + if self.has_else { 1 } else { 0 };
        out.push(JmpCmpNot {
            location: Location::Relative(jmp as _),
            source,
        });
        out.extend(inner);
    }
}

// Generalized loop statement.
// Equivalent to the following loop statement:
// ```no_rust
// loop {
//     <prefix>
//     <inner>
//     <suffix>
// }
// ```
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
        inner.push(Jmp {
            location: Location::Relative(-(loop_statements_signed + 1) as i8),
        });

        // replace Nop statements (placeholders for break and continue) with the
        // appropriate Jmp statements. Don't worry about optimizing jumps for now. That
        // will be dealt with later...
        let statements_len = inner.len();
        for (i, statement) in inner.iter_mut().enumerate() {
            match statement {
                // break
                Nop(NOP_BREAK) => {
                    let relative = statements_len - i - 1;
                    *statement = Jmp {
                        location: Location::Relative(relative as _),
                    };
                }
                // continue
                Nop(NOP_CONTINUE) => {
                    let relative = i as isize + 1;
                    *statement = Jmp {
                        location: Location::Relative(-relative as _),
                    };
                }
                _ => {}
            }
        }
        out.extend(inner);
    }
}

impl Compile for ast::Loop<'_> {
    fn compile<B: ByteOrder>(&self, context: &mut Context<B>, out: &mut Vec<Statement>) {
        compile_scope(context, |context| {
            LoopInner {
                prefix: Vec::new(),
                inner: &self.inner,
                suffix: Vec::new(),
            }
            .compile(context, out)
        })
    }
}

impl Compile for ast::For<'_> {
    fn compile<B: ByteOrder>(&self, context: &mut Context<B>, out: &mut Vec<Statement>) {
        compile_scope(context, |context| {
            let mut for_statements = Vec::new();
            let stack_address = context.symbol_alloc.alloc_stack_field(&self.field);

            // init for variable with the lhs side of the range
            // TODO non-U8 variables
            let init = expression::compile_expr_u8(
                &self.range.left,
                &context.symbol_alloc,
                &context.fn_alloc,
                &mut context.register_alloc,
                out,
            );
            expression::free_source_registers(&init, &mut context.register_alloc);
            out.push(Ld {
                source: init,
                destination: Destination::Pointer {
                    base: Pointer::Stack(stack_address),
                    offset: None,
                },
            });

            // if the for loop only performs a single iteration (an this can be determined
            // statically), compile as a regular block statement.
            // TODO handle cases where no iterations are performed:
            //  for i:u8 in n..n
            //  for i:u8 in n..+0
            let l = expression::const_expr(&self.range.left, Some(&context.symbol_alloc));
            let r = expression::const_expr(&self.range.right, Some(&context.symbol_alloc));
            match (l, r, &self.range.eq, &self.range.plus) {
                // for _ in n..=n
                (Some(l), Some(r), Some(_), None) if l == r => {
                    self.inner.compile(context, out);
                    return;
                }
                // for _ in n..(n+1)
                (Some(l), Some(r), None, None) if l + 1 == r => {
                    self.inner.compile(context, out);
                    return;
                }
                // for _ in n..=+0
                (Some(_), Some(0), Some(_), Some(_)) => {
                    self.inner.compile(context, out);
                    return;
                }
                _ => {}
            }

            // compute end index of the for loop with the rhs of the range
            // increment if it's an inclusive range
            let end = expression::compile_expr_u8(
                &self.range.right,
                &context.symbol_alloc,
                &context.fn_alloc,
                &mut context.register_alloc,
                out,
            );
            let end_register = context.register_alloc.alloc();
            out.push(Statement::Ld {
                source: end.clone(),
                destination: Destination::Register(end_register),
            });
            expression::free_source_registers(&end, &mut context.register_alloc);
            if self.range.eq.is_some() {
                out.push(Inc {
                    source: Source::Register(end_register),
                    destination: Destination::Register(end_register),
                });
            }

            // begin compiling the inner for loop statements.
            // check if for loop variable has reached the limit.
            let cmp_register = context.register_alloc.alloc();
            let prefix = vec![
                Sub {
                    left: Source::Register(end_register),
                    right: Source::Pointer {
                        base: Pointer::Stack(stack_address),
                        offset: None,
                    },
                    destination: Destination::Register(cmp_register),
                },
                // TODO optimize away, as this is equivalent to: if foo { break }
                JmpCmp {
                    location: Location::Relative(1),
                    source: Source::Register(cmp_register),
                },
                Nop(NOP_BREAK),
            ];
            context.register_alloc.free(cmp_register);
            // increment the for loop variable
            let suffix = vec![Inc {
                source: Source::Pointer {
                    base: Pointer::Stack(stack_address),
                    offset: None,
                },
                destination: Destination::Pointer {
                    base: Pointer::Stack(stack_address),
                    offset: None,
                },
            }];

            // parse inner loop statements
            LoopInner {
                prefix,
                inner: &self.inner,
                suffix,
            }
            .compile(context, &mut for_statements);

            out.extend(for_statements);

            // free register holding the last index of the for loop
            context.register_alloc.free(end_register);
        });
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
        compile_scope(context, |context| {
            // this is a function so only const and static symbols are visible
            context.symbol_alloc.free_stack();

            // allocate a new routine index/handle (used by the Call statement).
            // this is the index where the routine must be stored in Ir::routines.
            let _handle = context.fn_alloc.alloc(self);

            // allocate function parameters in the new stack frame.
            if let Some(args) = &self.fn_arg {
                for field in &args.inner {
                    context.symbol_alloc.alloc_stack_field(field);
                }
            }

            let args_size = context.symbol_alloc.stack_usage();
            context.stack_size = args_size;

            // like with main, start the routine with a Nop instruction
            let mut out = vec![Nop(NOP_PERSIST)];
            let return_layout = self.fn_return.as_ref().map(|r| Layout::new(&r.type_));

            let return_size = return_layout.as_ref().map(|l| l.size()).unwrap_or(0);

            context.return_ = return_layout;
            self.inner.compile(context, &mut out);
            context.return_ = None;

            out.push(Ret);

            let name = Some(self.ident.to_string());
            context.routines.push(Routine {
                debug_name: name,
                stack_size: context.stack_size,
                args_size,
                return_size,
                statements: out,
            });
        });
    }
}

impl Compile for ast::Return<'_> {
    fn compile<B: ByteOrder>(&self, context: &mut Context<B>, out: &mut Vec<Statement>) {
        if let Some(return_layout) = &context.return_ {
            expression::compile_expression_into_pointer::<B>(
                self.expression.as_ref().unwrap(),
                return_layout,
                &context.symbol_alloc,
                &context.fn_alloc,
                Pointer::Return(0),
                &mut context.register_alloc,
                out,
            );
        }
        out.push(Statement::Ret);
    }
}

#[cfg(test)]
mod test {
    use crate::ir::{
        compile::{Compile, Context},
        opcodes::Statement,
    };

    #[test]
    fn test_if_const_expr() {
        let ast = crate::parser::parse(
            r#"
        const F:u8 = 0
        static RESULT1:[u8 13]
        static RESULT2:[u8 13]
        if F {
            // this code will be removed!
            (= ([0]RESULT1) 1)
            (= ([1]RESULT1) 1)
            (= ([0]RESULT2) 1)
            (= ([1]RESULT2) 1)
            for offset:u8 in 2..13 {
                (= ([offset]RESULT1)
                   (+ ([(-offset 1)]RESULT1) ([(-offset 2)]RESULT1)))
                (+= ([offset]RESULT2)
                    ([(-offset 1)]RESULT2))
                (+= ([offset]RESULT2)
                    ([(-offset 2)]RESULT2))
            }
        }
        if 1 {
        } else {
            // this code will be removed!
            (= ([0]RESULT1) 1)
            (= ([1]RESULT1) 1)
            (= ([0]RESULT2) 1)
            (= ([1]RESULT2) 1)
            for offset:u8 in 2..13 {
                (= ([offset]RESULT1)
                    (+ ([(-offset 1)]RESULT1)
                       ([(-offset 2)]RESULT1)))
                (+= ([offset]RESULT2)
                    ([(-offset 1)]RESULT2))
                (+= ([offset]RESULT2)
                    ([(-offset 2)]RESULT2))
            }
        }
        "#,
        )
        .unwrap();
        let mut context = Context::<crate::byteorder::NativeEndian>::default();
        let mut statements = Vec::new();
        ast.inner.compile(&mut context, &mut statements);
        let gt: Vec<Statement> = vec![];
        assert_eq!(gt, statements); // no code must be generated
    }
}
