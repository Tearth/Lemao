use super::circle::Circle;
use super::disc::Disc;
use super::frame::Frame;
use super::line::Line;
use super::rectangle::Rectangle;
use super::text::Text;
use super::tilemap::Tilemap;
use super::*;

#[derive(Default)]
pub struct DrawableStorage {
    data: Vec<Option<Box<dyn Drawable>>>,
}

impl DrawableStorage {
    pub fn store_circle(&mut self, mut circle: Box<Circle>) -> usize {
        let id = self.get_free_component_id();
        circle.id = id;
        self.data[id] = Some(circle);

        id
    }

    pub fn store_disc(&mut self, mut disc: Box<Disc>) -> usize {
        let id = self.get_free_component_id();
        disc.id = id;
        self.data[id] = Some(disc);

        id
    }

    pub fn store_frame(&mut self, mut frame: Box<Frame>) -> usize {
        let id = self.get_free_component_id();
        frame.id = id;
        self.data[id] = Some(frame);

        id
    }

    pub fn store_line(&mut self, mut line: Box<Line>) -> usize {
        let id = self.get_free_component_id();
        line.id = id;
        self.data[id] = Some(line);

        id
    }

    pub fn store_rectangle(&mut self, mut rectangle: Box<Rectangle>) -> usize {
        let id = self.get_free_component_id();
        rectangle.id = id;
        self.data[id] = Some(rectangle);

        id
    }

    pub fn store_text(&mut self, mut text: Box<Text>) -> usize {
        let id = self.get_free_component_id();
        text.id = id;
        self.data[id] = Some(text);

        id
    }

    pub fn store_tilemap(&mut self, mut tilemap: Box<Tilemap>) -> usize {
        let id = self.get_free_component_id();
        tilemap.id = id;
        self.data[id] = Some(tilemap);

        id
    }

    pub fn get(&self, id: usize) -> Result<&dyn Drawable, String> {
        if id >= self.data.len() {
            return Err(format!("Drawable with id {} not found", id));
        }

        match &self.data[id] {
            Some(drawable) => Ok(drawable.as_ref()),
            None => Err(format!("Drawable with id {} not found", id)),
        }
    }

    pub fn get_mut(&mut self, id: usize) -> Result<&mut dyn Drawable, String> {
        if id >= self.data.len() {
            return Err(format!("Drawable with id {} not found", id));
        }

        match &mut self.data[id] {
            Some(drawable) => Ok(drawable.as_mut()),
            None => Err(format!("Drawable with id {} not found", id)),
        }
    }

    pub fn remove(&mut self, id: usize) -> Result<(), String> {
        if id >= self.data.len() {
            return Err(format!("Drawable with id {} not found", id));
        }
        self.data[id] = None;

        Ok(())
    }

    fn get_free_component_id(&mut self) -> usize {
        if let Some(id) = self.data.iter().position(|p| p.is_none()) {
            id
        } else {
            self.data.push(None);
            self.data.len() - 1
        }
    }
}
