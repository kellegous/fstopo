use std::{error::Error, fs, io, path::Path, str::FromStr};

use byteorder::{BigEndian, ByteOrder};
use lazy_static::lazy_static;
use memmap::{Mmap, MmapOptions};
use rand::{
    distributions::{Distribution, Uniform},
    RngCore,
};
use regex::Regex;

use crate::Color;

const THEME_SIZE: usize = 20;

lazy_static! {
    static ref REF_PATTERN: Regex = Regex::new(r"^(.*):(\d+)$").unwrap();
}

pub struct Themes {
    mem: Mmap,
}

impl Themes {
    pub fn open<P: AsRef<Path>>(src: P) -> io::Result<Self> {
        let f = fs::File::open(src)?;
        Ok(Themes {
            mem: unsafe { MmapOptions::new().map(&f)? },
        })
    }

    pub fn get(&self, idx: usize) -> Vec<Color> {
        let off = idx * THEME_SIZE;
        let mut colors = Vec::with_capacity(5);
        for i in 0..5 {
            let b = off + i * 4;
            colors.push(Color::from_rgb_u32(BigEndian::read_u32(
                &self.mem[b..b + 4],
            )));
        }
        colors
    }

    pub fn pick(&self, rng: &mut dyn RngCore) -> (usize, Vec<Color>) {
        let ix = Uniform::new(0, self.len()).sample(rng);
        (ix, self.get(ix))
    }

    pub fn len(&self) -> usize {
        self.mem.len() / THEME_SIZE
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[derive(Clone, Debug)]
pub struct ThemeRef {
    path: String,
    idx: Option<usize>,
}

impl ThemeRef {
    pub fn from_path(path: &str) -> Self {
        Self {
            path: path.to_string(),
            idx: None,
        }
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn idx(&self) -> Option<usize> {
        self.idx
    }

    pub fn from_arg(s: &str) -> Result<Self, String> {
        Self::from_str(s).map_err(|_| format!("invalid theme: {}", s))
    }

    pub fn pick(&self, rng: &mut dyn RngCore) -> Result<(usize, Vec<Color>), Box<dyn Error>> {
        let themes = Themes::open(self.path())?;
        Ok(match self.idx {
            Some(idx) => (idx, themes.get(idx)),
            None => themes.pick(rng),
        })
    }
}

impl FromStr for ThemeRef {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match REF_PATTERN.captures(s) {
            Some(caps) => {
                let path = caps.get(1).unwrap().as_str();
                let idx = caps.get(2).unwrap().as_str().parse::<usize>()?;

                Ok(ThemeRef {
                    path: path.to_string(),
                    idx: Some(idx),
                })
            }
            None => Ok(ThemeRef {
                path: s.to_string(),
                idx: None,
            }),
        }
    }
}

impl std::fmt::Display for ThemeRef {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.idx {
            Some(idx) => write!(f, "{}[{}]", self.path, idx),
            None => write!(f, "{}", self.path),
        }
    }
}
