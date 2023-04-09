use super::Contact;
use crate::body::Body;
use crate::collisions::Collision;

pub fn process(r#box: &Body, circle: &Body, collision: &Collision) -> Contact {
    Contact::new(circle.position - collision.direction * circle.size.x / 2.0)
}
