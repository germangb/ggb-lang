use ggbc::target::Rust;
use std::process::Command;

#[rustfmt::skip]
macro_rules! test {
    (fn $fn_name:ident, $test:ident) => {
        #[test]
        fn $fn_name() {
            let input = include_str!(concat!("../../vm/tests/programs/", stringify!($test), ".ggb"));
            let out_rs = concat!("/tmp/", stringify!($test), ".rs");
            let out = concat!("/tmp/", stringify!($test));
            let rust = ggbc::compile::<Rust>(input).unwrap();
            std::fs::write(out_rs, rust).unwrap();
            let exit_code = Command::new("rustc").args(&[out_rs, "-o", out]).spawn().unwrap().wait().unwrap();
            assert!(exit_code.success());
            let exit_code = Command::new("timeout").args(&["1s", out]).spawn().unwrap().wait().unwrap();
            assert!(exit_code.success());
        }
    };
    ($fn_name:ident) => { test!(fn $fn_name, $fn_name); };
}

test!(array_assign);
test!(assign);
test!(bool);
//test!(break);
test!(compare);
//test!(const);
//test!(deref);
test!(fibonacci);
test!(fibonacci_recursive);
//test!(for);
test!(function);
//test!(io);
//test!(literal);
test!(fn loop_, loop);
test!(memcopy);
test!(mul);
test!(recursion);
test!(sort);
//test!(string);
test!(fn struct_, struct);
test!(union);
