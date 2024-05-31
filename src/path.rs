use std::{error::Error, fmt, str::FromStr};

use cairo::Context;
use serde::{de, ser};

use crate::{Point, Rect};

#[derive(Debug)]
pub struct Path {
    cmds: Vec<Cmd>,
}

impl Path {
    pub fn draw(&self, ctx: &Context) {
        for cmd in self.cmds.iter() {
            match cmd {
                Cmd::MoveTo(p) => ctx.move_to(p.x(), p.y()),
                Cmd::LineTo(p) => ctx.line_to(p.x(), p.y()),
            }
        }
    }

    pub fn transform_into<F>(&mut self, tx: F)
    where
        F: Fn(&Point) -> Point,
    {
        for cmd in self.cmds.iter_mut() {
            match cmd {
                Cmd::MoveTo(p) => *p = tx(p),
                Cmd::LineTo(p) => *p = tx(p),
            }
        }
    }

    pub fn transform<F>(&self, tx: F) -> Path
    where
        F: Fn(&Point) -> Point,
    {
        Path {
            cmds: self
                .cmds
                .iter()
                .map(|cmd| match cmd {
                    Cmd::MoveTo(p) => Cmd::MoveTo(tx(p)),
                    Cmd::LineTo(p) => Cmd::LineTo(tx(p)),
                })
                .collect(),
        }
    }

    pub fn len(&self) -> usize {
        self.cmds.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cmds.is_empty()
    }

    pub fn bounds(&self) -> Rect {
        let mut min_x = f64::INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut max_y = f64::NEG_INFINITY;
        for cmd in self.cmds.iter() {
            let p = cmd.point();
            min_x = min_x.min(p.x());
            min_y = min_y.min(p.y());
            max_x = max_x.max(p.x());
            max_y = max_y.max(p.y());
        }
        Rect::new(Point::from_xy(min_x, min_y), Point::from_xy(max_x, max_y))
    }

    pub fn is_valid(s: &str) -> bool {
        let mut elems = s.split_whitespace();
        while let Some(cmd) = elems.next() {
            match cmd {
                "M" | "L" => {
                    if !has_a::<f64>(&mut elems) || !has_a::<f64>(&mut elems) {
                        return false;
                    }
                }

                _ => return false,
            }
        }
        true
    }
}

fn has_a<'a, T: FromStr>(elems: &mut impl Iterator<Item = &'a str>) -> bool {
    elems.next().and_then(|s| s.parse::<T>().ok()).is_some()
}

impl FromStr for Path {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cmds = Vec::new();
        let mut elems = s.split_whitespace();
        while let Some(cmd) = elems.next() {
            let pt = Point::from_xy(
                elems.next().ok_or("no x")?.parse()?,
                elems.next().ok_or("no y")?.parse()?,
            );
            cmds.push(match cmd {
                "M" => Cmd::MoveTo(pt),
                "L" => Cmd::LineTo(pt),
                _ => return Err(format!("unknown command: {}", cmd).into()),
            });
        }
        Ok(Path { cmds })
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, cmd) in self.cmds.iter().enumerate() {
            if i == 0 {
                write!(f, "{}", cmd)?;
            } else {
                write!(f, " {}", cmd)?;
            }
        }
        Ok(())
    }
}

impl ser::Serialize for Path {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let s = format!("{}", self);
        ser.serialize_str(&s)
    }
}

impl<'d> de::Deserialize<'d> for Path {
    fn deserialize<D>(de: D) -> Result<Path, D::Error>
    where
        D: de::Deserializer<'d>,
    {
        de.deserialize_str(PathVisitor)
    }
}

struct PathVisitor;

impl<'d> de::Visitor<'d> for PathVisitor {
    type Value = Path;
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Path::from_str(v).map_err(|e| E::custom(e.to_string()))
    }

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a string")
    }
}

#[derive(Debug)]
pub enum Cmd {
    MoveTo(Point),
    LineTo(Point),
}

impl Cmd {
    pub fn point(&self) -> &Point {
        match self {
            Cmd::MoveTo(p) => p,
            Cmd::LineTo(p) => p,
        }
    }
}

impl fmt::Display for Cmd {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cmd::MoveTo(p) => write!(f, "M {} {}", p.x(), p.y()),
            Cmd::LineTo(p) => write!(f, "L {} {}", p.x(), p.y()),
        }
    }
}
