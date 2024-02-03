//! Format for the Tetris\tm playing font.
//!
//! ```
//! Tetris(42):wasdQ
//! ```

use std::io::Cursor;

use tetris::Event;

use crate::ds::StartEnd;

pub struct Parser {
    buf: Cursor<String>,
    ended: bool,
    start: usize,
}

impl Parser {
    pub fn new(vec: Vec<u8>) -> Option<(u64, Self)> {
        let mut parser = Parser {
            buf: Cursor::new(String::from_utf8(vec).unwrap()),
            ended: false,
            start: 0,
        };

        let seed = parser.find_start()?;

        Some((seed, parser))
    }

    fn find_start(&mut self) -> Option<u64> {
        let needle = "Tetris(";
        let p1 = self.buf.get_ref().find(needle)?;
        let p2 = p1 + self.buf.get_ref()[p1..].find("):")?;
        let seed: u64 = self.buf.get_ref()[(p1 + needle.len())..p2].parse().ok()?;

        self.start = p1;
        if self.buf.get_ref().len() > p2 + 1 {
            self.buf.set_position(p2 as u64 + 1);
        }

        Some(seed)
    }

    fn next_symbol(&mut self) -> Option<char> {
        let pos = self.buf.position();

        if self.ended {
            return None;
        };
        let c = *self.buf.get_ref().as_bytes().get(pos as usize)? as char;

        if c == 'Q' {
            self.ended = true;
            None
        } else {
            if (pos + 1) >= self.buf.get_ref().len() as u64 {
                self.ended = true;
            } else {
                self.buf.set_position(pos + 1);
            }
            Some(c)
        }
    }

    pub fn start_end(mut self) -> StartEnd {
        if self.next_symbol().is_some() {
            harfbuzz_wasm::debug("start_end called before end of iterator");
            panic!("start_end called before end of iterator");
        }

        StartEnd {
            start: self.start,
            end: self.buf.position() as usize,
        }
    }
}

impl Iterator for Parser {
    type Item = Event;

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.next_symbol()?;
        match c {
            'a' | 'A' => Some(Event::Left),
            'w' | 'W' => Some(Event::Turn),
            'd' | 'D' => Some(Event::Right),
            _ => Some(Event::Nop),
        }
    }
}
