use std::ops::{Add, Mul};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Add for Color {
    type Output = Color;

    fn add(self, oth: Color) -> Color {
        Color {
            r: self.r + oth.r,
            g: self.g + oth.g,
            b: self.b + oth.b,
        }
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, oth: Color) -> Color {
        Color {
            r: self.r * oth.r,
            g: self.g * oth.g,
            b: self.b * oth.b,
        }
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, oth: f32) -> Color {
        Color {
            r: self.r * oth,
            g: self.g * oth,
            b: self.b * oth,
        }
    }
}
