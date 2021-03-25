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
}
