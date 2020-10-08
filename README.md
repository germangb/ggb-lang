### `ggbc::`

#### Mockup

```rust
static@0x8000 VRAM :: struct {
    tile_data :: union {
        x8000 :: struct { data::[u8; 0x1000] },
        x8800 :: struct {
            _padding::[u8; 0x800],
            data::[u8; 0x1000]
        }
    },
    tile_map :: struct {
        x9800 :: [u8; 0x400],
        x9c00 :: [i8; 0x400]
    }
};

mod math {
    // Computes the sum of 4 bytes
    fn add(a::u8,
           b::u8,
           c::u8,
           d::u8) u8 {
        let tmp::u8 = (+a b);
        (+= tmp c);
        (+= tmp d);
        return tmp;
    }

    // multiplies lower nibbles
    fn mul(a::u8, b::u8) u8 {
        (&= a 0xf);
        (&= b 0xf);
        let result::u8 = 0;
        for _::u8 in 0..b {
            (+= result a);
        }
        return result;
    }
}

// Placeholder lisp-based expressions.
// might be worth keeping as an optional parsing mode though.
(= ([0] VRAM::tile_map::x9800) (math::mul 2 2));
```