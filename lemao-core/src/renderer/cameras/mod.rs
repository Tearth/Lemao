use lemao_math::mat4x4::Mat4x4;
use lemao_math::vec2::Vec2;
use lemao_math::vec3::Vec3;

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

    pub(crate) fn get_projection_matrix(&self) -> Mat4x4 {
        Mat4x4::ortho(self.size.x, self.size.y, 0.1, 100.0)
    }

    pub(crate) fn get_view_matrix(&self) -> Mat4x4 {
        Mat4x4::translate(Vec3::new(-self.position.x, -self.position.y, -1.0))
    }
}
