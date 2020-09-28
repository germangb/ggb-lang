### `ggbc::`

#### Mockup

```rust
// std.ggb

static@0x8000 VRAM :: struct {
    tile_data :: union {
        x8000 :: struct {                          data :: [u8; 0x1000] },
        x8800 :: struct { _padding :: [u8; 0x800], data :: [u8; 0x1000] },
    },
    tile_map  :: struct { x9800 :: [u8; 0x400],
                          x9c00 :: [i8; 0x400], }
};

static@0xff00 IO :: struct {
    joyp :: u8,
};
```

```rust
// main.ggb

use std::VRAM::tile_map;
use std::VRAM::tile_data;

// create black tile
for i in 0..16 {
    tile_data.x8000[i] = 0xff;
}

// set upper-left-most tile to black
std::VRAM::tile_map::x9800[0] = 1;

// enable interrupts
// using gb assembly instructions
asm {
    .ei
}

// loop forever
loop {}
```