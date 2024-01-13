#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Orientation {
    U,
    D,
    L,
    R,
}

impl Orientation {
    pub fn from_turning_number(turning_number: i32) -> Orientation {
        match turning_number.rem_euclid(4) {
            0 => Orientation::R,
            1 => Orientation::D,
            2 => Orientation::L,
            3 => Orientation::U,
            _ => panic!("Invalid turning number: {}", turning_number),
        }
    }
}
