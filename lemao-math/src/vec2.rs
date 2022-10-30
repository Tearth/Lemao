use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn as_ptr(&self) -> *const T {
        self as *const _ as *const T
    }
}

impl<T> Default for Vec2<T>
where
    T: Default,
{
    fn default() -> Self {
        Self { x: Default::default(), y: Default::default() }
    }
}

macro_rules! implement {
    ($type:ident) => {
        impl Vec2<$type> {
            pub fn length(&self) -> f64 {
                ((self.x as f64).powf(2.0) + (self.y as f64).powf(2.0)).sqrt()
            }
        }

        impl Add for Vec2<$type> {
            type Output = Self;
            fn add(self, rhs: Self) -> Self::Output {
                Self::Output { x: self.x + rhs.x, y: self.y + rhs.y }
            }
        }

        impl Sub for Vec2<$type> {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self::Output {
                Self::Output { x: self.x - rhs.x, y: self.y - rhs.y }
            }
        }

        impl Mul<$type> for Vec2<$type> {
            type Output = Vec2<$type>;
            fn mul(self, rhs: $type) -> Self::Output {
                Self::Output { x: self.x * rhs, y: self.y * rhs }
            }
        }

        impl Mul<Vec2<$type>> for $type {
            type Output = Vec2<$type>;
            fn mul(self, rhs: Vec2<$type>) -> Self::Output {
                Self::Output { x: self * rhs.x, y: self * rhs.y }
            }
        }

        impl Div<$type> for Vec2<$type> {
            type Output = Vec2<$type>;
            fn div(self, rhs: $type) -> Self::Output {
                Self::Output { x: self.x / rhs, y: self.y / rhs }
            }
        }
    };
}

implement!(u8);
implement!(i8);
implement!(u16);
implement!(i16);
implement!(u32);
implement!(i32);
implement!(f32);
implement!(u64);
implement!(i64);
implement!(f64);
implement!(usize);
implement!(isize);
