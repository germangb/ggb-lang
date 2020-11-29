use ggbc::{byteorder::NativeEndian, ir::Ir, parser::parse};
use ggbc_vm::{Machine, Opts};

fn _test_const(input: &str, gt: &[u8]) {
    let ast = parse(input).unwrap();
    let ir: Ir<NativeEndian> = Ir::new(&ast);
    let result = &ir.const_[..gt.len()];
    assert_eq!(gt, result);
}

#[test]
fn test_const() {
    _test_const(
        r#"
    const s0:u8 = 0
    const s1:u8 = 1
    const s2:u8 = 2
    {
        const s3:u8 = 3
        const s4:u8 = 4
        const s5:u8 = 5
        {
            const s6:u8 = 6
            const s7:u8 = 7
            {
                const s8:u8 = 8
            }
            const s9:u8 = 9
        }
        const s10:u8 = 10
    }
    const s11:u8 = 11
    {
        const s12:u8 = 12
        const s13:u8 = 13
        {
            const s14:u8 = 14
        }
        const s15:u8 = 15
    }
    "#,
        (0..=15).collect::<Vec<_>>().as_ref(),
    )
}

#[test]
fn test_const_fn() {
    _test_const(
        r#"
    const s0:u8 = 0
    const s1:u8 = 1
    const s2:u8 = 2
    fn foo {
        const s3:u8 = 3
        const s4:u8 = 4
        const s5:u8 = 5
        {
            const s6:u8 = 6
            const s7:u8 = 7
            {
                const s8:u8 = 8
            }
            const s9:u8 = 9
        }
        const s10:u8 = 10
    }
    const s11:u8 = 11
    fn bar {
        const s12:u8 = 12
        const s13:u8 = 13
        {
            const s14:u8 = 14
        }
        const s15:u8 = 15
    }
    "#,
        (0..=15).collect::<Vec<_>>().as_ref(),
    )
}
