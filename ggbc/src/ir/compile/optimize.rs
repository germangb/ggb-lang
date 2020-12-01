use crate::ir::opcodes::{Location, Source, Statement, NOP_UNREACHABLE};

/// Optimize Ir statements.
pub fn optimize(statements: &mut Vec<Statement>) {
    while mark_unreachable(statements) || jump_threading(statements) || delete_nops(statements) {}
}

// delete unreachable statements, previously marked as Nop(NOP_UNREACHABLE) by
// the other functions. TODO confusing code: document or rewrite
fn delete_nops(statements: &mut Vec<Statement>) -> bool {
    use Statement::{Jmp, JmpCmp, JmpCmpNot, Nop};

    // update jump instructions by counting the number of NOPs within a jump, and
    // updates the jump accordingly. After this loop, all Jmp statements will have
    // been updated and Nops can safely be removed from the ir.
    for i in 0..statements.len() {
        #[rustfmt::skip]
        let r0 = match &statements[i] {
            Jmp       { location: Location::Relative(r0)     } => *r0,
            JmpCmp    { location: Location::Relative(r0), .. } => *r0,
            JmpCmpNot { location: Location::Relative(r0), .. } => *r0,
            _ => continue,
        };
        // range to compute the # of NOPs inside of
        let mut range = i..(i + r0.abs() as usize + 1);
        if r0 < 0 {
            range.start -= r0.abs() as usize;
            range.end -= r0.abs() as usize;
        }
        let nops = statements[range.clone()]
            .iter()
            .filter(|s| matches!(s, Nop(NOP_UNREACHABLE)))
            .count();

        // update how much the statement jumps by, by subtracting the # of Nops found
        // within the jump.
        let r1 = if r0 < 0 {
            let mut t = r0 + nops as i8;
            if &statements[(i as isize + r0 as isize) as usize] == &Statement::Nop(NOP_UNREACHABLE)
            {
                t -= 1
            }
            t
        } else {
            r0 - nops as i8
        };

        #[rustfmt::skip]
        match &mut statements[i] {
            Jmp       { location: Location::Relative(r0)     } => *r0 = r1,
            JmpCmp    { location: Location::Relative(r0), .. } => *r0 = r1,
            JmpCmpNot { location: Location::Relative(r0), .. } => *r0 = r1,
            _ => unreachable!(),
        }; // rustfmt::skip woks on expressions but not statements (adding ;
           // turns match into the former)
    }

    // previous # of statements
    let len = statements.len();
    // once all Jmps and conditional Jmps have been updated, it is safe to delete
    // the remaining unreachable Nop statements.
    let opt_statements: Vec<_> = statements
        .iter()
        .cloned()
        .filter(|s| !matches!(s, Nop(NOP_UNREACHABLE)))
        .collect();
    *statements = opt_statements;
    len != statements.len()
}

// merge jumps when possible (a jump that lands on another jump)
fn jump_threading(statements: &mut Vec<Statement>) -> bool {
    use Statement::{Jmp, JmpCmp, JmpCmpNot};

    // clone statements in order to be able to handle loops
    // see test below
    let mut statements_opt = statements.clone();

    for i in 0..statements.len() {
        #[rustfmt::skip]
        match &statements[i] {
            // jump that lands on itself
            // usually happens when compiling an empty loop: loop {}
            Jmp       { location: Location::Relative(-1),    } |
            JmpCmp    { location: Location::Relative(-1), .. } |
            JmpCmpNot { location: Location::Relative(-1), .. } => {}

            JmpCmp { location: Location::Relative(r0), source } => {
                let next = ((i as isize) + (*r0 as isize) + 1) as usize;
                if let Jmp { location: Location::Relative(r1), } = &statements[next] {
                    statements_opt[i] = JmpCmp { location: Location::Relative(*r0 + *r1 + 1),
                                                 source: source.clone() };
                }
            }
            JmpCmpNot { location: Location::Relative(r0), source } => {
                let next = ((i as isize) + (*r0 as isize) + 1) as usize;
                if let Jmp { location: Location::Relative(r1), } = &statements[next] {
                    statements_opt[i] = JmpCmpNot { location: Location::Relative(*r0 + *r1 + 1),
                                                    source: source.clone() };
                }
            }
            Jmp { location: Location::Relative(r0) } => {
                let next = ((i as isize) + (*r0 as isize) + 1) as usize;
                if let Jmp { location: Location::Relative(r1), } = &statements[next]{
                    statements_opt[i] = Jmp { location: Location::Relative(*r0 + *r1 + 1) };
                }
            }
            _ => {}
        };
    }
    let opt = statements != &statements_opt;
    *statements = statements_opt;
    opt
}

