use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Sub;
use std::ops::SubAssign;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub fn as_ptr(&self) -> *const T {
        self as *const _ as *const T
    }
}

impl<T> Default for Vec3<T>
where
    T: Default,
{
    fn default() -> Self {
        Self { x: Default::default(), y: Default::default(), z: Default::default() }
    }
}

macro_rules! implement {
    ($type:ident) => {
        impl Vec3<$type> {
            pub fn length(&self) -> f64 {
                ((self.x as f64).powf(2.0) + (self.y as f64).powf(2.0) + (self.z as f64).powf(2.0)).sqrt()
            }
        }

        impl Add for Vec3<$type> {
            type Output = Self;
            fn add(self, rhs: Self) -> Self::Output {
                Self::Output { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
            }
        }

        impl AddAssign for Vec3<$type> {
            fn add_assign(&mut self, rhs: Self) {
                *self = *self + rhs;
            }
        }

        impl Sub for Vec3<$type> {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self::Output {
                Self::Output { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
            }
        }

        impl SubAssign for Vec3<$type> {
            fn sub_assign(&mut self, rhs: Self) {
                *self = *self - rhs;
            }
        }

        impl Mul for Vec3<$type> {
            type Output = Self;
            fn mul(self, rhs: Self) -> Self::Output {
                Self::Output { x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z }
            }
        }

        impl Mul<$type> for Vec3<$type> {
            type Output = Vec3<$type>;
            fn mul(self, rhs: $type) -> Self::Output {
                Self::Output { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
            }
        }

        impl Mul<Vec3<$type>> for $type {
            type Output = Vec3<$type>;
            fn mul(self, rhs: Vec3<$type>) -> Self::Output {
                Self::Output { x: self * rhs.x, y: self * rhs.y, z: self * rhs.z }
            }
        }

        impl Div<$type> for Vec3<$type> {
            type Output = Vec3<$type>;
            fn div(self, rhs: $type) -> Self::Output {
                Self::Output { x: self.x / rhs, y: self.y / rhs, z: self.z / rhs }
            }
        }

        impl Neg for Vec3<$type> {
            type Output = Self;
            fn neg(self) -> Self::Output {
                Self::Output { x: -self.x, y: -self.y, z: -self.z }
            }
        }
    };
}

implement!(i8);
implement!(i16);
implement!(i32);
implement!(f32);
implement!(i64);
implement!(f64);
implement!(isize);
