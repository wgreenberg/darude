use std::io;
use rayon::prelude::*;
use rand;

use color::{Color, MAX_CHANNEL};
use shape::{Shape, Point, Bounds, find_maximal_bounds};

pub struct Canvas {
    buf: Vec<Color>,
    height: usize,
    width: usize,
    bg: Color,
}

impl Canvas {
    pub fn new(height: usize, width: usize, bg: Color) -> Canvas {
        let mut buf: Vec<Color> = Vec::new();
        buf.resize(height * width, bg.clone());
        Canvas {
            buf: buf,
            height: height,
            width: width,
            bg: bg,
        }
    }

    pub fn write_as_ppm(&self, file: &mut io::Write) {
        write!(file, "P3\n").expect("could not write");
        write!(file, "{} {}\n", self.width, self.height).expect("could not write");
        write!(file, "{}\n", MAX_CHANNEL).expect("could not write");
        for (i, p) in self.buf.iter().enumerate() {
            let b = p.mix(&self.bg);
            write!(file, "{} {} {}", b.r, b.g, b.b).expect("could not write");
            if i == self.width - 1 {
                write!(file, "\n").expect("could not write");
            } else {
                write!(file, " ").expect("could not write");
            }
        }
        file.flush().expect("could not flush");
    }

    fn find_nearest_pixel(&self, p: Point, bounds: &Bounds) -> usize {
        let ratio_i = bounds.width() / bounds.height();
        let ratio_c = self.width as f32 / self.height as f32;
        let scaled_p: Point;
        let ctr = bounds.center();
        if ratio_c > ratio_i {
            scaled_p = Point {
                x: (((p.x - ctr.x) / bounds.width()) + 0.5) * (ratio_i / ratio_c),
                y: ((p.y - ctr.y) / bounds.height()) + 0.5,
            };
        } else {
            scaled_p = Point {
                x: ((p.x - ctr.x) / bounds.width()) + 0.5,
                y: (((p.y - ctr.y) / bounds.height()) + 0.5) * (ratio_c / ratio_i),
            };
        }
        let raster_x = scaled_p.x * (self.width - 1) as f32;
        let raster_y = (1.0 - scaled_p.y) * (self.height - 1) as f32;

        return raster_x as usize + (self.width * raster_y as usize);
    }

    pub fn rasterize_shapes<T: Shape + Send>(&mut self, shapes: Vec<T>, color: Color, n_points: usize) {
        let max_bounds = find_maximal_bounds(&shapes);

        let pixels: Vec<usize> = shapes.into_par_iter()
            .flat_map(|shape| shape.as_points(n_points, &mut rand::thread_rng()))
            .map(|point| self.find_nearest_pixel(point, &max_bounds))
            .collect();

        for i in pixels {
            if i < self.buf.len() {
                self.buf[i] = color.mix(&self.buf[i]);
            }
        }
    }
}
