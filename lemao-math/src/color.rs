use std::ops::Add;
use std::ops::Sub;

pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Color { r, g, b, a }
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            r: (self.r + rhs.r).clamp(0.0, 1.0),
            g: (self.g + rhs.g).clamp(0.0, 1.0),
            b: (self.b + rhs.b).clamp(0.0, 1.0),
            a: (self.a + rhs.a).clamp(0.0, 1.0),
        }
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            r: (self.r - rhs.r).clamp(0.0, 1.0),
            g: (self.g - rhs.g).clamp(0.0, 1.0),
            b: (self.b - rhs.b).clamp(0.0, 1.0),
            a: (self.a - rhs.a).clamp(0.0, 1.0),
        }
    }
}
