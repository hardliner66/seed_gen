mod cli;
use cli::Cli;

fn main() {
    let args = Cli::get_args();

    for seed in args.seeds {
        println!("{seed:?}");
    }
}
