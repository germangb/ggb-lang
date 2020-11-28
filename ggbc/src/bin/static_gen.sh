#!/usr/bin/env bash

cat << EOF
// generated (ggbc/src/bin/const_expr_gen.sh)
use ggbc::{byteorder::NativeEndian, ir::Ir, parser::parse};
use ggbc_vm::{Machine, Opts};

fn test_static(input: &str) {
    let ast = parse(input).unwrap();
    let ir: Ir<NativeEndian> = Ir::new(&ast);
    let result = Machine::new(&ir, Opts::default()).run().static_[0];
    assert_eq!(0xff, result);
}
EOF
