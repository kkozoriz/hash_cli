use clap::Parser;
use hash_cli::found_hash;
use std::ops::RangeInclusive;
use std::sync::{Arc, Mutex};

const VALUE_RANGE: RangeInclusive<usize> = 1..=10;

#[derive(Parser, Debug)]
#[command(name = "hash-cli")]
#[command(version = "0.1.0")]
#[command(about = "Command line arguments for hash-cli", long_about = None)]
struct Args {
    /// The number of trailing zeros the hash should end with
    #[arg(short = 'N', default_value_t = 1, value_parser = value_in_range)]
    zero_count: usize,
    /// The number of hashes to find
    #[arg(short = 'F', default_value_t = 1, value_parser = value_in_range)]
    hash_count: usize,
}

pub struct Cli {
    args: Args,
    result: Arc<Mutex<Vec<(u64, String)>>>,
}

impl Cli {
    pub(crate) fn new() -> Cli {
        Cli {
            args: Args::parse(),
            result: Arc::new(Mutex::new(vec![])),
        }
    }
    pub(crate) fn run(&mut self) {
        self.result = found_hash(self.args.zero_count, self.args.hash_count);
    }
}

fn value_in_range(s: &str) -> Result<usize, String> {
    let value: usize = s.parse().map_err(|_| format!("`{s}` isn't a number"))?;

    if VALUE_RANGE.contains(&value) {
        Ok(value)
    } else {
        Err(format!(
            "value not in range {}-{}",
            VALUE_RANGE.start(),
            VALUE_RANGE.end()
        ))
    }
}