// find unreachable statements, and replace them with a Nop so they can be
// safely deleted later by the `delete_nops` function.
fn mark_unreachable(statements: &mut Vec<Statement>) -> bool {
    use Statement::{Jmp, JmpCmp, JmpCmpNot, Nop, Ret, Stop};

    // DFS search on the program flow
    let mut visited = vec![false; statements.len()];
    let mut stack = vec![0];
    visited[0] = true;

    while let Some(n) = stack.pop() {
        if !matches!(statements[n], Stop(_) | Ret) {
            let mut next = n + 1; // next statement
            let mut next_branch = n + 1; // next (branched) statement
            #[rustfmt::skip]
            match statements[n] {
                JmpCmpNot { location: Location::Relative(r), source: Source::Literal(n) } if n != 0 => {
                    next_branch = (n as isize + r as isize + 1) as usize;
                    next = next_branch;
                }
                Jmp    { location: Location::Relative(r), .. } |
                JmpCmp { location: Location::Relative(r), source: Source::Literal(0) } => {
                    next_branch = (n as isize + r as isize + 1) as usize;
                    next = next_branch;
                }
                JmpCmp    { location: Location::Relative(r), .. } =>
                    next_branch = (n as isize + r as isize + 1) as usize,
                JmpCmpNot { location: Location::Relative(r), .. } =>
                    next_branch = (n as isize + r as isize + 1) as usize,
                _ => {}
            };
            if let Some(false) = visited.get(next) {
                stack.push(next);
                visited[next] = true;
            }
            if let Some(false) = visited.get(next_branch) {
                stack.push(next_branch);
                visited[next_branch] = true;
            }
        }
    }
    // replace all non-visited statements with a Nop
    let mut opt = false;
    statements
        .iter_mut()
        .zip(visited)
        .filter(|(s, visited)| !*visited && !matches!(s, Nop(NOP_UNREACHABLE)))
        .for_each(|(s, _)| {
            opt = true;
            *s = Nop(NOP_UNREACHABLE)
        });
    opt
}

#[cfg(test)]
mod test {
    use crate::ir::{
        compile::optimize::optimize,
        opcodes::{Location, Source, Statement, StopStatus, NOP_UNREACHABLE},
    };

    #[test]
    fn mark_unreachable_cmp_not_constexpr() {
        let mut statements = vec![
            Statement::Nop(0),
            Statement::JmpCmpNot {
                location: Location::Relative(2),
                source: Source::Literal(1),
            },
            Statement::Nop(0),
            Statement::Nop(0),
            Statement::Nop(0),
        ];

        super::mark_unreachable(&mut statements);

        let gt = vec![
            Statement::Nop(0),
            Statement::JmpCmpNot {
                location: Location::Relative(2),
                source: Source::Literal(1),
            },
            Statement::Nop(NOP_UNREACHABLE),
            Statement::Nop(NOP_UNREACHABLE),
            Statement::Nop(0),
        ];

        assert_eq!(gt, statements);
    }

    #[test]
    fn mark_unreachable_cmp_constexpr() {
        let mut statements = vec![
            Statement::Nop(0),
            Statement::JmpCmp {
                location: Location::Relative(2),
                source: Source::Literal(0),
            },
            Statement::Nop(0),
            Statement::Nop(0),
            Statement::Nop(0),
        ];

        super::mark_unreachable(&mut statements);

        let gt = vec![
            Statement::Nop(0),
            Statement::JmpCmp {
                location: Location::Relative(2),
                source: Source::Literal(0),
            },
            Statement::Nop(NOP_UNREACHABLE),
            Statement::Nop(NOP_UNREACHABLE),
            Statement::Nop(0),
        ];

        assert_eq!(gt, statements);
    }

