use harfbuzz_wasm::{debug, Font, Glyph};
use tetris::{Tetris, TetrisDisplay, HEIGHT, WIDTH};

use crate::{block_at, parser::Parser, pixel_at};

pub struct ReplaceCord {
    inner: Vec<StrOrReplaced>,
}

#[derive(Debug, Clone)]
pub struct StartEnd {
    pub(crate) start: usize,
    pub(crate) end: usize,
}

fn print_rc(rc: &ReplaceCord) -> String {
    let mut s = String::new();
    for e in rc.inner.iter() {
        match e {
            StrOrReplaced::Str(_) => s.push('S'),
            StrOrReplaced::Replaced(_) => s.push('R'),
        }
    }
    s
}

impl ReplaceCord {
    pub fn new(s: Vec<Glyph>) -> Self {
        ReplaceCord {
            inner: vec![StrOrReplaced::Str(s)],
        }
    }

    fn replace_one<F: Fn(&[Glyph]) -> Option<(StartEnd, Vec<Glyph>)>>(
        &mut self,
        f: F,
    ) -> Option<()> {
        let lst = self.inner.clone();
        for (i, sr) in lst.iter().enumerate() {
            match sr {
                StrOrReplaced::Str(s) => {
                    let Some((se, gs)) = f(s) else {
                        continue;
                    };
                    let (before, rest) = s.split_at(se.start);
                    let (_removed, after) = rest.split_at(se.end + 1 - se.start);

                    debug(&format!("se: {se:?}, removed: {:?}", _removed));

                    // Stupidly expensive operations.
                    // Could be made a bit better with extend_from_within
                    self.inner.remove(i);
                    self.inner.insert(i, StrOrReplaced::Str(before.to_owned()));
                    self.inner.insert(i + 1, StrOrReplaced::Replaced(gs));
                    self.inner
                        .insert(i + 2, StrOrReplaced::Str(after.to_owned()));

                    return Some(());
                }
                StrOrReplaced::Replaced(_) => continue,
            }
        }

        None
    }

    pub fn replace_all<F: Fn(&[Glyph]) -> Option<(StartEnd, Vec<Glyph>)>>(&mut self, f: F) {
        while self.replace_one(&f).is_some() {
            // Intentionally left empty.
        }
    }

    pub fn glyph_vec(self, font: &Font) -> Vec<Glyph> {
        debug(&print_rc(&self));
        let mut lst = Vec::new();

        for sr in self.inner.into_iter() {
            match sr {
                StrOrReplaced::Str(s) => {
                    lst.extend_from_slice(&s);
                }
                StrOrReplaced::Replaced(v) => {
                    lst.extend_from_slice(&v);
                }
            }
        }

        for (i, g) in lst.iter_mut().enumerate() {
            g.cluster = i as u32;
            g.codepoint = font.get_glyph(g.codepoint, 0);
        }

        lst
    }
}

pub fn start_end_wrapper(gs: &[Glyph]) -> Option<(StartEnd, Vec<Glyph>)> {
    let s = gs
        .iter()
        .map(|g| char::from_u32(g.codepoint).unwrap())
        .collect::<String>();

    let (seed, mut parser) = Parser::new(s.as_bytes().to_vec())?;

    let mut tetris = Tetris::with_seed(seed);
    tetris.run(&mut parser);
    let mut display = SavingDisplay::new();
    tetris.display(&mut display);
    let start_end = parser.start_end();

    let space = Glyph {
        codepoint: b' ' as u32,
        cluster: 0,
        x_advance: 0,
        y_advance: 0,
        x_offset: 0,
        y_offset: 0,
        flags: 0,
    };

    display.new_glyphs.reserve(2);
    display.new_glyphs.push(space);
    display.new_glyphs.push(space);

    Some((start_end, display.new_glyphs))
}

struct SavingDisplay {
    pub new_glyphs: Vec<Glyph>,
}

impl SavingDisplay {
    fn new() -> Self {
        SavingDisplay {
            new_glyphs: Vec::new(),
        }
    }
}

impl TetrisDisplay for SavingDisplay {
    fn display(&mut self, gameboard: [u16; 20]) {
        let mut new_glyphs = Vec::new();
        for h in 0..HEIGHT {
            for w in 0..WIDTH {
                if block_at(gameboard, (w as u8, h as u8)) {
                    debug(&format!("Block at: ({h}; {w})"));
                    new_glyphs.push(pixel_at((w as u8, h as u8)));
                }
            }
        }
        self.new_glyphs = new_glyphs;
    }
}

#[derive(Clone)]
enum StrOrReplaced {
    Str(Vec<Glyph>),
    Replaced(Vec<Glyph>),
}
