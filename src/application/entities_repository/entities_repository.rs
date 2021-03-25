use crate::domain::components::component::Component;

pub trait EntitiesRepository {
    fn register_entity(&mut self, entity_id: String, components: Vec<Box<dyn Component>>);

    fn retrieve_entity_by_id(&self, entity_id: &String) -> Option<&Vec<Box<dyn Component>>>;
}
