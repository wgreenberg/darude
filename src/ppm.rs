use std::io;

use color;

pub struct PPMFile {
    pub buf: Vec<color::Color>,
    pub height: usize,
    pub width: usize,
    pub max: u8,
}

impl PPMFile {
    pub fn write_to_file(&self, file: &mut io::Write) {
        write!(file, "P3\n").expect("could not write");
        write!(file, "{} {}\n", self.width, self.height).expect("could not write");
        write!(file, "{}\n", self.max).expect("could not write");
        for (i, p) in self.buf.iter().enumerate() {
            write!(file, "{} {} {}", p.r, p.g, p.b).expect("could not write");
            if i == self.width - 1 {
                write!(file, "\n").expect("could not write");
            } else {
                write!(file, " ").expect("could not write");
            }
        }
        file.flush().expect("could not flush");
    }
}
