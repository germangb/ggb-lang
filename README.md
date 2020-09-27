## Memory model
```rust
// raw mapping from the entire GB memory space to types.
// to provide bindings to the GB memory and systems.
mod gb {
    static@0x8000 VRAM :: struct {
        tile_data :: union {
            x8000 :: struct {                         mut data :: [u8; 4096], },
            x8800 :: struct { _padding :: [u8; 2048], mut data :: [u8; 4096], },
        },
        tile_map  :: struct { mut x9800 :: [u8; 1024],
                              mut x9c00 :: [i8; 1024], },
    };
    
    static@0xff00 IO :: struct {
        // Bit 7 - Not used
        // Bit 6 - Not used
        // Bit 5 - P15 Select Button Keys      (0=Select)
        // Bit 4 - P14 Select Direction Keys   (0=Select)
        // Bit 3 - P13 Input Down  or Start    (0=Pressed) (Read Only)
        // Bit 2 - P12 Input Up    or Select   (0=Pressed) (Read Only)
        // Bit 1 - P11 Input Left  or Button B (0=Pressed) (Read Only)
        // Bit 0 - P10 Input Right or Button A (0=Pressed) (Read Only)
        mut joyp :: u8,
    };
}
```

```rust
static GLOBAL :: u8;
static CURSOR :: struct { mut y :: u8, mut y :: u8, };

fn init_vram {
    const TILE_DATA :: [u8; 32] = [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff
                                   0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                                   0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                                   0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    for i in 0..32 {
        gb::VRAM::tile_data::x8000[i] = TILE_DATA[i];
    }
    // init tile map
    for i in 0..0x400 {
        gb::VRAM::tile_map::x9800[i] = 0;
    }

    // dma transfer
    asm {
        .ld  (0xff46), %a
        .ld  %a, 0x28
wait:
        .dec %a
        .jr  nz, wait
    }
}

fn main {
    init_vram();
    
    // enable interrupts
    asm {
        .ei
    }

    // loop forever
    loop {}
}
```