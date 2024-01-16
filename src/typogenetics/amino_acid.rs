use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AminoAcid {
    Cut,
    Del,
    Swi,
    Mvr,
    Mvl,
    Cop,
    Off,
    Ina,
    Inc,
    Ing,
    Int,
    Rpy,
    Rpu,
    Lpy,
    Lpu,
}

impl AminoAcid {
    pub fn from_string(amino_acid_str: &str) -> AminoAcid {
        match amino_acid_str {
            "cut" => AminoAcid::Cut,
            "del" => AminoAcid::Del,
            "swi" => AminoAcid::Swi,
            "mvr" => AminoAcid::Mvr,
            "mvl" => AminoAcid::Mvl,
            "cop" => AminoAcid::Cop,
            "off" => AminoAcid::Off,
            "ina" => AminoAcid::Ina,
            "inc" => AminoAcid::Inc,
            "ing" => AminoAcid::Ing,
            "int" => AminoAcid::Int,
            "rpy" => AminoAcid::Rpy,
            "rpu" => AminoAcid::Rpu,
            "lpy" => AminoAcid::Lpy,
            "lpu" => AminoAcid::Lpu,
            _ => panic!("Invalid amino acid string"),
        }
    }
}

impl Display for AminoAcid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match &self {
            AminoAcid::Cut => "cut",
            AminoAcid::Del => "del",
            AminoAcid::Swi => "swi",
            AminoAcid::Mvr => "mvr",
            AminoAcid::Mvl => "mvl",
            AminoAcid::Cop => "cop",
            AminoAcid::Off => "off",
            AminoAcid::Ina => "ina",
            AminoAcid::Inc => "inc",
            AminoAcid::Ing => "ing",
            AminoAcid::Int => "int",
            AminoAcid::Rpy => "rpy",
            AminoAcid::Rpu => "rpu",
            AminoAcid::Lpy => "lpy",
            AminoAcid::Lpu => "lpu",
        };
        write!(f, "{}", s)
    }
}
