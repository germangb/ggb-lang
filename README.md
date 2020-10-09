### `ggbc::`

#### Mockup

```rust
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
    tile_map :: struct {
        x9800::[u8 0x400]
        x9c00::[i8 0x400]
    }
}

mod math {
    fn add(a::u8
           b::u8
           c::u8
           d::u8) u8 {
        let tmp::u8 = (+a b)
        (+= tmp c)
        (+= tmp d)
        return tmp
    }

    fn mul(a::u8 b::u8) u8 {
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
        return result
    }
}

fn tile_map {
    let array::[i8 4] = [0 -1 2 3]
    let foo::u8 = math::add
    mod math2 {
    }
    let bar::u8 = math2
}

// Placeholder lisp-based expressions.
// might be worth keeping as an optional parsing mode though.
(= ([0] VRAM::tile_map::x9800)
   (math::mul 2 -2))
```

#### Wholesome language

```
.. |
20 |         let fuck::u8 = 0;
.. |             ^^^^ Forbidden identifier (Not the F-word, please...).
```
