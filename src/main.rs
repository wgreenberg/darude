use std::io;

mod shape;
mod canvas;
mod color;

const HEIGHT: usize = 2000;
const WIDTH:  usize = 2000;

fn main() {
    let mut shapes: Vec<shape::Circle> = Vec::new();
    for i in 0..4 {
        for j in 0..4 {
            shapes.push(shape::new_circle(i as f32, j as f32, 7.0));
        }
    }

    let mut canvas = canvas::Canvas::new(HEIGHT, WIDTH, color::rgb(0, 0, 0));
    canvas.rasterize_shapes(&shapes, color::rgba(255, 128, 255, 1.0));
    canvas.write_as_ppm(&mut io::stdout());
}
