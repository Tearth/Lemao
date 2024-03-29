use std::f32::consts::PI;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Sub;
use std::ops::SubAssign;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn new_from_angle(angle: f32) -> Self {
        Self { x: (PI / 2.0 + angle).cos(), y: (PI / 2.0 + angle).sin() }
    }

    pub fn length(&self) -> f32 {
        (self.x.powf(2.0) + self.y.powf(2.0)).sqrt()
    }

    pub fn normalized(&self) -> Self {
        let length = self.length();
        if length == 0.0 {
            return Vec2::new(0.0, 0.0);
        }

        Vec2::new(self.x / length, self.y / length)
    }

    pub fn abs(&self) -> Self {
        Vec2::new(self.x.abs(), self.y.abs())
    }

    pub fn sign(&self) -> Self {
        Vec2::new(self.x.signum(), self.y.signum())
    }

    pub fn dot(&self, rhs: Vec2) -> f32 {
        self.x * rhs.x + self.y * rhs.y
    }

    pub fn signed_angle(&self, rhs: Vec2) -> f32 {
        // https://stackoverflow.com/a/16544330
        // https://en.wikipedia.org/wiki/Atan2
        // rhs.y.atan2(rhs.x) - self.y.atan2(self.x)

        let dot = self.x * rhs.x + self.y * rhs.y;
        let det = self.x * rhs.y - self.y * rhs.x;

        det.atan2(dot)
    }

    pub fn distance(&self, rhs: Vec2) -> f32 {
        ((self.x - rhs.x).powi(2) + (self.y - rhs.y).powi(2)).sqrt()
    }

    pub fn floor(&self) -> Vec2 {
        Vec2::new(self.x.floor(), self.y.floor())
    }

    pub fn clamp(&self, floor: Vec2, ceil: Vec2) -> Vec2 {
        Vec2::new(self.x.clamp(floor.x, ceil.x), self.y.clamp(floor.y, ceil.y))
    }

    pub fn as_ptr(&self) -> *const f32 {
        self as *const _ as *const f32
    }
}

impl Add for Vec2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Vec2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Mul for Vec2 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output { x: self.x * rhs.x, y: self.y * rhs.y }
    }
}

impl Mul<f32> for Vec2 {
    type Output = Vec2;
    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output { x: self.x * rhs, y: self.y * rhs }
    }
}

impl Mul<Vec2> for f32 {
    type Output = Vec2;
    fn mul(self, rhs: Vec2) -> Self::Output {
        Self::Output { x: self * rhs.x, y: self * rhs.y }
    }
}

impl Div<f32> for Vec2 {
    type Output = Vec2;
    fn div(self, rhs: f32) -> Self::Output {
        Self::Output { x: self.x / rhs, y: self.y / rhs }
    }
}

impl Div<Vec2> for Vec2 {
    type Output = Vec2;
    fn div(self, rhs: Vec2) -> Self::Output {
        Self::Output { x: self.x / rhs.x, y: self.y / rhs.y }
    }
}

impl Neg for Vec2 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::Output { x: -self.x, y: -self.y }
    }
}
