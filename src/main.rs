use std::f32::consts::PI;

use application::{
    entities_repository::{
        entities_repository::EntitiesRepository,
        memory_entities_repository::MemoryEntitiesRepository,
    },
    systems::{movement::movement_system::MovementSystem, system::System},
};
use domain::components::{
    component::Component, orientation::OrientationComponent, position::PositionComponent,
    speed::SpeedComponent,
};

mod application;
mod core;
mod domain;

fn main() {
    let mut repository = MemoryEntitiesRepository::new();

    let mut components: Vec<Box<dyn Component>> = Vec::new();

    let movement_system = MovementSystem::new();

    components.push(Box::new(OrientationComponent {
        orientation: PI / 2.0,
    }));
    components.push(Box::new(PositionComponent {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    }));
    components.push(Box::new(SpeedComponent { speed: 387.32 }));

    repository.register_entity(String::from("car"), components);

    let mut i = 0;
    while i < 10 {
        let ticked = movement_system.tick(&mut repository);
        if let Err(err) = ticked {
            println!("ERROR {}", err);
        }
        i += 1;
    }
}
