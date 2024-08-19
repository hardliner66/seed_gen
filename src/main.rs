use clap::Parser;
use seed_gen::cli::Seeds;

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    seeds: Seeds,
}

fn main() {
    let args = Args::parse();

    for seed in args.seeds {
        println!("{seed:?}");
    }
}
