use super::*;

#[derive(Default)]
pub struct SampleStorage {
    data: Vec<Option<Sample>>,
}

impl SampleStorage {
    pub fn load(&mut self, path: &str) -> Result<usize, String> {
        let id = self.data.len();
        let mut sample = wav::load(path)?;

        sample.id = id;
        self.data.push(Some(sample));

        Ok(id)
    }

    pub fn get(&self, id: usize) -> Option<&Sample> {
        if id >= self.data.len() {
            return None;
        }

        match &self.data[id] {
            Some(sample) => Some(sample),
            None => None,
        }
    }
}
