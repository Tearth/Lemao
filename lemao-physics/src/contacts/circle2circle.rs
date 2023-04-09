use super::Contact;
use crate::body::Body;
use crate::collisions::Collision;

pub fn process(circle1: &Body, _circle2: &Body, collision: &Collision) -> Contact {
    Contact::new(circle1.position + collision.direction * circle1.size.x / 2.0)
}
