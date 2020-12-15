use ggbc::{byteorder::NativeEndian, ir::Ir, parser::parse};
use vm::{Machine, Opts};

fn _test_static(input: &str, gt: &[u8]) {
    let ast = parse(input).unwrap();
    let ir: Ir<NativeEndian> = Ir::new(&ast);
    let result = &Machine::new(&ir, Opts::default()).run().static_[..gt.len()];
    assert_eq!(gt, result);
}

#[test]
fn test_static() {
    _test_static(
        r#"
    static s0:u8 (= s0 0)
    static s1:u8 (= s1 1)
    static s2:u8 (= s2 2)
    {
        static s3:u8 (= s3 3)
        static s4:u8 (= s4 4)
        static s5:u8 (= s5 5)
        {
            static s6:u8 (= s6 6)
            static s7:u8 (= s7 7)
            {
                static s8:u8 (= s8 8)
            }
            static s9:u8 (= s9 9)
        }
        static s10:u8 (= s10 10)
    }
    static s11:u8 (= s11 11)
    {
        static s12:u8 (= s12 12)
        static s13:u8 (= s13 13)
        {
            static s14:u8 (= s14 14)
        }
        static s15:u8 (= s15 15)
    }
    "#,
        (0..=15).collect::<Vec<_>>().as_ref(),
    )
}

#[test]
fn test_static_fn() {
    _test_static(
        r#"
    static s0:u8 (= s0 0)
    static s1:u8 (= s1 1)
    static s2:u8 (= s2 2)
    fn foo {
        static s3:u8 (= s3 3)
        static s4:u8 (= s4 4)
        static s5:u8 (= s5 5)
        {
            static s6:u8 (= s6 6)
            static s7:u8 (= s7 7)
            {
                static s8:u8 (= s8 8)
            }
            static s9:u8 (= s9 9)
        }
        static s10:u8 (= s10 10)
    }
    static s11:u8 (= s11 11)
    fn bar {
        static s12:u8 (= s12 12)
        static s13:u8 (= s13 13)
        {
            static s14:u8 (= s14 14)
        }
        static s15:u8 (= s15 15)
    }
    (foo)
    (bar)
    "#,
        (0..=15).collect::<Vec<_>>().as_ref(),
    )
}
