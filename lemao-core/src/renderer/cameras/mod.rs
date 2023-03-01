use lemao_math::mat4x4::Mat4x4;
use lemao_math::vec2::Vec2;
use lemao_math::vec3::Vec3;

pub struct Camera {
    pub id: usize,

    position: Vec2,
    size: Vec2,
    dirty: bool,
}

impl Camera {
    pub fn new(position: Vec2, size: Vec2) -> Self {
        Self { id: 0, position, size, dirty: false }
    }

    pub fn get_position(&self) -> Vec2 {
        self.position
    }

    pub fn set_position(&mut self, position: Vec2) {
        self.position = position;
        self.dirty = true;
    }

    pub fn move_delta(&mut self, delta: Vec2) {
        self.position += delta;
        self.dirty = true;
    }

    pub fn get_size(&self) -> Vec2 {
        self.size
    }

    pub fn set_size(&mut self, size: Vec2) {
        self.size = size;
        self.dirty = true;
    }

    pub(crate) fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub(crate) fn set_dirty_flag(&mut self, value: bool) {
        self.dirty = value;
    }

    pub(crate) fn get_projection_matrix(&self) -> Mat4x4 {
        Mat4x4::ortho(self.size.x, self.size.y, 0.1, 100.0)
    }

    pub(crate) fn get_view_matrix(&self) -> Mat4x4 {
        Mat4x4::translate(Vec3::new(-self.position.x, -self.position.y, -1.0))
    }
}
