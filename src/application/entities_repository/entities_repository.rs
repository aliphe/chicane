use crate::domain::components::component::{Component, ComponentType};

pub trait EntitiesRepository {
    fn register_entity(&mut self, entity_id: String, components: Vec<Box<dyn Component>>);

    fn retrieve_entity_by_id(&self, entity_id: &String) -> Option<&Vec<Box<dyn Component>>>;
    fn retrieve_entity_by_id_mut(&mut self, entity_id: &String) -> Option<&mut Vec<Box<dyn Component>>>;

    fn retrieve_entities_by_components(&self, components: &Vec<ComponentType>) -> Vec<String>;

    fn retrieve_entity_component(
        &self, 
        entity_id: &String, 
        component_type: &ComponentType,
    ) -> Option<&dyn Component>;
    fn retrieve_entity_component_mut(
        &mut self, 
        entity_id: &String, 
        component_type: &ComponentType,
    ) -> Option<&mut dyn Component>;
}
