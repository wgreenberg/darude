extern crate rand;

use std::f32;
use shape::rand::{ThreadRng};

#[derive(PartialEq)]
#[derive(Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub const ORIGIN: Point = Point { x: 0.0, y: 0.0 };
}

#[derive(PartialEq)]
#[derive(Debug)]
pub struct Bounds {
    pub up_left: Point,
    pub down_right: Point,
}

impl Bounds {
    pub fn center(&self) -> Point {
        Point {
            x: (self.up_left.x + self.down_right.x) / 2.0,
            y: (self.up_left.y + self.down_right.y) / 2.0,
        }
    }

    pub fn height(&self) -> f32 {
        self.up_left.y - self.down_right.y
    }

    pub fn width(&self) -> f32 {
        self.down_right.x - self.up_left.x
    }

    pub fn merge(&mut self, other: Bounds) {
        self.up_left.x = self.up_left.x.min(other.up_left.x);
        self.up_left.y = self.up_left.y.max(other.up_left.y);
        self.down_right.x = self.down_right.x.max(other.down_right.x);
        self.down_right.y = self.down_right.y.min(other.down_right.y);
    }
}

pub trait Shape {
    // Return a vec of n_points which are equally distributed over the curve of the Shape
    fn as_points(&self, n_points: usize, rng: &mut ThreadRng) -> Vec<Point>;

    // Returns a bounding box for the shape
    fn find_bounds(&self) -> Bounds;
}

pub fn find_maximal_bounds<T: Shape>(shapes: &Vec<T>) -> Bounds {
    let empty_bounds = Bounds { up_left: Point::ORIGIN, down_right: Point::ORIGIN };
    return shapes.iter().fold(empty_bounds, |mut acc, shape| {
        acc.merge(shape.find_bounds());
        acc
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bounds_merging() {
        let mut b0 = Bounds {
            up_left: Point::ORIGIN,
            down_right: Point::ORIGIN,
        };
        let b1 = Bounds {
            up_left: Point { x: -10.0, y: 10.0 },
            down_right: Point { x: 10.0, y: -10.0 },
        };
        let b2 = Bounds {
            up_left: Point { x: -100.0, y: 1.0 },
            down_right: Point { x: 5.0, y: -100.0 },
        };

        b0.merge(b1);
        b0.merge(b2);

        assert_eq!(b0, Bounds {
            up_left: Point { x: -100.0, y: 10.0 },
            down_right: Point { x: 10.0, y: -100.0 },
        });
    }

    #[test]
    fn test_bounds_getters() {
        let b = Bounds {
            up_left: Point { x: -1.0, y: 1.0 }, 
            down_right: Point { x: 1.0, y: 0.0 }, 
        };
        assert_eq!(b.center(), Point { x: 0.0, y: 0.5 });
        assert_eq!(b.height(), 1.0);
        assert_eq!(b.width(), 2.0);
    }
}
