extern crate rand;

use std::f32;
use shape::rand::{Rng, ThreadRng};

#[derive(PartialEq)]
#[derive(Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(PartialEq)]
#[derive(Debug)]
pub struct Bounds {
    up_left: Point,
    down_right: Point,
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
        if other.up_left.x < self.up_left.x {
            self.up_left.x = other.up_left.x;
        }
        if other.up_left.y > self.up_left.y {
            self.up_left.y = other.up_left.y;
        }
        if other.down_right.x > self.down_right.x {
            self.down_right.x = other.down_right.x;
        }
        if other.down_right.y < self.down_right.y {
            self.down_right.y = other.down_right.y;
        }
    }
}

pub trait Shape {
    // Return a vec of n_points which are equally distributed over the curve of the Shape
    fn as_points(&self, n_points: usize, rng: &mut ThreadRng) -> Vec<Point>;

    // Returns a bounding box for the shape
    fn find_bounds(&self) -> Bounds;
}

pub fn find_maximal_bounds<T: Shape>(shapes: &Vec<T>) -> Bounds {
    let mut max_bounds = Bounds {
        up_left: Point { x: 0.0, y: 0.0 },
        down_right: Point { x: 0.0, y: 0.0 },
    };
    for shape in shapes {
        max_bounds.merge(shape.find_bounds());
    }
    return max_bounds;
}

pub struct Circle {
    center: Point,
    radius: f32,
}

pub fn new_circle(x: f32, y: f32, r: f32) -> Circle {
    Circle {
        center: Point { x: x, y: y },
        radius: r,
    }
}

impl Shape for Circle {
    fn as_points(&self, n_points: usize, rng: &mut ThreadRng) -> Vec<Point> {
        let mut points: Vec<Point> = Vec::new();
        let dr = (2.0 * f32::consts::PI) / (n_points as f32);
        for i in 0..n_points {
            let rot = dr * (i as f32) + rng.gen_range(-dr/2.0, dr/2.0);
            points.push(Point {
                x: self.center.x + self.radius * rot.cos(),
                y: self.center.y + self.radius * rot.sin(),
            });
        }
        return points;
    }

    fn find_bounds(&self) -> Bounds {
        Bounds {
            up_left: Point {
                x: self.center.x - self.radius, 
                y: self.center.y + self.radius,
            },
            down_right: Point {
                x: self.center.x + self.radius,
                y: self.center.y - self.radius,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bounds_merging() {
        let mut b0 = Bounds {
            up_left: Point { x: 0.0, y: 0.0 },
            down_right: Point { x: 0.0, y: 0.0 },
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

    #[test]
    fn test_circle_bounds() {
        let c = new_circle(0.0, 1.0, 1.0);
        assert_eq!(c.find_bounds(), Bounds {
            up_left: Point { x: -1.0, y: 2.0 },
            down_right: Point { x: 1.0, y: 0.0 },
        });
    }
}
