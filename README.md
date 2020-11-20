# `ggb-lang`
[![Build Status](https://travis-ci.org/germangb/ggb-lang.svg?branch=main)](https://travis-ci.org/germangb/ggb-lang)

A toy compiler project.

## Syntax

Syntax looks a bit LISP-like at the moment. Once the IR compilation reaches a decent level of robustness and fully runs on the VM, then I will allow syntax to change into something closer to Rust or C.

## Compilation targets

There's currently two main targets planned (not-implemented-yet):

- Rust (or C, but probably Rust)
- [LR35902](https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html) (The architecture of the Game Boy)

At the current stage, there's a [VM](modules/ggbc-vm) to runs the IR statements.

## Example

See the examples under [`modules/ggbc-vm/tests/programs/`](modules/ggbc-vm/tests/programs/).

This is a recursive example which can be compiled and run with the following command:

```bash
$ cargo run --example fibonacci_recursive
```
```rust
fn fibonacci(n:u8):u8 {
    // values overflow an u8 at this point
    if (> n 13) {
        !! // panic
    }

    // memoization cache array
    // holds more items than this function can handle
    static CACHE:[u8 256]
    
    // handle base case
    if (| (==n 0) (==n 1)) {
        return 1
    }

    if (== ([n]CACHE) 0) {
        let n1:u8 = (fibonacci (-n 1))
        let n2:u8 = (fibonacci (-n 2))
        (= ([n]CACHE) (+n1 n2))
    }

    return ([n]CACHE)
}
```
```
0000 | NOP 0
0001 | GREATER %0 ← (stack@0000) #13
0002 | JMPCMPNOT 1 %0
0003 | STOP
0004 | EQ %0 ← (stack@0000) #0
0005 | EQ %1 ← (stack@0000) #1
0006 | OR %0 ← %0 %1
0007 | JMPCMPNOT 2 %0
0008 | LD (return@0000) ← #1
0009 | RET
000a | EQ %0 ← (static@0004+(stack@0000)) #0
000b | JMPCMPNOT 8 %0
000c | SUB (stack@0001) ← (stack@0000) #1
000d | CALL 2 0001..0002
000e | LD (stack@0001) ← (return@0000)
000f | SUB (stack@0002) ← (stack@0000) #2
0010 | CALL 2 0002..0003
0011 | LD (stack@0002) ← (return@0000)
0012 | ADD %0 ← (stack@0001) (stack@0002)
0013 | LD (static@0004+(stack@0000)) ← %0
0014 | LD (return@0000) ← (static@0004+(stack@0000))
0015 | RET
```
