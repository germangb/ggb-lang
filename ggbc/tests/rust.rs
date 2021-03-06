use ggbc::target::Rust;
use std::process::{Command, Stdio};

macro_rules! test {
    ( $(#[$($meta:meta)+])* fn $fn_name:ident, $test:ident) => {
        #[test]
        $(#[$($meta)+])*
        fn $fn_name() {
            let input = include_str!(concat!("../../vm/tests/programs/", stringify!($test), ".ggb"));
            let out_rs = concat!("/tmp/", stringify!($test), ".rs");
            let out = concat!("/tmp/", stringify!($test));
            let rust = ggbc::compile::<Rust>(input).unwrap();
            std::fs::write(out_rs, rust).unwrap();
            let exit_code = Command::new("rustc")
                .args(&[out_rs, "-o", out])
                .stderr(Stdio::null())
                .spawn()
                .unwrap().wait().unwrap();
            assert!(exit_code.success());
            let exit_code = Command::new("timeout").args(&["1s", out]).spawn().unwrap().wait().unwrap();
            assert!(exit_code.success());
        }
    };
    ($fn_name:ident) => { test!(fn $fn_name, $fn_name); };
}

test!(array_assign);
test!(assign);
test!(fn bool, bool);
test!(fn break_, break);
test!(compare);
test!(fn const_, const);
test!(#[ignore] fn deref, deref);
test!(fibonacci);
test!(fibonacci_recursive);
test!(fn for_, for);
test!(function);
test!(#[ignore] fn io, io);
test!(#[ignore] fn literal, literal);
test!(fn loop_, loop);
test!(memcopy);
test!(mul);
test!(recursion);
test!(sort);
test!(#[ignore] fn string, string);
test!(fn struct_, struct);
test!(union);
test!(not_halt);
test!(#[should_panic] fn halt, halt);