    #[test]
    fn mark_unreachable_cmp() {
        let mut statements = vec![
            Statement::Nop(0),
            Statement::JmpCmp {
                location: Location::Relative(2),
                source: Source::Register(0),
            },
            Statement::Nop(0),
            Statement::Nop(0),
            Statement::Nop(0),
        ];

        super::mark_unreachable(&mut statements);

        let gt = vec![
            Statement::Nop(0),
            Statement::JmpCmp {
                location: Location::Relative(2),
                source: Source::Register(0),
            },
            Statement::Nop(0),
            Statement::Nop(0),
            Statement::Nop(0),
        ];

        assert_eq!(gt, statements);
    }

    #[test]
    fn mark_unreachable_cmp_not() {
        let mut statements = vec![
            Statement::Nop(0),
            Statement::JmpCmpNot {
                location: Location::Relative(2),
                source: Source::Register(0),
            },
            Statement::Nop(0),
            Statement::Nop(0),
            Statement::Nop(0),
        ];

        super::mark_unreachable(&mut statements);

        let gt = vec![
            Statement::Nop(0),
            Statement::JmpCmpNot {
                location: Location::Relative(2),
                source: Source::Register(0),
            },
            Statement::Nop(0),
            Statement::Nop(0),
            Statement::Nop(0),
        ];

        assert_eq!(gt, statements);
    }

    #[test]
    fn mark_unreachable_jmp() {
        let mut statements = vec![
            Statement::Nop(0),
            Statement::Jmp {
                location: Location::Relative(2),
            },
            Statement::Nop(0),
            Statement::Nop(0),
            Statement::Nop(0),
        ];

        super::mark_unreachable(&mut statements);

        let gt = vec![
            Statement::Nop(0),
            Statement::Jmp {
                location: Location::Relative(2),
            },
            Statement::Nop(NOP_UNREACHABLE),
            Statement::Nop(NOP_UNREACHABLE),
            Statement::Nop(0),
        ];

        assert_eq!(gt, statements);
    }

    #[test]
    fn mark_unreachable_jmp_loop() {
        let mut statements = vec![
            Statement::Nop(0),
            Statement::Jmp {
                location: Location::Relative(-1),
            },
            Statement::Nop(0),
            Statement::Nop(0),
        ];

        super::mark_unreachable(&mut statements);

        let gt = vec![
            Statement::Nop(0),
            Statement::Jmp {
                location: Location::Relative(-1),
            },
            Statement::Nop(NOP_UNREACHABLE),
            Statement::Nop(NOP_UNREACHABLE),
        ];

        assert_eq!(gt, statements);
    }

    #[test]
    fn remove_nops_land_on_persist_nop() {
        // Nop     => Nop
        // Jmp(1)  => Jmp(0)
        // Nop'    => Nop
        // Nop     => Jmp(-2)
        // Nop'    =>
        // Jmp(-3) =>
        let mut statements = vec![
            Statement::Nop(0),
            Statement::Jmp {
                location: Location::Relative(1),
            },
            Statement::Nop(NOP_UNREACHABLE),
            Statement::Nop(0),
            Statement::Nop(NOP_UNREACHABLE),
            Statement::Jmp {
                location: Location::Relative(-3),
            },
        ];

        super::delete_nops(&mut statements);

        let gt = vec![
            Statement::Nop(0),
            Statement::Jmp {
                location: Location::Relative(0),
            },
            Statement::Nop(0),
            Statement::Jmp {
                location: Location::Relative(-2),
            },
        ];

        assert_eq!(gt, statements);
    }

    #[test]
    fn remove_nops_jump_over_nops() {
        // Nop     => Nop
        // Jmp(5)  => Jmp(2)
        // Jmp(3)  => Jmp(0)
        // Nop'    => Jmp(-2)
        // Nop'    => Jmp(-4)
        // Nop'    =>
        // Jmp(-5) =>
        // Jmp(-7) =>
        let mut statements = vec![
            Statement::Nop(0),
            Statement::Jmp {
                location: Location::Relative(5),
            },
            Statement::Jmp {
                location: Location::Relative(3),
            },
            Statement::Nop(NOP_UNREACHABLE),
            Statement::Nop(NOP_UNREACHABLE),
            Statement::Nop(NOP_UNREACHABLE),
            Statement::Jmp {
                location: Location::Relative(-5),
            },
            Statement::Jmp {
                location: Location::Relative(-7),
            },
        ];

        super::delete_nops(&mut statements);

        let gt = vec![
            Statement::Nop(0),
            Statement::Jmp {
                location: Location::Relative(2),
            },
            Statement::Jmp {
                location: Location::Relative(0),
            },
            Statement::Jmp {
                location: Location::Relative(-2),
            },
            Statement::Jmp {
                location: Location::Relative(-4),
            },
        ];

        assert_eq!(gt, statements);
    }

