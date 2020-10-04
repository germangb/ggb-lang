use ggbc_parser::{Ast, Error};
fn main() {
    ggbc_parser::parse::<Ast>(
        r#"
// adds a layer of typing to an existing region of memory
// here, VRAM starts at address 0x8000 ans is layed out like this:
mod std {
    static@0x0000 MEM_MAP :: [u8; 0x10000];
    static@0x8000 VRAM :: struct {
        tile_data :: union {
            x8000 :: struct {                        data::[u8; 0x1000] },
            x8800 :: struct { _padding::[u8; 0x800], data::[u8; 0x1000] }
        },
        tile_map :: struct { x9800::[u8; 0x400],
                             x9c00::[i8; 0x400] }
    };
}

// C-style for loop
for offset::u16 in 0..+16 {
    // equivalent statements:
    (= ([] std::MEM_MAP (+ 0x8000 offset)) 0xff);
    (= ([] std::VRAM::tile_data::x8000 offset) 0xff);
}

fn __vblank (foo::u8) {
    (+ foo 2);
    std::VRAM;
}

//let fuck::u8 = (__vblank (42));

loop {}
    "#,
    )
    .unwrap();
}
