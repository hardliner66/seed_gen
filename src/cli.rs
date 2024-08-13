use std::str::FromStr;

use clap::{Parser, Subcommand};
use seahash::hash;

pub struct Cli;

impl Cli {
    pub fn get_args() -> Args {
        Args::parse()
    }
}

#[derive(Parser, Debug)]
pub struct Args {
    #[command(subcommand)]
    pub seeds: SubCommand,
}

#[derive(Subcommand, Debug)]
pub enum SubCommand {
    #[clap(visible_alias = "s")]
    /// Returns the provided seed. If seed is a string, it is hashed beforehand
    Single {
        /// The specified seed
        seed: NumberOrString,
    },
    #[clap(visible_aliases = ["r", "rand"])]
    /// Returns the specied amount of random seeds
    Random {
        /// The amount of seeds to print
        count: Option<u64>,
    },
    /// Returns numbers in a range
    Range {
        /// The lowest number to print
        min: u64,
        /// The highest number to print
        max: u64,
        #[clap(short, long)]
        /// The step size between two numbers
        step: Option<u64>,
    },
    /// Generates all values from u64::MIN to u64::MAX
    Full,
}

#[derive(Debug, Clone)]
pub enum NumberOrString {
    Str(String),
    Number(u64),
}

pub struct SeedIter {
    current: Option<u64>,
    max: u64,
    step: u64,
    randomize: bool,
}

impl SeedIter {
    fn new(current: u64, max: u64, step: u64, randomize: bool) -> Self {
        Self {
            current: Some(current),
            max,
            step,
            randomize,
        }
    }
}

impl Iterator for SeedIter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current) = self.current {
            let seed = if self.randomize {
                rand::random()
            } else {
                current
            };
            if current == self.max {
                self.current = None;
            } else {
                let _ = self.current.replace(current + self.step);
            }
            Some(seed)
        } else {
            None
        }
    }
}

impl IntoIterator for &SubCommand {
    type Item = u64;

    type IntoIter = SeedIter;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            SubCommand::Single { seed } => match seed {
                NumberOrString::Str(s) => {
                    let seed = hash(s.as_bytes());
                    SeedIter::new(seed, seed, 1, false)
                }
                NumberOrString::Number(n) => SeedIter::new(*n, *n, 1, false),
            },
            SubCommand::Range { min, max, step } => {
                SeedIter::new(*min, *max, step.unwrap_or(1), false)
            }
            SubCommand::Random { count } => SeedIter::new(1, count.unwrap_or(1), 1, true),
            SubCommand::Full => SeedIter::new(u64::MIN, u64::MAX, 1, false),
        }
    }
}

impl IntoIterator for SubCommand {
    type Item = u64;

    type IntoIter = SeedIter;

    fn into_iter(self) -> Self::IntoIter {
        (&self).into_iter()
    }
}

impl FromStr for NumberOrString {
    type Err = &'static str; // The actual type doesn't matter since we never error, but it must implement `Display`
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.parse::<u64>()
            .map(NumberOrString::Number)
            .unwrap_or_else(|_| NumberOrString::Str(s.to_string())))
    }
}
