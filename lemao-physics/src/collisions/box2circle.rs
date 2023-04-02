use super::Collision;
use crate::body::Body;
use lemao_math::vec2::Vec2;

pub fn process(r#box: &Body, circle: &Body) -> Option<Collision> {
    let r#box_v1 = Vec2::new(r#box.size.x, r#box.size.y) / 2.0;
    let r#box_v2 = Vec2::new(-r#box.size.x, r#box.size.y) / 2.0;
    let r#box_v3 = Vec2::new(-r#box.size.x, -r#box.size.y) / 2.0;
    let r#box_v4 = Vec2::new(r#box.size.x, -r#box.size.y) / 2.0;

    let angle = Vec2::new(r#box.rotation.cos(), r#box.rotation.sin());
    let r#box_v1 = r#box.position + Vec2::new(r#box_v1.x * angle.x - r#box_v1.y * angle.y, r#box_v1.x * angle.y + r#box_v1.y * angle.x);
    let r#box_v2 = r#box.position + Vec2::new(r#box_v2.x * angle.x - r#box_v2.y * angle.y, r#box_v2.x * angle.y + r#box_v2.y * angle.x);
    let r#box_v3 = r#box.position + Vec2::new(r#box_v3.x * angle.x - r#box_v3.y * angle.y, r#box_v3.x * angle.y + r#box_v3.y * angle.x);
    let r#box_v4 = r#box.position + Vec2::new(r#box_v4.x * angle.x - r#box_v4.y * angle.y, r#box_v4.x * angle.y + r#box_v4.y * angle.x);
    let r#box_vertices = [r#box_v1, r#box_v2, r#box_v3, r#box_v4];

    let mut collision_detected = true;
    let mut collision_depth = f32::MAX;
    let mut collision_direction = Vec2::default();

    let mut axes = Vec::new();

    for [v1, v2] in [[r#box_v1, r#box_v2], [r#box_v2, r#box_v3]] {
        let edge = v2 - v1;
        let axis = Vec2::new(-edge.y, edge.x).normalized();

        axes.push(axis);
    }

    let mut min_dist = f32::MAX;
    let mut nearest_vertice = Vec2::default();

    for v in r#box_vertices {
        let dist = v.distance(circle.position);
        if dist < min_dist {
            min_dist = dist;
            nearest_vertice = v;
        }
    }

    axes.push((nearest_vertice - circle.position).normalized());

    let mut invert_depth = false;
    for axis in axes {
        let circle_min = (circle.position - axis * circle.size.x / 2.0).dot(axis);
        let circle_max = (circle.position + axis * circle.size.x / 2.0).dot(axis);

        let mut r#box_min = f32::MAX;
        let mut r#box_max = f32::MIN;

        for v in r#box_vertices {
            let projection = v.dot(axis);
            r#box_min = r#box_min.min(projection);
            r#box_max = r#box_max.max(projection);
        }

        if circle_min >= r#box_max || r#box_min >= circle_max {
            collision_detected = false;
            break;
        }

        let d1 = r#box_max - circle_min;
        let d2 = circle_max - r#box_min;
        let axis_depth = d1.min(d2);

        if axis_depth < collision_depth {
            collision_depth = axis_depth;
            collision_direction = axis;
            invert_depth = d1 > d2;
        }
    }

    if collision_detected {
        if invert_depth {
            collision_direction = -collision_direction;
        }

        Some(Collision::new(collision_depth, collision_direction))
    } else {
        None
    }
}
