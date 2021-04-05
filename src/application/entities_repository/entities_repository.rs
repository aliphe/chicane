use crate::domain::components::component::{Component, ComponentType};

pub trait EntitiesRepository {
    fn register_entity(&mut self, entity_id: String, components: Vec<Box<dyn Component>>);

    fn retrieve_entity_by_id(&self, entity_id: &String) -> Option<&Vec<Box<dyn Component>>>;

    fn retrieve_entities_by_components(&self, components: Vec<ComponentType>) -> Vec<String>;
}
