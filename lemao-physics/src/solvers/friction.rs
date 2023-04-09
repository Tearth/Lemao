use crate::body::Body;
use crate::collisions::Collision;
use crate::contacts::Contact;
use lemao_math::vec2::Vec2;

pub fn solve(body1: &mut Body, body2: &mut Body, collision: &Collision, contact: &Contact, pressure: f32) {
    if pressure == 0.0 {
        return;
    }

    let r1 = contact.position - body1.position;
    let r2 = contact.position - body2.position;
    let r1_perp = Vec2::new(-r1.y, r1.x).normalized();
    let r2_perp = Vec2::new(-r2.y, r2.x).normalized();
    let v1 = body1.velocity_linear + body1.velocity_angular * r1_perp;
    let v2 = body2.velocity_linear + body2.velocity_angular * r2_perp;
    let friction_static = (body1.friction_static + body2.friction_static) / 2.0;
    let friction_dynamic = (body1.friction_dynamic + body2.friction_dynamic) / 2.0;
    let relative_velocity = v2 - v1;
    let direction = -(relative_velocity - relative_velocity.dot(collision.direction) * collision.direction).normalized();

    let total_mass = 1.0 / body1.mass + 1.0 / body2.mass;
    let total_intertia = (r1_perp.dot(direction).powi(2) / body1.inertia) + (r2_perp.dot(direction).powi(2) / body2.inertia);
    let mut j = relative_velocity.dot(direction) / (total_mass + total_intertia);

    // Check if dynamic friction should be applied
    if j.abs() > (pressure * friction_static) {
        j *= friction_dynamic;
    }

    if body1.dynamic {
        body1.velocity_angular += r1_perp.dot(j * direction) / body1.inertia;
        body1.velocity_linear += j * direction / body1.mass;
    }

    if body2.dynamic {
        body2.velocity_angular -= r2_perp.dot(j * direction) / body2.inertia;
        body2.velocity_linear -= j * direction / body2.mass;
    }
}
