use crate::vec3::Vec3;
use std::ops::Add;
use std::ops::Index;
use std::ops::IndexMut;
use std::ops::Sub;

pub struct Mat4x4 {
    data: [f32; 16],
}

impl Mat4x4 {
    #[rustfmt::skip]
    pub fn identity() -> Self {
        Self { data: [
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0
        ]}
    }

    pub fn ortho(width: f32, height: f32, near: f32, far: f32) -> Self {
        let mut matrix: Mat4x4 = Default::default();
        matrix[0] = 2.0 / width;
        matrix[5] = 2.0 / height;
        matrix[10] = 2.0 / (near - far);
        matrix[15] = 1.0;

        matrix[3] = -1.0;
        matrix[7] = -1.0;
        matrix[11] = (far + near) / (near - far);

        matrix
    }

    pub fn translate(translation: Vec3<f32>) -> Self {
        let mut matrix: Mat4x4 = Mat4x4::identity();
        matrix[3] = translation.x;
        matrix[7] = translation.y;
        matrix[11] = translation.z;

        matrix
    }

    pub fn as_ptr(&self) -> *const f32 {
        self.data.as_ptr() as *const f32
    }
}

impl Default for Mat4x4 {
    #[rustfmt::skip]
    fn default() -> Self {
        Self { data: [
            0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0
        ]}
    }
}

impl Add for Mat4x4 {
    type Output = Self;

    #[rustfmt::skip]
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output { data: [
            self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2], self[3] + rhs[3],
            self[4] + rhs[4], self[5] + rhs[5], self[6] + rhs[6], self[7] + rhs[7],
            self[8] + rhs[8], self[9] + rhs[9], self[10] + rhs[10], self[11] + rhs[11],
            self[12] + rhs[12], self[13] + rhs[13], self[14] + rhs[14], self[15] + rhs[15]
        ]}
    }
}

impl Sub for Mat4x4 {
    type Output = Self;

    #[rustfmt::skip]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output { data: [
            self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2], self[3] - rhs[3],
            self[4] - rhs[4], self[5] - rhs[5], self[6] - rhs[6], self[7] - rhs[7],
            self[8] - rhs[8], self[9] - rhs[9], self[10] - rhs[10], self[11] - rhs[11],
            self[12] - rhs[12], self[13] - rhs[13], self[14] - rhs[14], self[15] - rhs[15]
        ]}
    }
}

impl Index<usize> for Mat4x4 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for Mat4x4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}
