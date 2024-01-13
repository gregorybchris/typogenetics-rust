use std::{
    fmt::{Debug, Display, Formatter},
    slice::Iter,
};

use crate::amino_acid::AminoAcid;

#[derive(PartialEq, Eq, Clone)]
pub struct Enzyme {
    amino_acids: Vec<AminoAcid>,
}

impl Enzyme {
    pub fn new(amino_acids: Vec<AminoAcid>) -> Enzyme {
        Enzyme { amino_acids }
    }

    #[allow(dead_code)]
    pub fn from_string(enzyme_str: &str) -> Enzyme {
        let amino_acids: Vec<AminoAcid> =
            enzyme_str.split('-').map(AminoAcid::from_string).collect();
        Enzyme { amino_acids }
    }

    pub fn iter_amino_acids(&self) -> Iter<'_, AminoAcid> {
        self.amino_acids.iter()
    }
}

impl Display for Enzyme {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = self
            .amino_acids
            .iter()
            .map(|a| a.to_string())
            .collect::<Vec<String>>()
            .join("-");
        write!(f, "{}", s)
    }
}

impl Debug for Enzyme {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = self
            .amino_acids
            .iter()
            .map(|a| a.to_string())
            .collect::<Vec<String>>()
            .join("-");
        write!(f, "{}", s)
    }
}
