use lemao_math::vec2::Vec2;

#[derive(Debug)]
pub struct Body {
    pub id: usize,
    pub shape: BodyShape,
    pub position: Vec2,
    pub rotation: f32,
    pub size: Vec2,
    pub mass: f32,
    pub inertia: f32,
    pub velocity_linear: Vec2,
    pub velocity_angular: f32,
    pub bounciness: f32,
    pub friction_static: f32,
    pub friction_dynamic: f32,
    pub dynamic: bool,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BodyShape {
    Box,
    Circle,
}

impl Body {
    pub fn new(shape: BodyShape, position: Vec2, rotation: f32, size: Vec2, mass: f32, dynamic: bool) -> Self {
        let mut body = Self {
            id: 0,
            shape,
            position,
            rotation,
            size,
            mass,
            inertia: 0.0,
            velocity_linear: Default::default(),
            velocity_angular: 0.0,
            bounciness: 0.2,
            friction_static: 0.9,
            friction_dynamic: 0.8,
            dynamic,
        };
        body.update_intertia();
        body
    }

    pub fn update_intertia(&mut self) {
        self.inertia = match self.shape {
            BodyShape::Box => self.mass * (self.size.x.powi(2) + self.size.y.powi(2)) / 12.0,
            BodyShape::Circle => self.mass * self.size.x.powi(2) / 2.0,
        };
    }
}
