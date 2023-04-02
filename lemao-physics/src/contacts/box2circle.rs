use crate::body::Body;
use crate::collisions::Collision;
use lemao_math::vec2::Vec2;

use super::Contact;

pub fn process(r#box: &Body, circle: &Body, collision: &Collision) -> Vec<Contact> {
    let position = circle.position - collision.direction * circle.size.x / 2.0;

    vec![Contact::new(position)]
}
