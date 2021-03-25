use application::entities_repository::{
    entities_repository::EntitiesRepository, memory_entities_repository::MemoryEntitiesRepository,
};
use domain::components::{
    component::Component, orientation::OrientationComponent, position::PositionComponent,
};

mod application;
mod core;
mod domain;

fn main() {
    let mut a = MemoryEntitiesRepository::new();

    let mut components: Vec<Box<dyn Component>> = Vec::new();

    components.push(Box::new(OrientationComponent { orientation: 12.0 }));
    components.push(Box::new(PositionComponent { x: 0, y: 0, z: 0 }));

    a.register_entity(String::from("car"), components);

    let stored_components = a.retrieve_entity_by_id(&String::from("car"));
    match stored_components {
        Some(comps) => {
            println!("{}", comps.len())
        }
        _ => (),
    }
}
