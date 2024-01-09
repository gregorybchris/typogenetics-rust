#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Turn {
    L,
    S,
    R,
}

impl Turn {
    pub fn to_int(self) -> i32 {
        match self {
            Turn::L => -1,
            Turn::S => 0,
            Turn::R => 1,
        }
    }
}
