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

#[cfg(test)]
mod cli_tests {
    use crate::cli::{value_in_range, Args};

    #[test]
    fn verify_cli_args() {
        use clap::CommandFactory;
        Args::command().debug_assert()
    }

    #[test]
    fn test_value_in_range_valid() {
        assert_eq!(value_in_range("9"), Ok(9));
    }

    #[test]
    fn test_value_in_range_lower_bound() {
        assert_eq!(value_in_range("1"), Ok(1));
    }

    #[test]
    fn test_value_in_range_upper_bound() {
        assert_eq!(value_in_range("10"), Ok(10));
    }

    #[test]
    fn test_value_in_range_below_range() {
        assert_eq!(
            value_in_range("0"),
            Err("value not in range 1-10".to_string())
        );
    }

    #[test]
    fn test_value_in_range_above_range() {
        assert_eq!(
            value_in_range("101"),
            Err("value not in range 1-10".to_string())
        );
    }

    #[test]
    fn test_value_in_range_non_numeric() {
        assert_eq!(
            value_in_range("abc"),
            Err("`abc` isn't a number".to_string())
        );
    }

    #[test]
    fn test_value_in_range_empty_string() {
        assert_eq!(value_in_range(""), Err("`` isn't a number".to_string()));
    }

    #[test]
    fn test_value_in_range_negative_number() {
        assert_eq!(value_in_range("-5"), Err("`-5` isn't a number".to_string()));
    }
}
