use std::collections::HashMap;

use crate::domain::components::component::{Component, ComponentType};

use super::entities_repository::EntitiesRepository;

pub struct MemoryEntitiesRepository {
    entities_components: HashMap<String, Vec<Box<dyn Component>>>,

    entities_per_components: HashMap<ComponentType, Vec<String>>,
}

impl MemoryEntitiesRepository {
    pub fn new() -> MemoryEntitiesRepository {
        MemoryEntitiesRepository {
            entities_components: HashMap::new(),
            entities_per_components: HashMap::new(),
        }
    }
}

impl EntitiesRepository for MemoryEntitiesRepository {
    fn register_entity(&mut self, entity_id: String, components: Vec<Box<dyn Component>>) {
        self.entities_components
            .insert(entity_id.clone(), components);

        let comps = self.entities_components.get(&entity_id);
        match comps {
            Some(comps) => {
                for c in comps {
                    self.entities_per_components
                        .insert(c.get_identifier(), Vec::new());
                }
            }
            _ => (),
        };
    }

    fn retrieve_entity_by_id(&self, entity_id: &String) -> Option<&Vec<Box<dyn Component>>> {
        self.entities_components.get(entity_id).clone()
    }

    fn retrieve_entities_by_components(&self, components: Vec<ComponentType>) -> Vec<String> {
        self.entities_components
            .iter()
            .filter_map(|(key, val)| {
                let mut components_matching = 0;
                for component in &components[..] {
                    match val.iter().find(|c| component == &c.get_identifier()) {
                        Some(_) => components_matching += 1,
                        None => (),
                    }
                }
                if components_matching == components.len() {
                    return Some(key.clone());
                }
                None
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::components::{orientation::OrientationComponent, position::PositionComponent};

    use super::*;

    #[test]
    fn register_entity_basic_components_success() {
        let mut repository = MemoryEntitiesRepository::new();
        let mut components: Vec<Box<dyn Component>> = Vec::new();
        components.push(Box::new(OrientationComponent { orientation: 12.0 }));
        repository.register_entity(String::from("car"), components);
        let stored_components = repository.entities_components.get(String::from("car")); //retrieve_entity_by_id(&String::from("car"));

        let mut cmp: Vec<Box<dyn Component>> = Vec::new();
        cmp.push(Box::new(OrientationComponent { orientation: 12.0 }));

        match stored_components {
            Some(comps) => assert!(
                comps.iter().zip(cmp.iter()).all(|(a, b)| a.get_identifier() == b.get_identifier()),
                "Arrays are not equal"
            ),
            None => panic!("Stored components for this entity should not be empty"),
        }
    }

    #[test]
    fn retrieve_entity_by_id_existing_entity_return_component_list() {
        let mut repository = MemoryEntitiesRepository::new();
        let mut components: Vec<Box<dyn Component>> = Vec::new();
        components.push(Box::new(OrientationComponent { orientation: 12.0 }));
        components.push(Box::new(PositionComponent { x: 0, y: 0, z: 0 }));
        repository.register_entity(String::from("car"), components); 
    }
}
