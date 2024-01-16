use crate::search::EditType;
use crate::typogenetics::{Base, Strand};
use rand::Rng;

pub struct Editor;

impl Editor {
    const PROB_MUTATE: f64 = 0.80;
    const PROB_INSERT: f64 = 0.10;
    const PROB_DELETE: f64 = 0.10;

    pub fn edit(strand: &Strand, rng: &mut rand::rngs::ThreadRng) -> Strand {
        let edit_type = Editor::select_edit_type(rng);
        match edit_type {
            EditType::Mutate => Editor::mutate(strand, rng),
            EditType::Insert => Editor::insert(strand, rng),
            EditType::Delete => Editor::delete(strand, rng),
        }
    }

    pub fn mutate(strand: &Strand, rng: &mut rand::rngs::ThreadRng) -> Strand {
        let mut new_bases = strand.clone_bases();
        let r1 = rng.gen_range(0..strand.len());
        let base = new_bases[r1];
        while new_bases[r1] == base {
            let all_bases = [Base::A, Base::C, Base::G, Base::T];
            let r2 = rng.gen_range(0..all_bases.len());
            new_bases[r1] = all_bases[r2];
        }
        Strand::new(new_bases)
    }

    pub fn insert(strand: &Strand, rng: &mut rand::rngs::ThreadRng) -> Strand {
        let mut new_bases = strand.clone_bases();
        let r1 = rng.gen_range(0..=strand.len());
        let all_bases = [Base::A, Base::C, Base::G, Base::T];
        let r2 = rng.gen_range(0..all_bases.len());
        new_bases.insert(r1, all_bases[r2]);
        Strand::new(new_bases)
    }

    pub fn delete(strand: &Strand, rng: &mut rand::rngs::ThreadRng) -> Strand {
        let mut new_bases = strand.clone_bases();
        let r1 = rng.gen_range(0..strand.len());
        new_bases.remove(r1);
        Strand::new(new_bases)
    }

    fn select_edit_type(rng: &mut rand::rngs::ThreadRng) -> EditType {
        let r: f64 = rng.gen();
        let edit_types = [
            (EditType::Mutate, Editor::PROB_MUTATE),
            (EditType::Insert, Editor::PROB_INSERT),
            (EditType::Delete, Editor::PROB_DELETE),
        ];
        let mut cumulative_prob = 0.0;
        for (edit_type, prob) in edit_types.iter() {
            cumulative_prob += prob;
            if r <= cumulative_prob {
                return *edit_type;
            }
        }
        panic!("Random number is not in range [0, 1]");
    }
}
