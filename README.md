### `ggbc::`

#### Mockup

```rust
// ::VRAM
//  ::tile_data
//  ::tile_maps
//      ::x9800
//      ::x9c000
static@0x8000 VRAM :: struct {
    tile_data :: union {
        x8000 :: struct {
            data::[u8 0x1000]
        }
        x8800 :: struct {
            _padding::[u8 0x0800]
                data::[u8 0x1000]
        }
    }
    tile_maps :: struct {
        x9800::[u8 0x400]
        x9c00::[i8 0x400]
    }
}

mod math {
    // ::math::add
    //  ::a
    //  ::b
    //  ::c
    //  ::d
    //  ::tmp
    fn add(a::u8
           b::u8
           c::u8
           d::u8)::u8 {
        let tmp::u8 = (+a b)
        (+= tmp c)
        (+= tmp d)
        return tmp
    }

    // ::math::mul
    //  ::a
    //  ::b
    //  ::result
    //  ::0::_i
    //  ::tmp
    fn mul(a::u8 b::u8)::u8 {
        // stack
        // ===
        // 0: a
        // 1: b
        // 2: result
        // 3: _i
        (&= a 0xf)
        (&= b 0xf)
        let result::u8 = 0
        for _i::u8 in 0..b {
            (+= result a)
        }
        let tmp::u8 = result
        return tmp
    }
}

// ::tile_map
//  ::foo
//  ::array
//  ::bar
//  ::baz
fn tile_map(foo:: fn([u8 2] u8)::u8) {
    let array::[i8; 4] = [0 -1 2 3]
    let bar::u8 = math::add
    mod math2 {
    }
    let baz::u8 = math2
    (foo [0 baz] 42)
}

// Placeholder lisp-based expressions.
// might be worth keeping as an optional parsing mode though.
(= ([0] VRAM::tile_maps::x9800) (math::mul 2 -2))
```

#### Wholesome language

```
.. |
20 |         let fuck::u8 = 0;
.. |             ^^^^ Forbidden identifier (Not the F-word, please...).
```
