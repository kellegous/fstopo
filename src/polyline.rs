use std::{error::Error, str::FromStr};

use cairo::Context;

use super::Point;
pub struct Polyline {
    pts: Vec<Point>,
}

impl Polyline {
    pub fn draw(&self, ctx: &Context) {
        for (i, p) in self.pts.iter().enumerate() {
            if i == 0 {
                ctx.move_to(p.x(), p.y());
            } else {
                ctx.line_to(p.x(), p.y());
            }
        }
    }

    pub fn transform<F>(&mut self, tx: F)
    where
        F: Fn(&Point) -> Point,
    {
        for p in self.pts.iter_mut() {
            *p = tx(p);
        }
    }

    pub fn len(&self) -> usize {
        self.pts.len()
    }

    pub fn is_empty(&self) -> bool {
        self.pts.is_empty()
    }
}
impl FromStr for Polyline {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pts = Vec::new();

        let mut elems = s.split_whitespace();
        while let Some(cmd) = elems.next() {
            match cmd {
                "M" | "L" => {
                    let p = Point::from_xy(
                        elems.next().ok_or("no x")?.parse()?,
                        elems.next().ok_or("no y")?.parse()?,
                    );
                    if let Some(c) = pts.last() {
                        if p.distance_to(c) < 0.0001 {
                            continue;
                        }
                    }
                    pts.push(p);
                }
                _ => return Err(format!("unknown command: {}", cmd).into()),
            }
        }

        Ok(Polyline { pts })
    }
}
