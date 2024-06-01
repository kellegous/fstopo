use rand::{rngs::SmallRng, Rng, SeedableRng};
use std::{num::ParseIntError, str::FromStr};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Seed {
    v: u64,
}

impl Default for Seed {
    fn default() -> Self {
        let mut rng = SmallRng::from_entropy();
        Self { v: rng.gen() }
    }
}

impl FromStr for Seed {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        u64::from_str_radix(s, 16).map(Seed::new)
    }
}

impl std::fmt::Display for Seed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:016x}", self.v)
    }
}

impl Seed {
    pub fn new(v: u64) -> Self {
        Self { v }
    }

    pub fn from_arg(s: &str) -> Result<Seed, String> {
        Self::from_str(s).map_err(|_| format!("invalid seed: {}", s))
    }

    pub fn value(&self) -> u64 {
        self.v
    }

    pub fn rng(&self) -> SmallRng {
        SmallRng::seed_from_u64(self.v)
    }
}
