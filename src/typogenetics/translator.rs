use crate::{AminoAcid, Base, Duplet, Enzyme, Strand};

/// |     | A   | C   | G   | T   |
/// | --- | --- | --- | --- | --- |
/// | A   |     | cut | del | swi |
/// | C   | mvr | mvl | cop | off |
/// | G   | ina | inc | ing | int |
/// | T   | rpy | rpu | lpy | lpu |
pub struct Translator;

impl Translator {
    pub fn translate(strand: &Strand) -> Vec<Enzyme> {
        let mut enzymes = Vec::new();
        let mut amino_acids = Vec::new();

        for duplet in strand.iter_duplets() {
            if let Some(amino_acid) = Self::translate_duplet(duplet) {
                amino_acids.push(amino_acid);
            } else if !amino_acids.is_empty() {
                let enzyme = Enzyme::new(amino_acids.clone());
                enzymes.push(enzyme);
                amino_acids.clear();
            }
        }

        if !amino_acids.is_empty() {
            let enzyme = Enzyme::new(amino_acids.clone());
            enzymes.push(enzyme);
        }

        enzymes
    }

    pub fn translate_duplet(duplet: Duplet) -> Option<AminoAcid> {
        match duplet {
            (Base::A, Base::A) => None,
            (Base::A, Base::C) => Some(AminoAcid::Cut),
            (Base::A, Base::G) => Some(AminoAcid::Del),
            (Base::A, Base::T) => Some(AminoAcid::Swi),
            (Base::C, Base::A) => Some(AminoAcid::Mvr),
            (Base::C, Base::C) => Some(AminoAcid::Mvl),
            (Base::C, Base::G) => Some(AminoAcid::Cop),
            (Base::C, Base::T) => Some(AminoAcid::Off),
            (Base::G, Base::A) => Some(AminoAcid::Ina),
            (Base::G, Base::C) => Some(AminoAcid::Inc),
            (Base::G, Base::G) => Some(AminoAcid::Ing),
            (Base::G, Base::T) => Some(AminoAcid::Int),
            (Base::T, Base::A) => Some(AminoAcid::Rpy),
            (Base::T, Base::C) => Some(AminoAcid::Rpu),
            (Base::T, Base::G) => Some(AminoAcid::Lpy),
            (Base::T, Base::T) => Some(AminoAcid::Lpu),
        }
    }
}
