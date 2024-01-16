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
use crate::{enzyme::Enzyme, rewriter::Rewriter, strand::Strand, translator::Translator};

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Translate a strand into a list of enzymes
    Translate {
        /// Strand to translate
        strand_str: String,

        /// Whether to emit debug logs
        #[arg(long, default_value_t = false)]
        debug: bool,
    },

    /// Rewrite a strand using an enzyme
    Rewrite {
        /// Enzyme to apply to strand
        enzyme_str: String,

        /// Strand to rewrite with enzyme
        strand_str: String,

        /// Whether to emit debug logs
        #[arg(long, default_value_t = false)]
        debug: bool,
    },

    /// Simulate generations of enzyme application
    Simulate {
        /// Initial strand to start the simulation
        init_strand_str: String,

        /// Number of iterations to simulate
        #[arg(long, default_value_t = 100_000)]
        n_iterations: i32,

        /// Random seed
        #[arg(long)]
        random_state: Option<i32>,

        /// Whether to emit debug logs
        #[arg(long, default_value_t = false)]
        debug: bool,

        /// Whether to print all discovered strands at the end of simulation
        #[arg(long, default_value_t = false)]
        print_strands: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Translate {
            strand_str,
            debug: _,
        }) => {
            let strand = Strand::from_string(strand_str);
            let enzymes = Translator::translate(&strand);
            for enzyme in enzymes {
                println!("{}", enzyme);
            }
        }
        Some(Commands::Rewrite {
            enzyme_str,
            strand_str,
            debug: _,
        }) => {
            let enzyme = Enzyme::from_string(enzyme_str);
            let strand = Strand::from_string(strand_str);
            let new_strands = Rewriter::rewrite(&enzyme, &strand);
            for new_strand in new_strands {
                println!("{}", new_strand);
            }
        }
        Some(Commands::Simulate {
            init_strand_str: _,
            n_iterations: _,
            random_state,
            debug: _,
            print_strands: _,
        }) => {
            println!("{:?}", random_state)
            // todo!("Simulate command not implemented")
        }
        None => {
            println!("Default subcommand");
        }
    }
}
