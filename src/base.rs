use std::fmt::{Display, Formatter, Result};

use crate::base_type::BaseType;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Base {
    C,
    G,
    T,
    A,
}

impl Display for Base {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Base::C => write!(f, "C"),
            Base::G => write!(f, "G"),
            Base::T => write!(f, "T"),
            Base::A => write!(f, "A"),
        }
    }
}

impl Base {
    pub fn from_string(base_str: &str) -> Base {
        match base_str {
            "C" => Base::C,
            "G" => Base::G,
            "T" => Base::T,
            "A" => Base::A,
            _ => panic!("Invalid base string: {}", base_str),
        }
    }

    pub fn is_type(&self, base_type: BaseType) -> bool {
        match base_type {
            BaseType::Purine => self.is_purine(),
            BaseType::Pyrimidine => self.is_pyrimidine(),
        }
    }

    pub fn is_purine(&self) -> bool {
        matches!(self, Base::A | Base::G)
    }

    pub fn is_pyrimidine(&self) -> bool {
        matches!(self, Base::C | Base::T)
    }

    pub fn get_complement(&self) -> Base {
        match self {
            Base::C => Base::G,
            Base::G => Base::C,
            Base::T => Base::A,
            Base::A => Base::T,
        }
    }
}
