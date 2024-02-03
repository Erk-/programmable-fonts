use console::{Key, Term};
use std::io::Read;
use tetris::{DebugDisplay, Event, Tetris, TetrisDisplay};

struct TermDisplay {
    term: Term,
}

impl TermDisplay {
    fn new() -> Self {
        TermDisplay {
            term: Term::buffered_stdout(),
        }
    }

    fn key(&self) -> Key {
        self.term.read_key().unwrap()
    }
}

impl TetrisDisplay for TermDisplay {
    fn display(&mut self, gameboard: [u16; 20]) {
        self.term.clear_screen().unwrap();
        for line in gameboard {
            self.term
                .write_line(&format!("{line:010b}").replace('1', "█").replace('0', " "))
                .unwrap();
        }
        self.term.flush().unwrap();
    }
}

fn main() {
    let mut tetris = Tetris::new();
    let mut d = TermDisplay::new();

    loop {
        tetris.display(&mut d);

        let event = match d.key() {
            Key::ArrowLeft => Event::Left,
            Key::ArrowRight => Event::Right,
            Key::ArrowUp => Event::Turn,
            _ => Event::Nop,
        };

        if !tetris.run([event]) {
            println!("Game Over!");
            break;
        }
    }
}
