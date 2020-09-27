## Memory model
```rust
// raw mapping from the entire GB memory space to types.
// to provide bindings to the GB memory and systems.
mod gb {
    static@0x8000 VRAM :: struct {
        tile_data :: union {
            x8000 :: struct {                          data :: [u8; 0x1000], },
            x8800 :: struct { _padding :: [u8; 0x800], data :: [u8; 0x1000], },
        },
        tile_map  :: struct { x9800 :: [u8; 0x400],
                              x9c00 :: [i8; 0x400], },
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
        joyp :: u8,
    };
}
```

```rust
static GLOBAL :: [u8; 6] = "german";
static CURSOR :: struct { y :: u8, y :: u8, };

fn init_vram() {
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

fn main() {
    init_vram();
    
    // enable interrupts
    asm {
        .ei
    }

    // loop forever
    loop {}
}
```