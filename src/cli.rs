use std::str::FromStr;

use clap::Subcommand;
use seahash::hash;

#[derive(Subcommand, Debug, Clone)]
pub enum Seeds {
    #[clap(visible_alias = "s")]
    /// Execute the operation for a specified seed
    Single {
        /// The specified seed
        seed: NumberOrString,
    },
    #[clap(visible_aliases = ["r", "rand"])]
    /// Execute the operation for a specified number of random seeds
    Random {
        /// The number of seeds
        count: Option<u64>,
    },
    /// Execute the operation across a specified range of seeds
    Range {
        /// The starting point of the range
        min: u64,
        /// The ending point of the range
        max: u64,
        #[clap(short, long)]
        /// The interval between seeds in the range
        step: Option<u64>,
    },
    /// Execute the operation across the full possible range of seeds
    Full,
}

impl Seeds {
    #[must_use]
    pub fn iter(&self) -> SeedIter {
        <&Self as IntoIterator>::into_iter(self)
    }
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

impl IntoIterator for &Seeds {
    type Item = u64;

    type IntoIter = SeedIter;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Seeds::Single { seed } => match seed {
                NumberOrString::Str(s) => {
                    let seed = hash(s.as_bytes());
                    SeedIter::new(seed, seed, 1, false)
                }
                NumberOrString::Number(n) => SeedIter::new(*n, *n, 1, false),
            },
            Seeds::Range { min, max, step } => SeedIter::new(*min, *max, step.unwrap_or(1), false),
            Seeds::Random { count } => SeedIter::new(1, count.unwrap_or(1), 1, true),
            Seeds::Full => SeedIter::new(u64::MIN, u64::MAX, 1, false),
        }
    }
}

impl IntoIterator for Seeds {
    type Item = u64;

    type IntoIter = SeedIter;

    fn into_iter(self) -> Self::IntoIter {
        (&self).into_iter()
    }
}

impl FromStr for NumberOrString {
    type Err = &'static str; // The actual type doesn't matter since we never error, but it must implement `Display`
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.parse::<u64>().map_or_else(
            |_| NumberOrString::Str(s.to_string()),
            NumberOrString::Number,
        ))
    }
}
