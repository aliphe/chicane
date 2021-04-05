use application::entities_repository::{
    entities_repository::EntitiesRepository, memory_entities_repository::MemoryEntitiesRepository,
};
use domain::components::{component::{Component, ComponentType}, orientation::OrientationComponent, position::PositionComponent, speed::{SpeedComponent}};

mod application;
mod core;
mod domain;

fn main() {
    let mut a = MemoryEntitiesRepository::new();

    let mut components: Vec<Box<dyn Component>> = Vec::new();

    components.push(Box::new(OrientationComponent { orientation: 12.0 }));
    components.push(Box::new(PositionComponent { x: 0, y: 0, z: 0 }));
    components.push(Box::new(SpeedComponent{ speed: 387.32}));

    a.register_entity(String::from("car"), components);

    let stored_components = a.retrieve_entity_by_id(&String::from("car"));
    match stored_components {
        Some(comps) => {
            let identifiers: Vec<ComponentType> = comps.into_iter().map(|c| {
                c.as_ref().get_identifier()
            }).collect();

            println!("This entity is defined by the following components {:#?}", identifiers)
        }
        _ => (),
    }
}
