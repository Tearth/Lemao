use crate::vec3::Vec3;
use crate::vec4::Vec4;
use std::arch::x86_64;
use std::mem;
use std::ops::Index;
use std::ops::IndexMut;
use std::ops::Mul;

#[repr(C)]
#[repr(align(16))]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
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

        matrix[12] = -1.0;
        matrix[13] = -1.0;
        matrix[14] = (far + near) / (near - far);

        matrix
    }

    pub fn translate(translation: Vec3) -> Self {
        let mut matrix: Mat4x4 = Mat4x4::identity();
        matrix[12] = translation.x;
        matrix[13] = translation.y;
        matrix[14] = translation.z;

        matrix
    }

    pub fn rotate(rotation: f32) -> Self {
        let mut matrix: Mat4x4 = Mat4x4::identity();
        matrix[0] = rotation.cos();
        matrix[4] = -rotation.sin();
        matrix[1] = rotation.sin();
        matrix[5] = rotation.cos();

        matrix
    }

    pub fn scale(scale: Vec3) -> Self {
        let mut matrix: Mat4x4 = Mat4x4::identity();
        matrix[0] = scale.x;
        matrix[5] = scale.y;
        matrix[10] = scale.z;

        matrix
    }

    pub fn as_ptr(&self) -> *const f32 {
        self.data.as_ptr() as *const f32
    }

    pub fn as_mut_ptr(&mut self) -> *mut f32 {
        self.data.as_mut_ptr() as *mut f32
    }
}

impl Mul for Mat4x4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        /*
            1x + 2y + 3z   1hx + 2hy + 3hz
            6x + 5y + 4z   6hx + 5hy + 4hz
            7x + 8y + 9z   7hx + 8hy + 9hz

            (1)  + (2)  + (3)    (1)   + (2)   + (3)
            (6)x + (5)y + (4)z   (6)hx + (5)hy + (4)hz
            (7)  + (8)  + (9)    (7)   + (8)   + (9)
        */
        // https://math.stackexchange.com/a/64639
        unsafe {
            let mut matrix = Mat4x4::default();
            let col1 = x86_64::_mm_load_ps(self.as_ptr().add(0));
            let col2 = x86_64::_mm_load_ps(self.as_ptr().add(4));
            let col3 = x86_64::_mm_load_ps(self.as_ptr().add(8));
            let col4 = x86_64::_mm_load_ps(self.as_ptr().add(12));

            for col in 0..4 {
                let col_m128 = x86_64::_mm_load_ps(rhs.as_ptr().add(col * 4));
                let x_m128 = x86_64::_mm_shuffle_ps::<0x00>(col_m128, col_m128);
                let y_m128 = x86_64::_mm_shuffle_ps::<0x55>(col_m128, col_m128);
                let z_m128 = x86_64::_mm_shuffle_ps::<0xaa>(col_m128, col_m128);
                let w_m128 = x86_64::_mm_shuffle_ps::<0xff>(col_m128, col_m128);

                let r1 = x86_64::_mm_mul_ps(x_m128, col1);
                let r2 = x86_64::_mm_add_ps(r1, x86_64::_mm_mul_ps(y_m128, col2));
                let r3 = x86_64::_mm_add_ps(r2, x86_64::_mm_mul_ps(z_m128, col3));
                let r4 = x86_64::_mm_add_ps(r3, x86_64::_mm_mul_ps(w_m128, col4));

                x86_64::_mm_store_ps(matrix.as_mut_ptr().add(col * 4), r4);
            }

            matrix
        }

        /*
        let mut matrix = Mat4x4::default();

        for i in 0..16 {
            let row = i / 4;
            let col = i % 4;
            let mut sum = 0.0;

            for p in 0..4 {
                sum += self[(row * 4) + p] * rhs[col + (p * 4)];
            }

            matrix[i] = sum;
        }

        matrix
        */
    }
}

impl Mul<Vec4> for Mat4x4 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Self::Output {
        // https://math.stackexchange.com/a/64639
        unsafe {
            let col1 = x86_64::_mm_load_ps(self.as_ptr().add(0));
            let col2 = x86_64::_mm_load_ps(self.as_ptr().add(4));
            let col3 = x86_64::_mm_load_ps(self.as_ptr().add(8));
            let col4 = x86_64::_mm_load_ps(self.as_ptr().add(12));

            let rhs_m128 = x86_64::_mm_load_ps(rhs.as_ptr());
            let x_m128 = x86_64::_mm_shuffle_ps::<0x00>(rhs_m128, rhs_m128);
            let y_m128 = x86_64::_mm_shuffle_ps::<0x55>(rhs_m128, rhs_m128);
            let z_m128 = x86_64::_mm_shuffle_ps::<0xaa>(rhs_m128, rhs_m128);
            let w_m128 = x86_64::_mm_shuffle_ps::<0xff>(rhs_m128, rhs_m128);

            let r1 = x86_64::_mm_mul_ps(x_m128, col1);
            let r2 = x86_64::_mm_add_ps(r1, x86_64::_mm_mul_ps(y_m128, col2));
            let r3 = x86_64::_mm_add_ps(r2, x86_64::_mm_mul_ps(z_m128, col3));
            let r4 = x86_64::_mm_add_ps(r3, x86_64::_mm_mul_ps(w_m128, col4));

            mem::transmute(r4)
        }

        /*
        Vec4::new(
            self[(0 * 4) + 0] * rhs.x + self[(0 * 4) + 1] * rhs.y + self[(0 * 4) + 2] * rhs.z + self[(0 * 4) + 3] * rhs.w,
            self[(1 * 4) + 0] * rhs.x + self[(1 * 4) + 1] * rhs.y + self[(1 * 4) + 2] * rhs.z + self[(1 * 4) + 3] * rhs.w,
            self[(2 * 4) + 0] * rhs.x + self[(2 * 4) + 1] * rhs.y + self[(2 * 4) + 2] * rhs.z + self[(2 * 4) + 3] * rhs.w,
            self[(3 * 4) + 0] * rhs.x + self[(3 * 4) + 1] * rhs.y + self[(3 * 4) + 2] * rhs.z + self[(3 * 4) + 3] * rhs.w,
        )
        */
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
