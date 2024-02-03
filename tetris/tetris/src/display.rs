pub trait TetrisDisplay {
    fn display(&mut self, gameboard: [u16; 20]);
}

pub struct DebugDisplay;

impl TetrisDisplay for DebugDisplay {
    fn display(&mut self, gameboard: [u16; 20]) {
        for line in gameboard {
            println!("{line:010b}");
        }
    }
}
