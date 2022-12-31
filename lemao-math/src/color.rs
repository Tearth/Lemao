use std::ops::Add;
use std::ops::Sub;

#[repr(C)]
#[derive(Copy, Clone, Default, PartialEq)]
pub struct SolidColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl SolidColor {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub fn new_rgb(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r: (r as f32) / 255.0, g: (g as f32) / 255.0, b: (b as f32) / 255.0, a: (a as f32) / 255.0 }
    }

    pub fn as_ptr(&self) -> *const f32 {
        self as *const _ as *const f32
    }
}

impl Add for SolidColor {
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

impl Sub for SolidColor {
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
