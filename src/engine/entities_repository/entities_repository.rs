use crate::engine::component::{Component, ComponentType};

pub trait EntitiesRepository<'a> {
    fn register_entity(&mut self, entity_id: String, components: Vec<Box<dyn Component>>);

    fn retrieve_entity_by_id(&mut self, entity_id: &String) -> Option<Vec<Box<dyn Component>>>;
    fn retrieve_entity_by_id_mut(
        &mut self,
        entity_id: &String,
    ) -> Option<&mut Vec<Box<dyn Component>>>;

    fn retrieve_entities_by_components(&self, components: &Vec<ComponentType>) -> Vec<String>;

    fn retrieve_entity_component(
        &mut self,
        entity_id: &String,
        component_type: &ComponentType,
    ) -> Option<&dyn Component>;
    fn retrieve_entity_component_mut(
        &mut self,
        entity_id: &String,
        component_type: &ComponentType,
    ) -> Option<&mut dyn Component>;
}

pub fn retrieve_entity_component<'a, C: Component + Copy + 'static>(
    entities_repository: &'a dyn EntitiesRepository,
    entity_id: &String,
    component_type: ComponentType,
) -> C {
    *entities_repository
        .retrieve_entity_component(entity_id, &component_type)
        .and_then(|component| component.as_any().downcast_ref::<C>())
        .and_then(|throttle_component| Some(throttle_component))
        .expect("Unable to retrieve component")
}

pub fn retrieve_entity_component_mut<'a, C: Component + 'static>(
    entities_repository: &'a mut dyn EntitiesRepository,
    entity_id: &String,
    component_type: ComponentType,
) -> &'a mut C {
    entities_repository
        .retrieve_entity_component_mut(entity_id, &component_type)
        .and_then(|component| component.as_any_mut().downcast_mut::<C>())
        .and_then(|throttle_component| Some(throttle_component))
        .expect("Unable to retrieve component")
}
