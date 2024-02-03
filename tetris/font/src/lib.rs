use ds::{start_end_wrapper, ReplaceCord};
use harfbuzz_wasm::{debug, Font, Glyph, GlyphBuffer};
use tetris::{HEIGHT, WIDTH};
use wasm_bindgen::prelude::*;

mod ds;
mod parser;

const PIXEL_HEIGHT: i32 = 40;
const PIXEL_WIDTH: i32 = 40;

#[wasm_bindgen]
pub fn shape(
    _shape_plan: u32,
    font_ref: u32,
    buf_ref: u32,
    _features: u32,
    _num_features: u32,
) -> i32 {
    let font = Font::from_ref(font_ref);
    let mut buffer = GlyphBuffer::from_ref(buf_ref);
    debug(&format!("{:?}", buffer.glyphs));

    let mut rc = ReplaceCord::new(buffer.glyphs.clone());
    rc.replace_all(start_end_wrapper);
    buffer.glyphs = rc.glyph_vec(&font);
    debug(&format!("{:?}", buffer.glyphs));

    let tetris_glyph = font.get_glyph(0x1337, 0);

    for item in buffer.glyphs.iter_mut() {
        if item.codepoint == tetris_glyph {
            continue;
        }
        item.x_advance = font.get_glyph_h_advance(item.codepoint);
    }

    1
}

pub(crate) fn block_at(gameboard: [u16; 20], pos: (u8, u8)) -> bool {
    let (x, y) = pos;
    if (x as usize) >= WIDTH || (y as usize) >= HEIGHT {
        return true;
    }

    let line = gameboard[y as usize];
    let mask = 1 << (WIDTH as u8 - (x + 1));

    (line & mask) != 0
}

pub(crate) fn pixel_at(pos: (u8, u8)) -> Glyph {
    let (posx, posy) = pos;
    let x = PIXEL_WIDTH * (posx as usize) as i32;
    let y = (-PIXEL_HEIGHT * (posy as usize) as i32) + 800;

    Glyph {
        codepoint: 0x1337,
        cluster: (posx as u32) << 8 | posy as u32,
        x_advance: 0,
        y_advance: 0,
        x_offset: x,
        y_offset: y,
        flags: 0,
    }
}
