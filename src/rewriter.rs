use crate::{
    amino_acid::AminoAcid, base::Base, base_pair::BasePair, base_type::BaseType, enzyme::Enzyme,
    folder::Folder, strand::Strand,
};

#[derive(Debug, PartialEq, Eq, Clone)]

/// | ins | action                                         |
/// | --- | ---------------------------------------------- |
/// | cut | cut strand(s)                                  |
/// | del | delete a base from strand                      |
/// | swi | switch enzyme to other strand                  |
/// | mvr | move one unit to the right                     |
/// | mvl | move one unit to the left                      |
/// | cop | turn on Copy mode                              |
/// | off | turn off Copy mode                             |
/// | ina | insert A to the right of this unit             |
/// | inc | insert C to the right of this unit             |
/// | ing | insert G to the right of this unit             |
/// | int | insert T to the right of this unit             |
/// | rpy | search for the nearest pyrimidine to the right |
/// | rpu | search for the nearest purine to the right     |
/// | lpy | search for the nearest pyrimidine to the left  |
/// | lpu | search for the nearest purine to the left      |
pub struct Rewriter;

impl Rewriter {
    pub fn rewrite(enzyme: &Enzyme, strand: &Strand) -> Vec<Strand> {
        let mut copy_mode = false;
        let unit = Folder::get_binding_site(enzyme, strand);

        if let Some(mut unit) = unit {
            let mut pairs: Vec<BasePair> = strand
                .iter_bases()
                .map(|&base| BasePair {
                    bind: Some(base),
                    comp: None,
                })
                .collect();

            let mut strands = Vec::new();

            for amino_acid in enzyme.iter_amino_acids() {
                println!("Applying {:?} @ {}, copy={}", amino_acid, unit, copy_mode);

                if *amino_acid == AminoAcid::Cut {
                    let cut_pairs = pairs.split_off(unit + 1);
                    strands.extend(Self::strands_from_pairs(&cut_pairs));
                    pairs.truncate(unit + 1);
                } else if *amino_acid == AminoAcid::Del {
                    if let Some(pair) = pairs.get_mut(unit) {
                        pair.bind = None;
                    }
                    if unit == 0 {
                        println!("Reached end of strand");
                        break;
                    }
                    unit -= 1;

                    if pairs.get(unit).map_or(true, |pair| pair.bind.is_none()) {
                        println!("Reached end of strand");
                        break;
                    }
                } else if *amino_acid == AminoAcid::Swi {
                    if pairs.get(unit).map_or(true, |pair| pair.comp.is_none()) {
                        println!("Tried to switch to empty base pair complement");
                        break;
                    }
                    for pair in &mut pairs {
                        pair.swap();
                    }
                    pairs.reverse();
                    unit = pairs.len() - unit - 1;
                } else if *amino_acid == AminoAcid::Mvr || *amino_acid == AminoAcid::Mvl {
                    if let Some(direction) = Self::amino_acid_to_direction(*amino_acid) {
                        let new_unit = Self::usize_add(unit, direction);
                        if let Some(new_unit) = new_unit {
                            unit = new_unit;
                        } else {
                            println!("Reached end of strand");
                            break;
                        }

                        if unit >= pairs.len()
                            || pairs.get(unit).map_or(true, |pair| pair.bind.is_none())
                        {
                            println!("Reached end of strand");
                            break;
                        }

                        if copy_mode {
                            if let Some(pair) = pairs.get_mut(unit) {
                                pair.add_comp()
                            }
                        }
                    }
                } else if *amino_acid == AminoAcid::Cop {
                    copy_mode = true;
                    if let Some(pair) = pairs.get_mut(unit) {
                        if let Some(bind) = pair.bind {
                            pair.comp = Some(bind.get_complement());
                        }
                    }
                } else if *amino_acid == AminoAcid::Off {
                    copy_mode = false;
                } else if *amino_acid == AminoAcid::Ina
                    || *amino_acid == AminoAcid::Inc
                    || *amino_acid == AminoAcid::Ing
                    || *amino_acid == AminoAcid::Int
                {
                    if let Some(bind) = Self::amino_acid_to_base(*amino_acid) {
                        pairs.insert(
                            unit + 1,
                            BasePair {
                                bind: Some(bind),
                                comp: if copy_mode {
                                    Some(bind.get_complement())
                                } else {
                                    None
                                },
                            },
                        );
                    }
                } else if *amino_acid == AminoAcid::Rpy
                    || *amino_acid == AminoAcid::Rpu
                    || *amino_acid == AminoAcid::Lpy
                    || *amino_acid == AminoAcid::Lpu
                {
                    if let Some(direction) = Self::amino_acid_to_direction(*amino_acid) {
                        let mut end_of_strand = false;

                        while !end_of_strand {
                            let new_unit = Self::usize_add(unit, direction);
                            if let Some(new_unit) = new_unit {
                                unit = new_unit;
                            } else {
                                end_of_strand = true;
                                break;
                            }

                            if unit >= pairs.len() {
                                end_of_strand = true;
                                break;
                            } else {
                                let pair = &pairs[unit];
                                let bind_base = pair.bind;
                                if let Some(bind_base) = bind_base {
                                    if copy_mode {
                                        if let Some(pair) = pairs.get_mut(unit) {
                                            pair.add_comp();
                                        }
                                    }

                                    let base_type = Self::amino_acid_to_base_type(*amino_acid);
                                    if bind_base.is_type(base_type) {
                                        break;
                                    }
                                } else {
                                    end_of_strand = true;
                                    break;
                                }
                            }
                        }

                        if end_of_strand {
                            println!("Reached end of strand");
                            break;
                        }
                    }
                }

                // Debug print for pairs
                println!("{}", Self::pairs_to_string(&pairs));
            }

            strands.extend(Self::strands_from_pairs(&pairs));
            strands
        } else {
            vec![strand.clone()]
        }
    }

