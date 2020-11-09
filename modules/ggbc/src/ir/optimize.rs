use crate::ir::{Location, Statement, NOP_UNREACHABLE};

/// Optimize statements.
/// Returns `true` if the statements were mutated.
pub fn optimize(statements: &mut Vec<Statement>) -> bool {
    mark_unreachable(statements) || jump_threading(statements) || delete_nops(statements)
}

// delete unreachable statements, previously marked as Nop(NOP_UNREACHABLE) by
// the other functions. TODO confusing code: document or rewrite
fn delete_nops(statements: &mut Vec<Statement>) -> bool {
    use Location::Relative;
    use Statement::{Jmp, JmpCmp, JmpCmpNot, Nop};

    let mut opt = false;

    // update jump instructions by counting the number of NOPs within a jump, and
    // updates the jump accordingly. After this loop, all Jmp statements will've
    // been updated and Nops can safely be removed from the program.
    for i in 0..statements.len() {
        let r0 = match &statements[i] {
            Jmp { location: Relative(r0), } => *r0,
            JmpCmp { location: Relative(r0),
                     .. } => *r0,
            JmpCmpNot { location: Relative(r0),
                        .. } => *r0,
            _ => continue,
        };
        // range to compute the # of NOPs inside of
        let mut range = i..(i + r0.abs() as usize + 1);
        if r0 < 0 {
            range.start -= r0.abs() as usize;
            range.end -= r0.abs() as usize;
        }
        let nops = statements[range.clone()].iter()
                                            .filter(|s| matches!(s, Nop(NOP_UNREACHABLE)))
                                            .count();

        // if a NOP has been found, it will be removed and therefore this functions must
        // return true to report that the list of statements has been partially
        // optimized.
        opt |= nops > 0;

        // update how much the statement jumps by, by subtracting the # of Nops found
        // within the jump.
        let r1 = if r0 < 0 {
            r0 + nops as i8
        } else {
            r0 - nops as i8
        };
        match &mut statements[i] {
            Jmp { location: Relative(r0), } => *r0 = r1,
            JmpCmp { location: Relative(r0),
                     .. } => *r0 = r1,
            JmpCmpNot { location: Relative(r0),
                        .. } => *r0 = r1,
            _ => unreachable!(),
        };
    }

    // once all Jmps and conditional Jmps have been updated, it is safe to delete
    // the remaining unreachable Nop statements.
    // TODO less copy
    if opt {
        let opt_statements: Vec<_> = statements.iter()
                                               .cloned()
                                               .filter(|s| !matches!(s, Nop(NOP_UNREACHABLE)))
                                               .collect();
        *statements = opt_statements;
    }
    opt
}

// TODO refactor
// merge jumps when possible (a jump that lands on another jump)
fn jump_threading(statements: &mut Vec<Statement>) -> bool {
    use Location::Relative;
    use Statement::{Jmp, JmpCmp, JmpCmpNot};

    let mut opt = false;

    // FIXME borrow checker :/
    for i in 0..statements.len() {
        match statements[i].clone() {
            JmpCmp { location: Relative(r0),
                     source, } => {
                let next = ((i as isize) + (r0 as isize) + 1) as usize;
                let next_statement = statements[next].clone();
                if let Jmp { location: Relative(r1), } = next_statement {
                    statements[i] = JmpCmp { location: Relative(r0 + r1 + 1),
                                             source };
                    opt = true;
                }
            }
            JmpCmpNot { location: Relative(r0),
                        source, } => {
                let next = ((i as isize) + (r0 as isize) + 1) as usize;
                let next_statement = statements[next].clone();
                if let Jmp { location: Relative(r1), } = next_statement {
                    statements[i] = JmpCmpNot { location: Relative(r0 + r1 + 1),
                                                source };
                    opt = true;
                }
            }
            Jmp { location: Relative(r0), } => {
                let next = ((i as isize) + (r0 as isize) + 1) as usize;
                let next_statement = statements[next].clone();
                if let Jmp { location: Relative(r1), } = next_statement {
                    statements[i] = Jmp { location: Relative(r0 + r1 + 1) };
                    opt = true;
                }
            }
            _ => {}
        }
    }
    opt
}

// find unreachable statements, and replace them with a Nop so they can be
// safely deleted later by the `delete_nops` function.
fn mark_unreachable(statements: &mut Vec<Statement>) -> bool {
    use Location::Relative;
    use Statement::{Jmp, JmpCmp, JmpCmpNot, Nop, Stop};

    // DFS search on the program flow
    let mut visited = vec![false; statements.len()];
    let mut stack = vec![0];
    visited[0] = true;

    while let Some(n) = stack.pop() {
        if !matches!(statements[n], Stop) {
            let mut next = n + 1; // next statement
            let mut next_branch = n + 1; // next (branched) statement
            match statements[n] {
                Jmp { location: Relative(r), } => {
                    next_branch = ((n as isize) + (r as isize)) as usize;
                    next_branch += 1;
                    next = next_branch;
                }
                JmpCmp { location: Relative(r),
                         .. } => next_branch = ((n as isize) + (r as isize) + 1) as usize,
                JmpCmpNot { location: Relative(r),
                            .. } => next_branch = ((n as isize) + (r as isize) + 1) as usize,
                _ => {}
            }
            if !visited[next] {
                stack.push(next);
                visited[next] = true;
            }
            if !visited[next_branch] {
                stack.push(next_branch);
                visited[next_branch] = true;
            }
        }
    }
    // replace all non-visited statements with a Nop
    let mut opt = false;
    let nop = Nop(NOP_UNREACHABLE);
    statements.iter_mut()
              .zip(visited)
              .filter(|(s, seen)| !*seen && *s != &nop)
              .for_each(|(s, _)| {
                  opt = true;
                  *s = nop.clone();
              });
    opt
}
