use crate::body::BodyShape;
use crate::storage::PhysicsStorage;
use lemao_core::renderer::context::RendererContext;
use lemao_core::renderer::drawable::circle::Circle;
use lemao_core::renderer::drawable::frame::Frame;
use lemao_core::renderer::drawable::line::Line;
use lemao_core::renderer::drawable::Color;
use lemao_math::color::SolidColor;
use lemao_math::vec2::Vec2;
use std::f32::consts;

pub struct PhysicsContext {
    pub debug_box: Frame,
    pub debug_circle: Circle,
    pub debug_line: Line,
    pub pixels_per_meter: f32,
    pub bodies: PhysicsStorage,
}

impl PhysicsContext {
    pub fn new(renderer: &mut RendererContext) -> Result<Self, String> {
        let physics = Self {
            debug_circle: renderer.create_circle()?,
            debug_box: renderer.create_frame()?,
            debug_line: renderer.create_line()?,
            pixels_per_meter: 100.0,
            bodies: Default::default(),
        };

        Ok(physics)
    }

    pub fn update(&mut self, delta_time: f32) -> Result<(), String> {
        for i in 0..self.bodies.len() {
            for j in 0..self.bodies.len() {
                if i == j {
                    continue;
                }

                let b1 = self.bodies.get(i)?;
                let b2 = self.bodies.get(j)?;

                if b1.shape == BodyShape::Circle && b2.shape == BodyShape::Circle {
                    let r1 = b1.size.x / 2.0;
                    let r2 = b2.size.y / 2.0;
                    let distance = b1.position.distance(b2.position);
                    let depth = distance - (r1 + r2);

                    if depth < 0.0 {
                        let direction = b2.position - b1.position;
                        let change = direction * depth;
                        let b1_mass_ratio = 1.0 - b1.mass / (b1.mass + b2.mass);
                        let b2_mass_ratio = 1.0 - b2.mass / (b1.mass + b2.mass);

                        let b1_position = b1.position + change * b1_mass_ratio;
                        let b2_position = b2.position - change * b2_mass_ratio;

                        self.bodies.get_mut(i)?.position = b1_position;
                        self.bodies.get_mut(j)?.position = b2_position;
                    }
                } else if b1.shape == BodyShape::Box && b2.shape == BodyShape::Box {
                    let b1_v1 = Vec2::new(b1.size.x, b1.size.y) / 2.0;
                    let b1_v2 = Vec2::new(-b1.size.x, b1.size.y) / 2.0;
                    let b1_v3 = Vec2::new(-b1.size.x, -b1.size.y) / 2.0;
                    let b1_v4 = Vec2::new(b1.size.x, -b1.size.y) / 2.0;

                    let angle = Vec2::new(b1.rotation.cos(), b1.rotation.sin());
                    let b1_v1 = b1.position + Vec2::new(b1_v1.x * angle.x - b1_v1.y * angle.y, b1_v1.x * angle.y + b1_v1.y * angle.x);
                    let b1_v2 = b1.position + Vec2::new(b1_v2.x * angle.x - b1_v2.y * angle.y, b1_v2.x * angle.y + b1_v2.y * angle.x);
                    let b1_v3 = b1.position + Vec2::new(b1_v3.x * angle.x - b1_v3.y * angle.y, b1_v3.x * angle.y + b1_v3.y * angle.x);
                    let b1_v4 = b1.position + Vec2::new(b1_v4.x * angle.x - b1_v4.y * angle.y, b1_v4.x * angle.y + b1_v4.y * angle.x);
                    let b1_vertices = [b1_v1, b1_v2, b1_v3, b1_v4];

                    let b2_v1 = Vec2::new(b2.size.x, b2.size.y) / 2.0;
                    let b2_v2 = Vec2::new(-b2.size.x, b2.size.y) / 2.0;
                    let b2_v3 = Vec2::new(-b2.size.x, -b2.size.y) / 2.0;
                    let b2_v4 = Vec2::new(b2.size.x, -b2.size.y) / 2.0;

                    let angle = Vec2::new(b2.rotation.cos(), b2.rotation.sin());
                    let b2_v1 = b2.position + Vec2::new(b2_v1.x * angle.x - b2_v1.y * angle.y, b2_v1.x * angle.y + b2_v1.y * angle.x);
                    let b2_v2 = b2.position + Vec2::new(b2_v2.x * angle.x - b2_v2.y * angle.y, b2_v2.x * angle.y + b2_v2.y * angle.x);
                    let b2_v3 = b2.position + Vec2::new(b2_v3.x * angle.x - b2_v3.y * angle.y, b2_v3.x * angle.y + b2_v3.y * angle.x);
                    let b2_v4 = b2.position + Vec2::new(b2_v4.x * angle.x - b2_v4.y * angle.y, b2_v4.x * angle.y + b2_v4.y * angle.x);
                    let b2_vertices = [b2_v1, b2_v2, b2_v3, b2_v4];

                    let mut collision_detected = true;
                    let mut collision_depth = f32::MAX;
                    let mut collision_axis = Vec2::default();

                    let mut invert_depth = false;
                    for [v1, v2] in [[b1_v1, b1_v2], [b1_v2, b1_v3], [b2_v1, b2_v2], [b2_v2, b2_v3]] {
                        let edge = v2 - v1;
                        let axis = Vec2::new(-edge.y, edge.x).normalized();

                        let mut b1_min = f32::MAX;
                        let mut b1_max = f32::MIN;

                        for v in b1_vertices {
                            let projection = v.dot(axis);
                            b1_min = b1_min.min(projection);
                            b1_max = b1_max.max(projection);
                        }

                        let mut b2_min = f32::MAX;
                        let mut b2_max = f32::MIN;

                        for v in b2_vertices {
                            let projection = v.dot(axis);
                            b2_min = b2_min.min(projection);
                            b2_max = b2_max.max(projection);
                        }

                        if b1_min >= b2_max || b2_min >= b1_max {
                            collision_detected = false;
                            break;
                        }

                        let d1 = b1_max - b2_min;
                        let d2 = b2_max - b1_min;
                        let axis_depth = d1.min(d2);

                        if axis_depth < collision_depth {
                            collision_depth = axis_depth;
                            collision_axis = axis;
                            invert_depth = d1 < d2;
                        }
                    }

                    if collision_detected {
                        let fixed_collision_axis = collision_axis;

                        if invert_depth {
                            collision_depth = -collision_depth;
                        }

                        let change = fixed_collision_axis * collision_depth;
                        let b1_mass_ratio = 1.0 - b1.mass / (b1.mass + b2.mass);
                        let b2_mass_ratio = 1.0 - b2.mass / (b1.mass + b2.mass);

                        let b1_position = b1.position + change * b1_mass_ratio;
                        let b2_position = b2.position - change * b2_mass_ratio;

                        self.bodies.get_mut(i)?.position = b1_position;
                        self.bodies.get_mut(j)?.position = b2_position;
                    }
                } else if b1.shape == BodyShape::Circle && b2.shape == BodyShape::Box {
                    let b2_v1 = Vec2::new(b2.size.x, b2.size.y) / 2.0;
                    let b2_v2 = Vec2::new(-b2.size.x, b2.size.y) / 2.0;
                    let b2_v3 = Vec2::new(-b2.size.x, -b2.size.y) / 2.0;
                    let b2_v4 = Vec2::new(b2.size.x, -b2.size.y) / 2.0;

                    let angle = Vec2::new(b2.rotation.cos(), b2.rotation.sin());
                    let b2_v1 = b2.position + Vec2::new(b2_v1.x * angle.x - b2_v1.y * angle.y, b2_v1.x * angle.y + b2_v1.y * angle.x);
                    let b2_v2 = b2.position + Vec2::new(b2_v2.x * angle.x - b2_v2.y * angle.y, b2_v2.x * angle.y + b2_v2.y * angle.x);
                    let b2_v3 = b2.position + Vec2::new(b2_v3.x * angle.x - b2_v3.y * angle.y, b2_v3.x * angle.y + b2_v3.y * angle.x);
                    let b2_v4 = b2.position + Vec2::new(b2_v4.x * angle.x - b2_v4.y * angle.y, b2_v4.x * angle.y + b2_v4.y * angle.x);
                    let b2_vertices = [b2_v1, b2_v2, b2_v3, b2_v4];

                    let mut collision_detected = true;
                    let mut collision_depth = f32::MAX;
                    let mut collision_axis = Vec2::default();

                    let mut axes = Vec::new();

                    for [v1, v2] in [[b2_v1, b2_v2], [b2_v2, b2_v3]] {
                        let edge = v2 - v1;
                        let axis = Vec2::new(edge.y, -edge.x).normalized();

                        axes.push(axis);
                    }

                    let mut min_dist = f32::MAX;
                    let mut nearest_vertice = Vec2::default();

                    for v in b2_vertices {
                        let dist = v.distance(b1.position);
                        if dist < min_dist {
                            min_dist = dist;
                            nearest_vertice = v;
                        }
                    }

                    axes.push((nearest_vertice - b1.position).normalized());

                    let mut invert_depth = false;
                    for axis in axes {
                        let b1_min = (b1.position - axis * b1.size.x / 2.0).dot(axis);
                        let b1_max = (b1.position + axis * b1.size.x / 2.0).dot(axis);

                        let mut b2_min = f32::MAX;
                        let mut b2_max = f32::MIN;

                        for v in b2_vertices {
                            let projection = v.dot(axis);
                            b2_min = b2_min.min(projection);
                            b2_max = b2_max.max(projection);
                        }

                        if b1_min >= b2_max || b2_min >= b1_max {
                            collision_detected = false;
                            break;
                        }

                        let d1 = b1_max - b2_min;
                        let d2 = b2_max - b1_min;
                        let axis_depth = d1.min(d2);

                        if axis_depth < collision_depth {
                            collision_depth = axis_depth;
                            collision_axis = axis;
                            invert_depth = d1 < d2;
                        }
                    }

                    if collision_detected {
                        let fixed_collision_axis = collision_axis;

                        if invert_depth {
                            collision_depth = -collision_depth;
                        }

                        let change = fixed_collision_axis * collision_depth;
                        let b1_mass_ratio = 1.0 - b1.mass / (b1.mass + b2.mass);
                        let b2_mass_ratio = 1.0 - b2.mass / (b1.mass + b2.mass);

                        let b1_position = b1.position + change * b1_mass_ratio;
                        let b2_position = b2.position - change * b2_mass_ratio;

                        self.bodies.get_mut(i)?.position = b1_position;
                        self.bodies.get_mut(j)?.position = b2_position;
                    }
                }
            }
        }

        Ok(())
    }

