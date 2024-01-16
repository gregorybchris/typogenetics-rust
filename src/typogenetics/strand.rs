use std::{
    fmt::{Debug, Display, Formatter},
    slice::Iter,
};

use crate::{Base, Duplet};

#[derive(PartialEq, Eq, Clone)]
pub struct Strand {
    bases: Vec<Base>,
}

impl Strand {
    pub fn new(bases: Vec<Base>) -> Strand {
        Strand { bases }
    }

    pub fn from_string(strand_str: &str) -> Strand {
        let bases: Vec<Base> = strand_str
            .chars()
            .filter(|&c| c != ' ')
            .map(|c| Base::from_string(&c.to_string()))
            .collect();

        Strand { bases }
    }

    pub fn iter_bases(&self) -> Iter<'_, Base> {
        self.bases.iter()
    }

    pub fn iter_duplets(&self) -> impl Iterator<Item = Duplet> + '_ {
        self.bases.chunks(2).flat_map(|chunk| {
            if chunk.len() == 2 {
                Some((chunk[0], chunk[1]))
            } else {
                None
            }
        })
    }
}

impl Display for Strand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for base in &self.bases {
            s.push_str(&base.to_string());
        }
        write!(f, "{}", s)
    }
}

impl Debug for Strand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for base in &self.bases {
            s.push_str(&base.to_string());
        }
        write!(f, "{}", s)
    }
}
