use std::{error::Error, fmt, str::FromStr};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Size {
    w: f64,
    h: f64,
}

impl Size {
    pub fn new(w: f64, h: f64) -> Self {
        Self { w, h }
    }

    pub fn from_arg(s: &str) -> Result<Size, String> {
        Size::from_str(s).map_err(|_| format!("invalid size: {}", s))
    }

    pub fn width(&self) -> f64 {
        self.w
    }

    pub fn height(&self) -> f64 {
        self.h
    }
}

impl FromStr for Size {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.find('x') {
            Some(ix) => {
                let w = s[..ix].parse()?;
                let h = s[ix + 1..].parse()?;
                Ok(Size { w, h })
            }
            _ => {
                let w = s.parse()?;
                Ok(Size { w, h: w })
            }
        }
    }
}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}x{}", self.w, self.h)
    }
}
