use application::entities_repository::{
    entities_repository::EntitiesRepository, memory_entities_repository::MemoryEntitiesRepository,
};
use domain::components::{
    component::{Component, ComponentType},
    orientation::OrientationComponent,
    position::PositionComponent,
    speed::SpeedComponent,
};

mod application;
mod core;
mod domain;

fn main() {
    let mut repository = MemoryEntitiesRepository::new();

    let mut components: Vec<Box<dyn Component>> = Vec::new();

    components.push(Box::new(OrientationComponent { orientation: 12.0 }));
    components.push(Box::new(PositionComponent { x: 0, y: 0, z: 0 }));
    components.push(Box::new(SpeedComponent { speed: 387.32 }));

    repository.register_entity(String::from("car"), components);

    let stored_components = repository.retrieve_entity_by_id(&String::from("car"));
    match stored_components {
        Some(comps) => {
            let identifiers: Vec<ComponentType> = comps
                .into_iter()
                .map(|c| c.as_ref().get_identifier())
                .collect();

            println!(
                "This entity is defined by the following components {:#?}",
                identifiers
            )
        }
        _ => (),
    }

    let empty_search: Vec<ComponentType> = Vec::new();
    let empty = repository.retrieve_entities_by_components(empty_search);
    println!("Entities matching signature {:?}", empty);

    let mut speed_search: Vec<ComponentType> = Vec::new();
    speed_search.push(ComponentType::Speed);
    let with_speed = repository.retrieve_entities_by_components(speed_search);
    println!("Entities matching signature {:?}", with_speed);

    let mut all_search: Vec<ComponentType> = Vec::new();
    all_search.push(ComponentType::Speed);
    all_search.push(ComponentType::Position);
    all_search.push(ComponentType::Orientation);
    let with_all = repository.retrieve_entities_by_components(all_search);
    println!("Entities matching signature {:?}", with_all);
}
