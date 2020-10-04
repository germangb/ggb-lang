### `ggbc::`

#### Mockup

```rust
// std.ggbc

static@0x8000 VRAM :: struct {
    tile_data::union {
        x8000::struct {                        data::[u8; 0x1000] },
        x8800::struct { _padding::[u8; 0x800], data::[u8; 0x1000] }
    },
    tile_map::struct { x9800::[u8; 0x400],
                       x9c00::[i8; 0x400] }
};
```

```rust
// main.ggbc

use std::VRAM;

let foo::u8 = 42;
let bar::&u8 = @foo;

// create black tile
for i::u8 in 0..16 {
    VRAM::tile_data::x8000[16 + i] = 0xff;
}

VRAM::tile_map::x9800[0] = 1;

// enable interrupts
// using gb assembly instructions
asm {
    .ei
}

// loop forever
loop {}
```