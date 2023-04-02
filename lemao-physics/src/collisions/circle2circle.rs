use crate::body::Body;

use super::Collision;

pub fn process(circle1: &Body, circle2: &Body) -> Option<Collision> {
    let radius1 = circle1.size.x / 2.0;
    let radius2 = circle2.size.y / 2.0;
    let distance = circle1.position.distance(circle2.position);
    let collision_depth = distance - (radius1 + radius2);
    let collision_direction = (circle2.position - circle1.position).normalized();

    if collision_depth < 0.0 {
        Some(Collision::new(collision_depth.abs(), collision_direction))
    } else {
        None
    }
}
