extern crate darude;
extern crate rand;

use std::io;
use std::f32;

use darude::shape::{Point, Shape, Bounds};
use darude::color::{rgba, rgb};
use darude::canvas::Canvas;
use darude::math::{rlerp, rlerp_point};

use rand::{ThreadRng, Rng};

const HEIGHT: usize = 2000;
const WIDTH:  usize = 2000;

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
        let n_lines = 10_000;
        let n_cardioid_nodes = rng.gen_range(2, 8);
        let n_points_per_line = n_points / n_lines;

        for rot in rlerp(rng, 0.0, 2.0 * f32::consts::PI, n_lines) {
            // generate a chord along the edge of the circle
            let p1 = Point { 
                x: self.center.x + self.radius * rot.cos(),
                y: self.center.y + self.radius * rot.sin(),
            };
            let p2rot = n_cardioid_nodes as f32 * rot;
            let p2 = Point {
                x: self.center.x + self.radius * p2rot.cos(),
                y: self.center.y + self.radius * p2rot.sin(),
            };

            // generate a series of points along the chord
            points.append(&mut rlerp_point(rng, p1, p2, n_points_per_line));
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

fn main() {
    let mut shapes: Vec<Circle> = Vec::new();
    for i in 0..1 {
        for j in 0..1 {
            shapes.push(new_circle(i as f32, j as f32, 0.5));
        }
    }

    let mut canvas = Canvas::new(HEIGHT, WIDTH, rgb(255, 255, 255));
    canvas.rasterize_shapes(&shapes, rgba(0, 0, 0, 0.25), 1_000_000);
    canvas.write_as_ppm(&mut io::stdout());
}
