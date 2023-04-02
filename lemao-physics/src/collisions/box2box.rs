use super::Collision;
use crate::body::Body;
use lemao_math::vec2::Vec2;

pub fn process(box1: &Body, box2: &Body) -> Option<Collision> {
    let body1_v1 = Vec2::new(box1.size.x, box1.size.y) / 2.0;
    let body1_v2 = Vec2::new(-box1.size.x, box1.size.y) / 2.0;
    let body1_v3 = Vec2::new(-box1.size.x, -box1.size.y) / 2.0;
    let body1_v4 = Vec2::new(box1.size.x, -box1.size.y) / 2.0;

    let angle = Vec2::new(box1.rotation.cos(), box1.rotation.sin());
    let body1_v1 = box1.position + Vec2::new(body1_v1.x * angle.x - body1_v1.y * angle.y, body1_v1.x * angle.y + body1_v1.y * angle.x);
    let body1_v2 = box1.position + Vec2::new(body1_v2.x * angle.x - body1_v2.y * angle.y, body1_v2.x * angle.y + body1_v2.y * angle.x);
    let body1_v3 = box1.position + Vec2::new(body1_v3.x * angle.x - body1_v3.y * angle.y, body1_v3.x * angle.y + body1_v3.y * angle.x);
    let body1_v4 = box1.position + Vec2::new(body1_v4.x * angle.x - body1_v4.y * angle.y, body1_v4.x * angle.y + body1_v4.y * angle.x);
    let body1_vertices = [body1_v1, body1_v2, body1_v3, body1_v4];

    let body2_v1 = Vec2::new(box2.size.x, box2.size.y) / 2.0;
    let body2_v2 = Vec2::new(-box2.size.x, box2.size.y) / 2.0;
    let body2_v3 = Vec2::new(-box2.size.x, -box2.size.y) / 2.0;
    let body2_v4 = Vec2::new(box2.size.x, -box2.size.y) / 2.0;

    let angle = Vec2::new(box2.rotation.cos(), box2.rotation.sin());
    let body2_v1 = box2.position + Vec2::new(body2_v1.x * angle.x - body2_v1.y * angle.y, body2_v1.x * angle.y + body2_v1.y * angle.x);
    let body2_v2 = box2.position + Vec2::new(body2_v2.x * angle.x - body2_v2.y * angle.y, body2_v2.x * angle.y + body2_v2.y * angle.x);
    let body2_v3 = box2.position + Vec2::new(body2_v3.x * angle.x - body2_v3.y * angle.y, body2_v3.x * angle.y + body2_v3.y * angle.x);
    let body2_v4 = box2.position + Vec2::new(body2_v4.x * angle.x - body2_v4.y * angle.y, body2_v4.x * angle.y + body2_v4.y * angle.x);
    let body2_vertices = [body2_v1, body2_v2, body2_v3, body2_v4];

    let mut collision_detected = true;
    let mut collision_depth = f32::MAX;
    let mut collision_normal = Vec2::default();

    let mut invert_depth = false;
    for [v1, v2] in [[body1_v1, body1_v2], [body1_v2, body1_v3], [body2_v1, body2_v2], [body2_v2, body2_v3]] {
        let edge = v2 - v1;
        let axis = Vec2::new(-edge.y, edge.x).normalized();

        let mut body1_min = f32::MAX;
        let mut body1_max = f32::MIN;

        for v in body1_vertices {
            let projection = v.dot(axis);
            body1_min = body1_min.min(projection);
            body1_max = body1_max.max(projection);
        }

        let mut body2_min = f32::MAX;
        let mut body2_max = f32::MIN;

        for v in body2_vertices {
            let projection = v.dot(axis);
            body2_min = body2_min.min(projection);
            body2_max = body2_max.max(projection);
        }

        if body1_min >= body2_max || body2_min >= body1_max {
            collision_detected = false;
            break;
        }

        let d1 = body1_max - body2_min;
        let d2 = body2_max - body1_min;
        let axis_depth = d1.min(d2);

        if axis_depth < collision_depth {
            collision_depth = axis_depth;
            collision_normal = axis;
            invert_depth = d1 > d2;
        }
    }

    if collision_detected {
        if invert_depth {
            collision_normal = -collision_normal;
        }

        Some(Collision::new(collision_depth, collision_normal))
    } else {
        None
    }
}
