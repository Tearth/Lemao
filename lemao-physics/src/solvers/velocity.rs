use crate::body::Body;
use crate::collisions::Collision;
use crate::contacts::Contact;
use lemao_math::vec2::Vec2;

pub fn solve(body1: &mut Body, body2: &mut Body, collision: &Collision, contact: &Contact) -> f32 {
    let body1_velocity_linear = body1.velocity_linear;
    let body2_velocity_linear = body2.velocity_linear;
    let body1_velocity_angular = body1.velocity_angular;
    let body2_velocity_angular = body2.velocity_angular;

    let r1 = contact.position - body1.position;
    let r2 = contact.position - body2.position;
    let r1_perp = Vec2::new(-r1.y, r1.x).normalized();
    let r2_perp = Vec2::new(-r2.y, r2.x).normalized();
    let v1 = body1_velocity_linear + body1_velocity_angular * r1_perp;
    let v2 = body2_velocity_linear + body2_velocity_angular * r2_perp;
    let bounciness = body1.bounciness.min(body2.bounciness);
    let relative_velocity = v2 - v1;

    if relative_velocity.dot(collision.direction) <= 0.0 {
        let total_mass = (1.0 / body1.mass + 1.0 / body2.mass);
        let total_intertia = (r1_perp.dot(collision.direction).powi(2) / body1.inertia) + (r2_perp.dot(collision.direction).powi(2) / body2.inertia);
        let j = -(1.0 + bounciness) * relative_velocity.dot(collision.direction) / (total_mass + total_intertia);

        if body1.dynamic {
            body1.velocity_linear -= j * collision.direction / body1.mass;
            body1.velocity_angular -= r1_perp.dot(j * collision.direction) / body1.inertia;
        }

        if body2.dynamic {
            body2.velocity_linear += j * collision.direction / body2.mass;
            body2.velocity_angular += r2_perp.dot(j * collision.direction) / body2.inertia;
        }

        j
    } else {
        0.0
    }
}
