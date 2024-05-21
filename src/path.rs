use std::{error::Error, str::FromStr};

use cairo::Context;

pub struct Path {
    cmds: Vec<Cmd>,
}

impl Path {
    pub fn draw(&self, ctx: &Context) {
        for cmd in self.cmds.iter() {
            match cmd {
                Cmd::MoveTo(x, y) => ctx.move_to(*x, *y),
                Cmd::LineTo(x, y) => ctx.line_to(*x, *y),
                Cmd::Close => ctx.close_path(),
            }
        }
    }

    pub fn transform<F>(&mut self, tx: F)
    where
        F: Fn(f64, f64) -> (f64, f64),
    {
        for cmd in self.cmds.iter_mut() {
            match cmd {
                Cmd::MoveTo(x, y) => (*x, *y) = tx(*x, *y),
                Cmd::LineTo(x, y) => (*x, *y) = tx(*x, *y),
                _ => {}
            }
        }
    }

    pub fn len(&self) -> usize {
        self.cmds.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cmds.is_empty()
    }
}

impl FromStr for Path {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cmds = Vec::new();
        let mut elems = s.split_whitespace();
        while let Some(cmd) = elems.next() {
            match cmd {
                "M" => {
                    cmds.push(Cmd::MoveTo(
                        elems.next().ok_or("no x")?.parse()?,
                        elems.next().ok_or("no y")?.parse()?,
                    ));
                }
                "L" => cmds.push(Cmd::LineTo(
                    elems.next().ok_or("no x")?.parse()?,
                    elems.next().ok_or("no y")?.parse()?,
                )),
                _ => return Err(format!("unknown command: {}", cmd).into()),
            }
        }
        Ok(Path { cmds })
    }
}

#[derive(Debug)]
pub enum Cmd {
    MoveTo(f64, f64),
    LineTo(f64, f64),
    Close,
}
