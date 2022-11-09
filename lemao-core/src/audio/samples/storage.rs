use super::*;

#[derive(Default)]
pub struct SampleStorage {
    data: Vec<Option<Sample>>,
}

impl SampleStorage {
    pub fn store(&mut self, mut sample: Sample) -> Result<usize, String> {
        let id = self.data.len();
        sample.id = id;
        self.data.push(Some(sample));

        Ok(id)
    }

    pub fn get(&self, id: usize) -> Option<&Sample> {
        if id >= self.data.len() {
            return None;
        }

        self.data[id].as_ref()
    }

    pub fn remove(&mut self, id: usize) -> Result<(), String> {
        if id >= self.data.len() {
            return Err(format!("Sample with id {} not found", id));
        }
        self.data[id] = None;

        Ok(())
    }
}
