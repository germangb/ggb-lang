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

See [`modules/ggbc-vm/tests/programs/`](modules/ggbc-vm/tests/programs/) for a list of programs.

```bash
$ cargo run --example fibonacci_recursive
```
**Source**
```rust
fn fibonacci(n:u8):u8 {
    // values that overflow an u8
    if (> n 13) {
        !! // panic
    }

    // memoization cache array, located at a fixed offset in memory (0x100)
    // holds more items than this function can handle
    static@0x100 CACHE:[u8 256]

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

// memory to store the result
static RESULT:[u8 13]

for i:u8 in 0..13 {
    let t:u8 = (fibonacci i)
    (= ([i]RESULT) t)
}
```
**Intermediate**
```
     | main#1:
0000 |   NOP 0
0001 |   LD (stack@0000) ← #0
0002 |   LD %0 ← #13
0003 |   SUB %1 ← %0 (stack@0000)
0004 |   JMPCMP 1 %1
0005 |   JMP 6
0006 |   LD (stack@0001) ← (stack@0000)
0007 |   CALL 0 0001..0002
0008 |   LD (stack@0001) ← (return@0000)
0009 |   LD (static@0000+(stack@0000)) ← (stack@0001)
000a |   INC (stack@0000) ← (stack@0000)
000b |   JMP -9
000c |   STOP
     | fibonacci#0:
0000 |   NOP 0
0001 |   GREATER %0 ← (stack@0000) #13
0002 |   JMPCMPNOT 1 %0
0003 |   STOP
0004 |   EQ %0 ← (stack@0000) #0
0005 |   EQ %1 ← (stack@0000) #1
0006 |   OR %0 ← %0 %1
0007 |   JMPCMPNOT 2 %0
0008 |   LD (return@0000) ← #1
0009 |   RET
000a |   EQ %0 ← (absolute@0100+(stack@0000)) #0
000b |   JMPCMPNOT 8 %0
000c |   SUB (stack@0001) ← (stack@0000) #1
000d |   CALL 0 0001..0002
000e |   LD (stack@0001) ← (return@0000)
000f |   SUB (stack@0002) ← (stack@0000) #2
0010 |   CALL 0 0002..0003
0011 |   LD (stack@0002) ← (return@0000)
0012 |   ADD %0 ← (stack@0001) (stack@0002)
0013 |   LD (absolute@0100+(stack@0000)) ← %0
0014 |   LD (return@0000) ← (absolute@0100+(stack@0000))
0015 |   RET
```
**Result**
```
0000 | 01 (1)
0001 | 01 (1)
0002 | 02 (2)
0003 | 03 (3)
0004 | 05 (5)
0005 | 08 (8)
0006 | 0d (13)
0007 | 15 (21)
0008 | 22 (34)
0009 | 37 (55)
000a | 59 (89)
000b | 90 (144)
000c | e9 (233)
```