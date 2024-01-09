use std::slice::Iter;

use crate::amino_acid::AminoAcid;

#[derive(Debug, PartialEq, Eq, Clone)]
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
