use ggbc::{byteorder::NativeEndian, ir::Ir};

fn test(size: u16, input: &str) {
    let ast = ggbc::parser::parse(input).unwrap();
    let ir: Ir<NativeEndian> = Ir::new(&ast);
    assert_eq!(size, ir.routines[0].stack_size);
}

fn test_routine(size: u16, input: &str) {
    let ast = ggbc::parser::parse(input).unwrap();
    let ir: Ir<NativeEndian> = Ir::new(&ast);
    assert_eq!(size, ir.routines[1].stack_size);
}

#[test]
fn static_usage_0() {
    test(
        7,
        r#"
        let s0:u8 = 0
        let s1:[u8 2] = [1 2]
        let s2:[[u8 2] 2] = [[0 1] [2 3]]
        "#,
    );
}

#[test]
fn static_usage_1() {
    test(
        5,
        r#"
        let s0:u8 = 0
        {
            let s1:[u8 2] = [1 2]
        }
        let s2:[[u8 2] 2] = [[0 1] [2 3]]
        "#,
    );
}

#[test]
fn static_usage_2() {
    test(
        17,
        r#"
        let s0:u8 = 0
        {
            let s1:[u8 16] = [1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16]
        }
        let s2:[[u8 2] 2] = [[0 1] [2 3]]
        "#,
    );
}

#[test]
fn static_usage_3() {
    test(
        17,
        r#"
        let s0:u8 = 0
        {
            {
                let s1:[u8 16] = [1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16]
            }
        }
        let s2:[[u8 2] 2] = [[0 1] [2 3]]
        "#,
    );
}

#[test]
fn static_usage_4() {
    test(
        21,
        r#"
        let s0:u8 = 0
        {
            let s1:[u8 16] = [1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16]
        }
        let s2:[[u8 2] 2] = [[0 1] [2 3]]
        let s3:[u8 16] = [1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16]
        "#,
    );
}

#[test]
fn static_usage_5() {
    test_routine(
        21,
        r#"
        fn foo {
            let s0:u8 = 0
            {
                let s1:[u8 16] = [1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16]
            }
            let s2:[[u8 2] 2] = [[0 1] [2 3]]
            let s3:[u8 16] = [1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16]
        }
        "#,
    );
}

#[test]
fn static_usage_6() {
    test_routine(
        28,
        r#"
        fn foo(a0:u8 a1:u8 a3:[u8 4]) {
            let s0:u8 = 0
            {
                let s1:[u8 16] = [1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16]
            }
            let s2:[[u8 2] 2] = [[0 1] [2 3]]
            let s3:[u8 16] = [1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16]
            let s4:u8 = 0
        }
        "#,
    );
}

#[test]
fn static_usage_7() {
    test(
        1,
        r#"
        fn foo(a0:u8 a1:u8):u8 {return 0}
        let bar:u8 = (foo 0 1)
        "#,
    );
}

#[test]
fn static_usage_8() {
    test(
        2,
        r#"
        fn foo(a0:u8 a1:u8):u8 {return 0}
        fn baz(a0:u8):u8 {return 0}
        let bar:u8 = (foo 0 1)
        let qux:u8 = (baz 2)
        "#,
    );
}
