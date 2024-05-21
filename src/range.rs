use std::{error::Error, fmt, str::FromStr};

#[derive(Debug, Clone)]
pub struct Range {
    r: std::ops::Range<f64>,
}

impl Range {
    pub fn from_arg(s: &str) -> Result<Self, String> {
        s.parse().map_err(|_| format!("invnalid range: {}", s))
    }

    pub fn to_std(&self) -> &std::ops::Range<f64> {
        &self.r
    }
}

impl From<std::ops::Range<f64>> for Range {
    fn from(r: std::ops::Range<f64>) -> Self {
        Self { r }
    }
}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.r.start, self.r.end)
    }
}

impl FromStr for Range {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once('-') {
            Some((a, b)) => Ok(Range {
                r: a.parse()?..b.parse()?,
            }),
            None => Err(format!("invalid range: {}", s).into()),
        }
    }
}
