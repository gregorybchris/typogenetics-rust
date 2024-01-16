use crate::typogenetics::{Rewriter, Strand, Translator};
use rand::Rng;
use std::collections::HashSet;

pub struct SearchAlgos;

impl SearchAlgos {
    pub fn random(
        init_strand: &Strand,
        n_iterations: i32,
        rng: &mut rand::rngs::ThreadRng,
        print_strands: bool,
    ) {
        let mut strands = vec![init_strand.clone()];
        let mut known_set = HashSet::new();
        known_set.insert(format!("{:?}", init_strand));

        for _ in 0..n_iterations {
            let enzyme_strand = &strands[rng.gen_range(0..strands.len())];
            let enzymes = Translator::translate(enzyme_strand);
            if enzymes.is_empty() {
                continue;
            }
            let enzyme_idx = rng.gen_range(0..enzymes.len());
            let enzyme = &enzymes[enzyme_idx];
            let rewrite_strand = &strands[rng.gen_range(0..strands.len())];
            let new_strands = Rewriter::rewrite(enzyme, rewrite_strand);
            for strand in new_strands {
                if !known_set.contains(&format!("{:?}", strand)) {
                    strands.push(strand.clone());
                    known_set.insert(format!("{:?}", strand));
                }
            }
        }

        if print_strands {
            println!("Unique strands:");
            let mut sorted_strands: Vec<String> = known_set.clone().into_iter().collect();
            sorted_strands.sort();
            for strand_str in sorted_strands {
                println!("- {}", strand_str);
            }
        }

        println!(
            "Discovered {} unique strands while simulating for {} iterations",
            known_set.len(),
            n_iterations
        );
    }
}
