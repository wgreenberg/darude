use std::io;

mod ppm;
mod color;

const SCREEN_HEIGHT: usize = 600;
const SCREEN_WIDTH:  usize = 800;

fn main() {
    let mut buf: Vec<color::Color> = Vec::new();
    for _ in 0..(SCREEN_WIDTH * SCREEN_HEIGHT) {
        buf.push(color::rgb(128, 0, 0));
    }

    let im = ppm::PPMFile {
        height: SCREEN_HEIGHT,
        width: SCREEN_WIDTH,
        max: 255,
        buf: buf,
    };
    im.write_to_file(&mut io::stdout());
}
