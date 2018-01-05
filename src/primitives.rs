use shape::{Point, Shape, Bounds};
use math::{rlerp, rlerp_point};

use std::f32;
use rand::ThreadRng;

pub struct Line {
    p1: Point,
    p2: Point,
}

impl Line {
    pub fn new(p1: Point, p2: Point) -> Line {
        Line { p1: p1, p2: p2 }
    }
}

impl Shape for Line {
    fn as_points(&self, n_points: usize, rng: &mut ThreadRng) -> Vec<Point> {
        rlerp_point(rng, &self.p1, &self.p2, n_points)
    }
    fn find_bounds(&self) -> Bounds {
        Bounds {
            up_left: Point {
                x: self.p1.x.min(self.p2.x),
                y: self.p1.y.max(self.p2.y),
            },
            down_right: Point {
                x: self.p1.x.max(self.p2.x),
                y: self.p1.y.min(self.p2.y),
            },
        }
    }
}

pub struct Circle {
    pub center: Point,
    pub radius: f32,
}

impl Circle {
    pub fn new(center: Point, r: f32) -> Circle {
        Circle { center: center, radius: r }
    }
}

impl Shape for Circle {
    fn as_points(&self, n_points: usize, rng: &mut ThreadRng) -> Vec<Point> {
        return rlerp(rng, 0.0, 2.0 * f32::consts::PI, n_points)
            .iter()
            .map(|rot| {
                Point { 
                    x: self.center.x + self.radius * rot.cos(),
                    y: self.center.y + self.radius * rot.sin(),
                }
            }).collect();
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
