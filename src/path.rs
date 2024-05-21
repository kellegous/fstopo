use std::{error::Error, str::FromStr};

use cairo::Context;

use crate::{Point, Rect};

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

    pub fn transform<F>(&mut self, tx: F)
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