    fn usize_add(u: usize, i: i32) -> Option<usize> {
        if i.is_negative() {
            u.checked_sub(i.wrapping_abs() as u32 as usize)
        } else {
            u.checked_add(i as usize)
        }
    }

    fn strands_from_pairs(pairs: &[BasePair]) -> Vec<Strand> {
        let mut strands = Vec::new();
        let mut bind_bases = Vec::new();
        let mut comp_bases = Vec::new();

        for pair in pairs {
            if let Some(bind) = pair.bind {
                bind_bases.push(bind);
            } else if !bind_bases.is_empty() {
                strands.push(Strand::new(bind_bases.clone()));
                bind_bases.clear();
            }

            if let Some(comp) = pair.comp {
                comp_bases.push(comp);
            } else if !comp_bases.is_empty() {
                strands.push(Strand::new(comp_bases.iter().cloned().rev().collect()));
                comp_bases.clear();
            }
        }

        if !bind_bases.is_empty() {
            strands.push(Strand::new(bind_bases.clone()));
        }

        if !comp_bases.is_empty() {
            strands.push(Strand::new(comp_bases.iter().cloned().rev().collect()));
        }

        strands
    }

    fn amino_acid_to_base(amino_acid: AminoAcid) -> Option<Base> {
        match amino_acid {
            AminoAcid::Ina => Some(Base::A),
            AminoAcid::Inc => Some(Base::C),
            AminoAcid::Ing => Some(Base::G),
            AminoAcid::Int => Some(Base::T),
            _ => None,
        }
    }

    fn amino_acid_to_base_type(amino_acid: AminoAcid) -> BaseType {
        match amino_acid {
            AminoAcid::Rpy | AminoAcid::Rpu => BaseType::Pyrimidine,
            AminoAcid::Lpy | AminoAcid::Lpu => BaseType::Purine,
            _ => panic!("Invalid amino acid"),
        }
    }

    fn amino_acid_to_direction(amino_acid: AminoAcid) -> Option<i32> {
        match amino_acid {
            AminoAcid::Rpy | AminoAcid::Rpu | AminoAcid::Mvr => Some(1),
            AminoAcid::Lpy | AminoAcid::Lpu | AminoAcid::Mvl => Some(-1),
            _ => None,
        }
    }

    fn pairs_to_string(pairs: &[BasePair]) -> String {
        let mut res = String::from("[ ");
        let comp_map: std::collections::HashMap<Base, &str> = [
            (Base::A, "∀"),
            (Base::C, "Ↄ"),
            (Base::G, "⅁"),
            (Base::T, "⊥"),
        ]
        .iter()
        .cloned()
        .collect();

        for pair in pairs {
            if let Some(comp) = pair.comp {
                res.push_str(comp_map.get(&comp).unwrap());
                res.push(' ');
            } else {
                res.push_str("  ");
            }
        }

        res.push_str("]\n[ ");

        for pair in pairs {
            if let Some(bind) = pair.bind {
                res.push_str(&bind.to_string());
                res.push(' ');
            } else {
                res.push_str("  ");
            }
        }

        res.push(']');
        res
    }
}
