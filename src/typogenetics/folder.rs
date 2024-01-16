use crate::typogenetics::{AminoAcid, Base, Enzyme, Orientation, Strand, Turn};

/// | ins | dir |
/// | --- | --- |
/// | cut | s   |
/// | del | s   |
/// | swi | r   |
/// | mvr | s   |
/// | mvl | s   |
/// | cop | r   |
/// | off | l   |
/// | ina | s   |
/// | inc | r   |
/// | ing | r   |
/// | int | l   |
/// | rpy | r   |
/// | rpu | l   |
/// | lpy | l   |
/// | lpu | l   |
///
/// | first | last | base |
/// | ----- | ---- | ---- |
/// | R     | R    | A    |
/// | R     | U    | C    |
/// | R     | D    | G    |
/// | R     | L    | T    |
pub struct Folder;

impl Folder {
    pub fn fold(enzyme: &Enzyme) -> Orientation {
        let mut turning_number = 0;
        for amino_acid in enzyme.iter_amino_acids() {
            let turn = Self::get_turn(amino_acid);
            turning_number += turn.to_int();
        }
        Orientation::from_turning_number(turning_number)
    }

    pub fn get_turn(amino_acid: &AminoAcid) -> Turn {
        match amino_acid {
            AminoAcid::Cut => Turn::S,
            AminoAcid::Del => Turn::S,
            AminoAcid::Swi => Turn::R,
            AminoAcid::Mvr => Turn::S,
            AminoAcid::Mvl => Turn::S,
            AminoAcid::Cop => Turn::R,
            AminoAcid::Off => Turn::L,
            AminoAcid::Ina => Turn::S,
            AminoAcid::Inc => Turn::R,
            AminoAcid::Ing => Turn::R,
            AminoAcid::Int => Turn::L,
            AminoAcid::Rpy => Turn::R,
            AminoAcid::Rpu => Turn::L,
            AminoAcid::Lpy => Turn::L,
            AminoAcid::Lpu => Turn::L,
        }
    }

    pub fn get_binding_site(enzyme: &Enzyme, strand: &Strand) -> Option<usize> {
        let orientation = Self::fold(enzyme);
        let binding_affinity = Self::get_binding_affinity(orientation);

        for (unit, base) in strand.iter_bases().enumerate() {
            if *base == binding_affinity {
                return Some(unit);
            }
        }
        None
    }

    pub fn get_binding_affinity(orientation: Orientation) -> Base {
        match orientation {
            Orientation::R => Base::A,
            Orientation::U => Base::C,
            Orientation::D => Base::G,
            Orientation::L => Base::T,
        }
    }
}
