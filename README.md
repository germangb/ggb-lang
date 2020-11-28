# `ggb-lang`
[![Build Status](https://travis-ci.org/germangb/ggb-lang.svg?branch=main)](https://travis-ci.org/germangb/ggb-lang)

A toy compiler project.

## Syntax

Syntax looks a bit LISP-like at the moment. Once the IR compilation reaches a decent level of robustness and fully runs on the VM, then I will allow syntax to change into something closer to Rust or C.

## Compilation targets

There's currently two main targets planned (not-implemented-yet):

- Rust (or C, but probably Rust)
- [LR35902](https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html) (The architecture of the Game Boy)

At the current stage, there's a [Virtual Machine](vm) to runs the IR statements.
