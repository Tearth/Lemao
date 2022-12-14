use super::animation::Animation;
use super::circle::Circle;
use super::frame::Frame;
use super::line::Line;
use super::rectangle::Rectangle;
use super::sprite::Sprite;
use super::text::Text;
use super::*;

#[derive(Default)]
pub struct DrawableStorage {
    data: Vec<Option<Box<dyn Drawable>>>,
}

impl DrawableStorage {
    pub fn store_animation(&mut self, mut animation: Box<Animation>) -> usize {
        let id = self.data.len();
        animation.id = id;
        self.data.push(Some(animation));

        id
    }

    pub fn store_circle(&mut self, mut circle: Box<Circle>) -> usize {
        let id = self.data.len();
        circle.id = id;
        self.data.push(Some(circle));

        id
    }

    pub fn store_frame(&mut self, mut frame: Box<Frame>) -> usize {
        let id = self.data.len();
        frame.id = id;
        self.data.push(Some(frame));

        id
    }

    pub fn store_line(&mut self, mut line: Box<Line>) -> usize {
        let id = self.data.len();
        line.id = id;
        self.data.push(Some(line));

        id
    }

    pub fn store_rectangle(&mut self, mut rectangle: Box<Rectangle>) -> usize {
        let id = self.data.len();
        rectangle.id = id;
        self.data.push(Some(rectangle));

        id
    }

    pub fn store_sprite(&mut self, mut sprite: Box<Sprite>) -> usize {
        let id = self.data.len();
        sprite.id = id;
        self.data.push(Some(sprite));

        id
    }

    pub fn store_text(&mut self, mut text: Box<Text>) -> usize {
        let id = self.data.len();
        text.id = id;
        self.data.push(Some(text));

        id
    }

    pub fn get(&self, id: usize) -> Result<&dyn Drawable, String> {
        if id >= self.data.len() {
            return Err(format!("Drawable with id {} not found", id));
        }

        match &self.data[id] {
            Some(drawable) => Ok(drawable.as_ref()),
            None => return Err(format!("Drawable with id {} not found", id)),
        }
    }

    pub fn get_mut(&mut self, id: usize) -> Result<&mut dyn Drawable, String> {
        if id >= self.data.len() {
            return Err(format!("Drawable with id {} not found", id));
        }

        match &mut self.data[id] {
            Some(drawable) => Ok(drawable.as_mut()),
            None => return Err(format!("Drawable with id {} not found", id)),
        }
    }

    pub fn remove(&mut self, id: usize) -> Result<(), String> {
        if id >= self.data.len() {
            return Err(format!("Drawable with id {} not found", id));
        }
        self.data[id] = None;

        Ok(())
    }
}
