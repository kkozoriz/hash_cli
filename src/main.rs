use clap::Parser;
use hash_cli::found_hash;

#[derive(Parser, Debug)]
#[command(name = "hash-cli")]
#[command(version = "0.1.0")]
#[command(about = "Command line arguments for hash-cli", long_about = None)]
pub struct Args {
    #[arg(short = 'N', default_value_t = 0)]
    zero_count: usize,

    #[arg(short = 'F', default_value_t = 1)]
    hash_count: usize,
}

struct Cli {
    args: Args,
}

impl Cli {
    fn new() -> Cli {
        Cli {
            args: Args::parse(),
        }
    }
    fn run(self) {
        found_hash(self.args.zero_count, self.args.hash_count)
    }
}

fn main() {
    Cli::new().run()
}
