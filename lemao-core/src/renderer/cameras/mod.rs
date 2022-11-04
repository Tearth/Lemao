use lemao_math::mat4x4::Mat4x4;
use lemao_math::vec2::Vec2;
use lemao_math::vec3::Vec3;

pub mod storage;

pub struct Camera {
    pub id: usize,
    pub position: Vec2,
    pub size: Vec2,

    pub dirty: bool,
}

impl Camera {
    pub fn new(position: Vec2, size: Vec2) -> Self {
        Self { id: 0, position, size, dirty: false }
    }

    pub fn get_projection_matrix(&self) -> Mat4x4 {
        Mat4x4::ortho(self.size.x, self.size.y, 0.1, 100.0)
    }

    pub fn get_view_matrix(&self) -> Mat4x4 {
        Mat4x4::translate(Vec3::new(-self.position.x, -self.position.y, -1.0))
    }

    pub fn get_position(&self) -> Vec2 {
        self.position
    }

    pub fn set_position(&mut self, position: Vec2) {
        self.position = position;
        self.dirty = true;
    }

    pub fn move_toward(&mut self, delta: Vec2) {
        self.position += delta;
        self.dirty = true;
    }

    pub fn set_viewport(&mut self, size: Vec2) {
        self.size = size;
        self.dirty = true;
    }
}
