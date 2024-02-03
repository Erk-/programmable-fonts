#![allow(clippy::new_without_default)]

mod display;

pub use display::{DebugDisplay, TetrisDisplay};

pub const WIDTH: usize = 10;
pub const HEIGHT: usize = 20;
const LINE: u16 = full_line();

const fn full_line() -> u16 {
    let mut i = 0;
    let mut l = 0;
    while i < WIDTH {
        l |= 1 << i;
        i += 1;
    }

    l
}

/// Deterministic Tetris that can be rendered at any time.
pub struct Tetris {
    rand: u64,
    // Tetris is 10 wide and 20 high, use a bitmap to draw from, upper
    // 6 bits are not used
    gameboard: [u16; HEIGHT],
    // 0..4 clockwise
    rot: u8,
    inflight_t: Tetromino,
    inflight_pos: (u8, u8),
}

#[repr(u8)]
enum Tetromino {
    /// ```
    /// XXXX
    /// ```
    I,
    /// ```
    /// X
    /// XXX
    /// ``
    J,
    /// ```
    ///   X
    /// XXX
    /// ```
    L,
    /// ```
    /// XX
    /// XX
    /// ```
    O,
    /// ```
    ///  XX  X    
    /// XX   XX
    ///       X
    /// ```
    S,
    /// ```
    ///  X
    /// XXX
    /// ```
    T,
    /// ```
    /// XX
    ///  XX
    /// ```
    Z,
}

impl From<u64> for Tetromino {
    fn from(value: u64) -> Self {
        let idx = value % 7;
        match idx {
            0 => Tetromino::I,
            1 => Tetromino::J,
            2 => Tetromino::L,
            3 => Tetromino::O,
            4 => Tetromino::S,
            5 => Tetromino::T,
            6 => Tetromino::Z,
            _ => unreachable!(),
        }
    }
}

