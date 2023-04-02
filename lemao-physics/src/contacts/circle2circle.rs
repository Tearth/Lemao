use crate::body::Body;
use crate::collisions::Collision;
use lemao_math::vec2::Vec2;

use super::Contact;

pub fn process(circle1: &Body, _circle2: &Body, collision: &Collision) -> Vec<Contact> {
    let position = circle1.position + collision.direction * circle1.size.x / 2.0;

    vec![Contact::new(position)]
}
