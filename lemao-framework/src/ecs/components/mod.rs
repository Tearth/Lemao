pub mod list;
pub mod manager;

pub trait Component {
    fn get_entity_id(&self) -> usize;
    fn set_entity_id(&mut self, entity_id: usize);
}