impl Tetromino {
    const fn offset(&self) -> [[Pos; 4]; 4] {
        match self {
            Tetromino::I => [
                [(0, 0), (0, 1), (0, 2), (0, 3)],
                [(0, 0), (1, 0), (2, 0), (3, 0)],
                [(0, 0), (0, 1), (0, 2), (0, 3)],
                [(0, 0), (1, 0), (2, 0), (3, 0)],
            ],
            Tetromino::J => [
                [(0, 0), (0, 1), (1, 1), (2, 1)],
                [(0, 0), (1, 0), (0, 1), (0, 2)],
                [(0, 0), (1, 0), (2, 0), (2, 1)],
                [(1, 0), (1, 1), (0, 2), (1, 2)],
            ],
            Tetromino::L => [
                [(2, 0), (0, 1), (1, 1), (2, 1)],
                [(0, 0), (0, 1), (0, 2), (1, 2)],
                [(0, 0), (1, 0), (2, 0), (0, 1)],
                [(0, 0), (1, 0), (1, 1), (1, 2)],
            ],
            Tetromino::O => [
                [(0, 0), (0, 1), (1, 0), (1, 1)],
                [(0, 0), (0, 1), (1, 0), (1, 1)],
                [(0, 0), (0, 1), (1, 0), (1, 1)],
                [(0, 0), (0, 1), (1, 0), (1, 1)],
            ],
            Tetromino::S => [
                [(1, 0), (2, 0), (0, 1), (1, 1)],
                [(0, 0), (0, 1), (1, 1), (1, 2)],
                [(1, 0), (2, 0), (0, 1), (1, 1)],
                [(0, 0), (0, 1), (1, 1), (1, 2)],
            ],
            Tetromino::T => [
                [(1, 0), (0, 1), (1, 1), (2, 1)],
                [(0, 0), (0, 1), (1, 1), (0, 2)],
                [(0, 0), (1, 0), (2, 0), (1, 1)],
                [(1, 0), (0, 1), (1, 1), (1, 2)],
            ],
            Tetromino::Z => [
                [(0, 0), (1, 0), (1, 1), (2, 1)],
                [(1, 0), (0, 1), (1, 1), (0, 2)],
                [(0, 0), (1, 0), (1, 1), (2, 1)],
                [(1, 0), (0, 1), (1, 1), (0, 2)],
            ],
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Event {
    Turn,
    Left,
    Right,
    Nop,
}

type Pos = (u8, u8);

impl Tetris {
    pub fn new() -> Self {
        Self::with_seed(42)
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut t = Tetris {
            rand: seed,
            gameboard: [0; HEIGHT],
            rot: 0,
            inflight_t: Tetromino::Z,
            inflight_pos: (0, 0),
        };
        t.new_block();
        t
    }

    /// False if the game is over.
    pub fn run<Iter: IntoIterator<Item = Event>>(&mut self, iter: Iter) -> bool {
        for e in iter {
            if !self.step(e) {
                //mreturn false;
            }
        }
        true
    }

    fn next_rand(&mut self) -> u64 {
        let mut x = self.rand;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.rand = x;
        x
    }

    fn new_block(&mut self) {
        let r = self.next_rand();
        self.inflight_pos = (4, 0);
        self.inflight_t = r.into();
    }

    /// Returns false if game is over.
    fn step(&mut self, event: Event) -> bool {
        let allowed = match event {
            Event::Turn => self.rot(),
            Event::Left => self.left(),
            Event::Right => self.right(),
            Event::Nop => self.nop(),
        };

        if !allowed {
            self.burn()
        } else {
            true
        }
    }

    fn nop(&mut self) -> bool {
        let [_, down, _] = self.can_move();
        if down {
            self.inflight_pos.1 += 1;
        }
        down
    }

    fn left(&mut self) -> bool {
        let [left, _, _] = self.can_move();
        if left {
            self.inflight_pos.0 -= 1;
        }
        self.nop()
    }

    fn right(&mut self) -> bool {
        let [_, _, right] = self.can_move();
        if right {
            self.inflight_pos.0 += 1;
        }
        self.nop()
    }

    fn rot(&mut self) -> bool {
        self.rot = self.rot.wrapping_add(1);
        let allowed = self.t_blocks().iter().all(|p| !self.block_at(*p));
        if !allowed {
            self.rot = self.rot.wrapping_sub(1);
        }

        true
        //allowed
    }

    fn t_blocks(&self) -> [Pos; 4] {
        let offset = self.inflight_t.offset()[self.rot as usize % 4];

        let (x, y) = self.inflight_pos;

        offset.map(|(ox, oy)| (x + ox, y + oy))
    }

    fn draw_inflight(&self) -> [u16; 20] {
        let mut draw = self.gameboard;

        for (x, y) in self.t_blocks() {
            let line = &mut draw[y as usize];
            let mask = 1 << (WIDTH as u8 - (x + 1));
            *line |= mask;
        }

        draw
    }

    // Left, Down, Right
    fn can_move(&self) -> [bool; 3] {
        let blocks = self.t_blocks();
        let left = blocks
            .iter()
            .all(|(x, y)| !self.block_at((x.wrapping_sub(1), y.wrapping_add(1))));
        let down = blocks
            .iter()
            .all(|(x, y)| !self.block_at((*x, y.wrapping_add(1))));
        let right = blocks
            .iter()
            .all(|(x, y)| !self.block_at((x.wrapping_add(1), y.wrapping_add(1))));

        [left, down, right]
    }

    fn block_at(&self, pos: (u8, u8)) -> bool {
        let (x, y) = pos;
        if (x as usize) >= WIDTH || (y as usize) >= HEIGHT {
            return true;
        }

        let line = self.gameboard[y as usize];
        let mask = 1 << (WIDTH as u8 - (x + 1));

        (line & mask) != 0
    }

    pub fn display<D: TetrisDisplay>(&self, d: &mut D) {
        let draw = self.draw_inflight();
        d.display(draw);
    }

    fn clear_lines(&mut self) {
        let mut i = 0;
        while i < HEIGHT {
            if self.gameboard[i] == LINE {
                self.gameboard[i] = 0;
                self.gameboard[..=i].rotate_right(1);
            }
            i += 1;
        }
    }

    fn burn(&mut self) -> bool {
        self.gameboard = self.draw_inflight();
        self.clear_lines();
        self.new_block();
        self.t_blocks().iter().all(|p| !self.block_at(*p))
    }
}
