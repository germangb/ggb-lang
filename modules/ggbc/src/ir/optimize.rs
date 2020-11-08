use crate::ir::{Location, Statement};

pub fn optimize(statements: &mut Vec<Statement>) -> bool {
    trailing(statements) || mark_unreachable(statements) || thread_jmp(statements)
}

// merge jumps when possible (a jump that lands on another jump)
fn thread_jmp(_statements: &mut Vec<Statement>) -> bool {
    false
}

// determines what statements are superfluous following the current flow of the
// program.
fn mark_unreachable(statements: &mut Vec<Statement>) -> bool {
    use Location::Relative;
    use Statement::{Jmp, JmpCmp, JmpCmpNot};

    // DFS search on the program flow
    let mut visited = vec![false; statements.len()];
    let mut stack = vec![0];
    visited[0] = true;
    let mut seen = 0;
    while let Some(n) = stack.pop() {
        seen += 1;
        if statements[n] != Statement::Stop {
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
    // replace all non-seen statements with a Nop
    let nop = Statement::Nop(42);
    let mut opt = 0;
    statements.iter_mut()
              .zip(visited)
              .filter(|(s, seen)| !*seen && *s != &nop)
              .for_each(|(s, _)| {
                  opt += 1;
                  *s = nop.clone();
              });
    opt != 0
}

// Removes superfluous trailing instructions.
// Removes all statements after a `Stop` statement.
fn trailing(statements: &mut Vec<Statement>) -> bool {
    use Statement::Stop;
    match statements.iter().enumerate().find(|(_, s)| *s == &Stop) {
        // no STOP found, so do nothing
        None => false,
        // single Stop found, but it's the last statement
        // again, do nothing
        Some((i, _)) if i + 1 == statements.len() => false,
        // remove trailing statements
        Some((i, _)) => {
            // this is safe because i is less than the current len
            unsafe { statements.set_len(i + 1) };
            true
        }
    }
}
