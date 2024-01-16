use rand_chacha::{rand_core::SeedableRng, ChaCha8Rng};
use typogenetics::search::SearchAlgos;
use typogenetics::typogenetics::{Enzyme, Rewriter, Strand, Translator};

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
    },

    /// Rewrite a strand using an enzyme
    Rewrite {
        /// Enzyme to apply to strand
        enzyme_str: String,

        /// Strand to rewrite with enzyme
        strand_str: String,
    },

    /// Simulate generations of enzyme application
    Simulate {
        /// Initial strand to start the simulation
        init_strand_str: String,

        /// Number of iterations to simulate
        #[arg(long = "iter", default_value_t = 100_000)]
        n_iterations: i32,

        /// Random seed
        #[arg(long)]
        seed: Option<i32>,

        /// Whether to print all discovered strands at the end of simulation
        #[arg(long, default_value_t = false)]
        print_strands: bool,
    },
}

fn main() {
    env_logger::init();

    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Translate { strand_str }) => {
            let strand = Strand::from_string(strand_str);
            let enzymes = Translator::translate(&strand);
            for enzyme in enzymes {
                println!("{}", enzyme);
            }
        }
        Some(Commands::Rewrite {
            enzyme_str,
            strand_str,
        }) => {
            let enzyme = Enzyme::from_string(enzyme_str);
            let strand = Strand::from_string(strand_str);
            let new_strands = Rewriter::rewrite(&enzyme, &strand);
            for new_strand in new_strands {
                println!("{}", new_strand);
            }
        }
        Some(Commands::Simulate {
            init_strand_str,
            n_iterations,
            seed: _,
            print_strands,
        }) => {
            let mut rng = ChaCha8Rng::seed_from_u64(2);
            let init_strand = Strand::from_string(init_strand_str);
            SearchAlgos::random(&init_strand, *n_iterations, &mut rng, *print_strands);
        }
        None => {
            panic!("No command provided")
        }
    }
}
