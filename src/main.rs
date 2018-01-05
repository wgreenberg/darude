extern crate darude;
extern crate rand;

use std::io;

use darude::shape::{Point, Shape, Bounds};
use darude::primitives::{Circle, Line};
use darude::color::{rgba, rgb};
use darude::canvas::Canvas;
use rand::ThreadRng;

const HEIGHT: usize = 2000;
const WIDTH:  usize = 2000;

struct Cardioid {
    circle: Circle,
    order: usize,
}

impl Cardioid {
    pub fn new(center: Point, radius: f32, order: usize) -> Cardioid {
        Cardioid {
            circle: Circle { center: center, radius: radius },
            order: order,
        }
    }
}

impl Shape for Cardioid {
    fn as_points(&self, n_points: usize, rng: &mut ThreadRng) -> Vec<Point> {
        let n_lines = 10_000;
        let n_points_per_line = n_points / n_lines;
        let c = &self.circle;

        let mut points: Vec<Point> = Vec::new();
        for p1 in c.as_points(n_lines, rng) {
            let rot = (p1.y - c.center.y).atan2(p1.x - c.center.x);
            let p2 = Point {
                x: c.center.x + c.radius * (rot * self.order as f32).cos(),
                y: c.center.y + c.radius * (rot * self.order as f32).sin(),
            };
            points.append(&mut Line::new(p1, p2).as_points(n_points_per_line, rng));
        }

        return points;
    }

    fn find_bounds(&self) -> Bounds {
        self.circle.find_bounds()
    }
}

fn main() {
    let mut shapes: Vec<Cardioid> = Vec::new();
    let n = 3;
    for i in 0..(n * n) {
        let x = (i % n) as f32;
        let y = (i / n) as f32;
        shapes.push(Cardioid::new(Point { x: x, y: y }, 0.5, i + 2));
    }

    let mut canvas = Canvas::new(HEIGHT, WIDTH, rgb(0x07, 0x36, 0x42));
    eprintln!("rasterizing...");
    canvas.rasterize_shapes(shapes, rgba(0x93, 0xa1, 0xa1, 0.05), 10_000_000);
    eprintln!("writing...");
    canvas.write_as_ppm(&mut io::stdout());
}
