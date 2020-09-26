## Memory model
```rust
mod gb {
    union TileData {
        x8000 :: struct {
            data     :: [u8; 0x1000],
        },
        x8800 :: struct {
            _padding :: [u8; 0x0800],
            data     :: [u8; 0x1000],
        }      
    }

    wram in [0xc000..0xdfff] {}
    vram in [0x8000..0x9fff] {
        _padding           :: [u8; 0x0800],
        mut tile_data      :: TileData,
        mut tile_map_x9800 :: [u8; 0x3ff],
        mut tile_map_x9c00 :: [u8; 0x3ff],
    }
    io in [0xff00..0xff7f] {
        joyp               :: u8,
        mut sb             :: u8,
        mut sc             :: u8,
        _padding0          :: u8,
        // timer
        mut div            :: u8,
        mut tima           :: u8,
        mut tma            :: u8,
        mut tac            :: u8,
        // audio
        mut nr10           :: u8,
        mut nr11           :: u8,
        mut nr12           :: u8,
        mut nr13           :: u8,
        mut nr14           :: u8,
        _padding1          :: u8,
        mut nr16           :: u8,
        mut nr17           :: u8,
        mut nr18           :: u8,
        mut nr19           :: u8,
        mut nr1a           :: u8,
        mut nr1b           :: u8,
        mut nr1c           :: u8,
        mut nr1d           :: u8,
        mut nr1e           :: u8,
        _padding2          :: u8,
    }
}

enum Cell :: u8 {
    Alive = 0,
    Dead = 1,
}

struct World {
    a :: [Cell; 32*32],
    b :: [Cell; 32*32],
}

struct Cursor {
    y :: u8,
    x :: u8,
}

game_of_life in gb::wram[..] {
    mut world  :: World,
    mut cursor :: Cursor,
}
```