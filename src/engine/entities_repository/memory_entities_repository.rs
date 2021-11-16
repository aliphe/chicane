use std::collections::HashMap;

use crate::engine::component::{Component, ComponentType};

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

impl<'a> EntitiesRepository<'a> for MemoryEntitiesRepository {
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

    fn retrieve_entity_by_id(&mut self, entity_id: &String) -> Option<Vec<Box<dyn Component>>> {
        // self.entities_components.get(entity_id).as_deref()
        todo!()
    }

    fn retrieve_entity_by_id_mut(
        &mut self,
        entity_id: &String,
    ) -> Option<&mut Vec<Box<dyn Component>>> {
        self.entities_components.get_mut(entity_id)
    }

    fn retrieve_entities_by_components(&self, components: &Vec<ComponentType>) -> Vec<String> {
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

    fn retrieve_entity_component(
        &mut self,
        entity_id: &String,
        component_type: &ComponentType,
    ) -> Option<&dyn Component> {
        let entity_search = self.retrieve_entity_by_id(entity_id);
        match entity_search {
            Some(components) => {
                let found_comp = components
                    .iter()
                    .find(|c| &c.get_identifier() == component_type);
                if let Some(val) = found_comp {
                    return Some(val.as_ref());
                }
                None
            }
            None => None,
        }
    }

    fn retrieve_entity_component_mut(
        &mut self,
        entity_id: &String,
        component_type: &ComponentType,
    ) -> Option<&mut dyn Component> {
        let entity_search = self.retrieve_entity_by_id_mut(entity_id);
        match entity_search {
            Some(components) => {
                let found_comp = components
                    .iter_mut()
                    .find(|c| &c.get_identifier() == component_type);
                if let Some(val) = found_comp {
                    return Some(val.as_mut());
                }
                None
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::vehicle::components::{
        orientation::OrientationComponent, position::PositionComponent, speed::SpeedComponent,
    };

    use super::*;

    #[test]
    fn register_entity_basic_components_success() {
        let mut repository = MemoryEntitiesRepository::new();
        let mut components: Vec<Box<dyn Component>> = Vec::new();
        components.push(Box::new(OrientationComponent { orientation: 12.0 }));
        repository.register_entity(String::from("car"), components);
        let stored_components = repository.entities_components.get(&String::from("car")); //retrieve_entity_by_id(&String::from("car"));

        let mut cmp: Vec<Box<dyn Component>> = Vec::new();
        cmp.push(Box::new(OrientationComponent { orientation: 12.0 }));

        match stored_components {
            Some(comps) => assert!(
                comps
                    .iter()
                    .zip(cmp.iter())
                    .all(|(a, b)| a.get_identifier() == b.get_identifier()),
                "Arrays are not equal"
            ),
            None => panic!("Stored components for this entity should not be empty"),
        }
    }

    #[test]
    fn retrieve_entity_by_id_existing_entity_return_component_list() {
        let mut entities_components: HashMap<String, Vec<Box<dyn Component>>> = HashMap::new();
        entities_components.insert(
            String::from("Car"),
            vec![Box::new(PositionComponent {
                x: 0.0,
                y: 2.0,
                z: 2.0,
            })],
        );
        let mut repository = MemoryEntitiesRepository {
            entities_components,
            entities_per_components: HashMap::new(),
        };

        let components = repository.retrieve_entity_by_id_mut(&String::from("Car"));
        match components {
            Some(comps) => assert!(
                comps
                    .iter()
                    .zip(
                        vec![Box::new(PositionComponent {
                            x: 0.0,
                            y: 2.0,
                            z: 2.0
                        })]
                        .iter()
                    )
                    .all(|(a, b)| a.get_identifier() == b.get_identifier()),
                "Arrays are not equal"
            ),
            None => panic!("Stored components for this entity should not be empty"),
        }
    }

    #[test]
    fn retrieve_entities_by_components_two_components_find_entity() {
        let mut entities_components: HashMap<String, Vec<Box<dyn Component>>> = HashMap::new();
        entities_components.insert(
            String::from("Car"),
            vec![
                Box::new(PositionComponent {
                    x: 0.0,
                    y: 2.0,
                    z: 2.0,
                }),
                Box::new(OrientationComponent { orientation: 90.0 }),
                Box::new(SpeedComponent { speed: 123.0 }),
            ],
        );

        let mut entities_per_components: HashMap<ComponentType, Vec<String>> = HashMap::new();
        entities_per_components.insert(ComponentType::Position, vec![String::from("Car")]);
        entities_per_components.insert(ComponentType::Orientation, vec![String::from("Car")]);
        entities_per_components.insert(ComponentType::Speed, vec![String::from("Car")]);

        let repository = MemoryEntitiesRepository {
            entities_components,
            entities_per_components,
        };

        let retrieved = repository.retrieve_entities_by_components(&vec![
            ComponentType::Position,
            ComponentType::Orientation,
            ComponentType::Speed,
        ]);

        assert_eq!(retrieved[0], String::from("Car"))
    }
}
