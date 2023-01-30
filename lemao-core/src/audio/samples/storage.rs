use super::*;

#[derive(Default)]
pub struct SampleStorage {
    data: Vec<Option<Sample>>,
}

impl SampleStorage {
    pub fn store(&mut self, mut sample: Sample) -> usize {
        let id = self.get_free_component_id();
        sample.id = id;
        self.data[id] = Some(sample);

        id
    }

    pub fn get(&self, id: usize) -> Result<&Sample, String> {
        if id >= self.data.len() {
            return Err(format!("Sample with id {} not found", id));
        }

        self.data[id].as_ref().ok_or(format!("Sample with id {} not found", id))
    }

    pub fn remove(&mut self, id: usize) -> Result<(), String> {
        if id >= self.data.len() {
            return Err(format!("Sample with id {} not found", id));
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
