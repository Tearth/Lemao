#[repr(C)]
#[repr(align(16))]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    pub fn as_ptr(&self) -> *const f32 {
        self as *const _ as *const f32
    }

    pub fn as_mut_ptr(&mut self) -> *mut f32 {
        self as *mut _ as *mut f32
    }
}
