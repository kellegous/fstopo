use std::str::FromStr;

use super::Point;

pub struct Rect {
    top_left: Point,
    bottom_right: Point,
}

impl Rect {
    pub fn new(top_left: Point, bottom_right: Point) -> Self {
        Self {
            top_left,
            bottom_right,
        }
    }

    fn from_xywh(x: f64, y: f64, w: f64, h: f64) -> Self {
        Self {
            top_left: Point::from_xy(x, y),
            bottom_right: Point::from_xy(x + w, y + h),
        }
    }

    pub fn x(&self) -> f64 {
        self.top_left.x()
    }

    pub fn y(&self) -> f64 {
        self.top_left.y()
    }

    pub fn width(&self) -> f64 {
        self.bottom_right.x() - self.top_left.x()
    }

    pub fn height(&self) -> f64 {
        self.bottom_right.y() - self.top_left.y()
    }

    pub fn intersects(&self, other: &Rect) -> bool {
        self.top_left.x() < other.bottom_right.x()
            && self.bottom_right.x() > other.top_left.x()
            && self.top_left.y() < other.bottom_right.y()
            && self.bottom_right.y() > other.top_left.y()
    }
}

impl FromStr for Rect {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let x = parts.next().ok_or("no x")?.parse()?;
        let y = parts.next().ok_or("no y")?.parse()?;
        let w = parts.next().ok_or("no w")?.parse()?;
        let h = parts.next().ok_or("no h")?.parse()?;
        Ok(Rect::from_xywh(x, y, w, h))
    }
}