    pub fn draw(&mut self, renderer: &mut RendererContext) -> Result<(), String> {
        for body in self.bodies.iter() {
            match body.shape {
                BodyShape::Box => {
                    self.debug_box.position = body.position * self.pixels_per_meter;
                    self.debug_box.rotation = body.rotation;
                    self.debug_box.size = body.size * self.pixels_per_meter;
                    self.debug_box.anchor = Vec2::new(0.5, 0.5);
                    self.debug_box.color = Color::SolidColor(SolidColor::new(1.0, 1.0, 1.0, 1.0));
                    self.debug_box.update();

                    self.debug_line.from = body.position * self.pixels_per_meter;
                    self.debug_line.to = (body.position + (Vec2::new_from_angle(body.rotation) * body.size.y / 2.0)) * self.pixels_per_meter;
                    self.debug_line.color = Color::SolidColor(SolidColor::new(1.0, 1.0, 1.0, 1.0));
                    self.debug_line.update();

                    renderer.draw(&mut self.debug_box)?;
                    renderer.draw(&mut self.debug_line)?;
                }
                BodyShape::Circle => {
                    self.debug_circle.position = body.position * self.pixels_per_meter;
                    self.debug_circle.rotation = body.rotation;
                    self.debug_circle.size = body.size * self.pixels_per_meter;
                    self.debug_circle.anchor = Vec2::new(0.5, 0.5);
                    self.debug_circle.color = Color::SolidColor(SolidColor::new(1.0, 1.0, 1.0, 1.0));
                    self.debug_circle.update();

                    self.debug_line.from = body.position * self.pixels_per_meter;
                    self.debug_line.to = (body.position + (Vec2::new_from_angle(body.rotation) * body.size.y / 2.0)) * self.pixels_per_meter;
                    self.debug_line.color = Color::SolidColor(SolidColor::new(1.0, 1.0, 1.0, 1.0));
                    self.debug_line.update();

                    renderer.draw(&mut self.debug_circle)?;
                    renderer.draw(&mut self.debug_line)?;
                }
            }
        }

        Ok(())
    }
}
