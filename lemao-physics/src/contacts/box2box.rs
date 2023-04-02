use crate::body::Body;
use crate::collisions::Collision;
use lemao_math::vec2::Vec2;

use super::Contact;

pub fn process(box1: &Body, box2: &Body, collision: &Collision) -> Vec<Contact> {
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
    let body1_edges = [[body1_v1, body1_v2], [body1_v2, body1_v3], [body1_v3, body1_v4], [body1_v4, body1_v1]];

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
    let body2_edges = [[body2_v1, body2_v2], [body2_v2, body2_v3], [body2_v3, body2_v4], [body2_v4, body2_v1]];

    let mut contacts = Vec::new();

    for edge in body1_edges {
        let edge_direction = (edge[1] - edge[0]).normalized();
        let dot = edge_direction.dot(collision.direction).abs();

        if dot.abs() < 0.001 {
            for v in body2_vertices {
                if distance_to_edge(edge[0], edge[1], v).abs() < 0.001 {
                    contacts.push(Contact::new(v));
                }
            }
        }
    }

    for edge in body2_edges {
        let edge_direction = (edge[1] - edge[0]).normalized();
        let dot = edge_direction.dot(collision.direction).abs();

        if dot.abs() < 0.001 {
            for v in body1_vertices {
                if distance_to_edge(edge[0], edge[1], v).abs() < 0.001 {
                    contacts.push(Contact::new(v));
                }
            }
        }
    }

    if contacts.len() == 1 {
        return contacts;
    }

    let mut dots = Vec::new();
    let collision_normal = Vec2::new(-collision.direction.y, collision.direction.x);

    for contact in contacts {
        dots.push((contact, contact.position.dot(collision_normal)));
    }

    dots.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    vec![dots[1].0, dots[2].0]
}

fn distance_to_edge(start: Vec2, end: Vec2, point: Vec2) -> f32 {
    let a = (((start.x - end.x) * (end.y - point.y)) - ((end.x - point.x) * (start.y - end.y))).abs();
    let b = ((start.x - end.x).powi(2) + (start.y - end.y).powi(2)).sqrt();

    a / b
}
