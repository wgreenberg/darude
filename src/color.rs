pub const MAX_CHANNEL: u8 = 255;

#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: f32,
}

pub fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color { r: r, g: g, b: b, a: 1.0 }
}

pub fn rgba(r: u8, g: u8, b: u8, a: f32) -> Color {
    Color { r: r, g: g, b: b, a: a }
}

impl Color {
    pub fn mix(self: &Color, bg: &Color) -> Color {
        Color {
            r: ((1.0 - self.a) * bg.r as f32 + (self.a * self.r as f32)) as u8,
            g: ((1.0 - self.a) * bg.g as f32 + (self.a * self.g as f32)) as u8,
            b: ((1.0 - self.a) * bg.b as f32 + (self.a * self.b as f32)) as u8,
            a: 1.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mix() {
        let red = rgba(255, 0, 0, 0.5);
        let blue = rgb(0, 0, 255);
        let purple = rgb(127, 0, 127);
        assert_eq!(red.mix(&blue), purple);
    }
}
