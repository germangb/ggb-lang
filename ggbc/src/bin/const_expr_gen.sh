#!/usr/bin/env bash

cat << EOF
// generated (ggbc/src/bin/const_expr_gen.sh)
use ggbc::{byteorder::NativeEndian, ir::Ir, parser::parse};
use ggbc_vm::{Machine, Opts};

fn test_const_expr(input: &str) {
    let ast = parse(input).unwrap();
    let ir: Ir<NativeEndian> = Ir::new(&ast);
    let result = Machine::new(&ir, Opts::default()).run().static_[0];
    assert_eq!(0xff, result);
}
EOF

# generate a bunch of random tests
for test_idx in $(seq 256); do
  echo "#[test]"
  echo "fn const_expr_$test_idx() {"
  expr=$(cargo run --bin const_expr_gen)
  echo "    test_const_expr(\"$expr\");"
  echo "}"
done