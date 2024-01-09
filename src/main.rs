mod amino_acid;
mod base;
mod base_pair;
mod base_type;
mod duplet;
mod enzyme;
mod folder;
mod orientation;
mod rewriter;
mod strand;
mod translator;
mod turn;
use crate::{
    duplet::Duplet, folder::Folder, rewriter::Rewriter, strand::Strand, translator::Translator,
};

fn main() {
    let strand = Strand::from_string("CATAAGTACCAGAATACCGTCCT");
    println!("{:?}", strand);
    let duplets: Vec<Duplet> = strand.iter_duplets().collect();
    println!("{:?}", duplets);
    let enzymes = Translator::translate(&strand);
    println!("{:?}", enzymes);
    if let Some(enzyme) = enzymes.get(0) {
        let unit = Folder::get_binding_site(enzyme, &strand);
        println!("{:?}", unit);
        let new_strands = Rewriter::rewrite(enzyme, &strand);
        println!("{:?}", new_strands);
    }
}