    #[test]
    fn remove_nops_jump_over_nops_and_land_on_nops() {
        // Nop     => Nop
        // Jmp(4)  => Jmp(2) // jump forward a Nop to be removed
        // Jmp(5)  => Jmp(2)
        // Jmp(3)  => Jmp(0)
        // Nop'    => Jmp(-2)
        // Nop'    => Jmp(-2)
        // Nop'    => Jmp(-3)
        // Jmp(-5) => Jmp(-6)
        // Jmp(-3) => // jump backwards to a Nop to be removed
        // Jmp(-5) => // jump backwards to a Nop to be removed
        // Jmp(-9) => // jump backwards to another jump
        let mut statements = vec![
            Statement::Nop(0),
            Statement::Jmp {
                location: Location::Relative(4),
            },
            Statement::Jmp {
                location: Location::Relative(5),
            },
            Statement::Jmp {
                location: Location::Relative(3),
            },
            Statement::Nop(NOP_UNREACHABLE),
            Statement::Nop(NOP_UNREACHABLE),
            Statement::Nop(NOP_UNREACHABLE),
            Statement::Jmp {
                location: Location::Relative(-5),
            },
            Statement::Jmp {
                location: Location::Relative(-3),
            },
            Statement::Jmp {
                location: Location::Relative(-5),
            },
            Statement::Jmp {
                location: Location::Relative(-9),
            },
        ];

        super::delete_nops(&mut statements);

        let gt = vec![
            Statement::Nop(0),
            Statement::Jmp {
                location: Location::Relative(2),
            },
            Statement::Jmp {
                location: Location::Relative(2),
            },
            Statement::Jmp {
                location: Location::Relative(0),
            },
            Statement::Jmp {
                location: Location::Relative(-2),
            },
            Statement::Jmp {
                location: Location::Relative(-2),
            },
            Statement::Jmp {
                location: Location::Relative(-3),
            },
            Statement::Jmp {
                location: Location::Relative(-6),
            },
        ];

        assert_eq!(gt, statements);
    }

    #[test]
    fn jump_threading_self() {
        let mut statements = vec![
            Statement::Nop(0),
            Statement::Jmp {
                location: Location::Relative(-1),
            },
        ];
        let gt = statements.clone();
        super::jump_threading(&mut statements);
        assert_eq!(gt, statements);
    }

    #[test]
    fn jump_threading_loop() {
        // Nop     => Nop
        // Jmp(1)  => Jmp(-1)
        // Nop     => Nop
        // Jmp(-3) => Jmp(-1)
        let mut statements = vec![
            Statement::Nop(0),
            Statement::Jmp {
                location: Location::Relative(1),
            },
            Statement::Nop(0),
            Statement::Jmp {
                location: Location::Relative(-3),
            },
        ];

        super::jump_threading(&mut statements);

        let gt = vec![
            Statement::Nop(0),
            Statement::Jmp {
                location: Location::Relative(-1),
            },
            Statement::Nop(0),
            Statement::Jmp {
                location: Location::Relative(-1),
            },
        ];

        assert_eq!(gt, statements);
    }

    #[test]
    fn jump_threading_double_thread() {
        let mut statements = vec![
            Statement::Nop(0),
            Statement::Jmp {
                location: Location::Relative(1),
            },
            Statement::Nop(0),
            Statement::Jmp {
                location: Location::Relative(1),
            },
            Statement::Nop(0),
            Statement::Jmp {
                location: Location::Relative(0),
            },
            Statement::Nop(0),
        ];

        super::jump_threading(&mut statements);
        super::jump_threading(&mut statements);

        let gt = vec![
            Statement::Nop(0),
            Statement::Jmp {
                location: Location::Relative(4),
            },
            Statement::Nop(0),
            Statement::Jmp {
                location: Location::Relative(2),
            },
            Statement::Nop(0),
            Statement::Jmp {
                location: Location::Relative(0),
            },
            Statement::Nop(0),
        ];

        assert_eq!(gt, statements);
    }
}
