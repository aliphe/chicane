extern crate redis;

use std::f32::consts::PI;

use crate::{
    engine::entities_repository::{
        entities_repository::EntitiesRepository,
        memory_entities_repository::MemoryEntitiesRepository,
    },
    vehicle::{
        components::{
            brake::BrakeComponent, component::Component, orientation::OrientationComponent,
            position::PositionComponent, speed::SpeedComponent, steering::SteeringComponent,
            throttle::ThrottleComponent,
        },
        systems::{
            movement::MovementSystem, rotation::RotationSystem, speed::SpeedSystem, system::System,
        },
    },
};

mod engine;
mod vehicle;

fn main() {
    // Application-specific
    let mut repository = MemoryEntitiesRepository::new();

    // Systems
    let movement_system = MovementSystem::new();
    let rotation_system = RotationSystem::new();
    let speed_system = SpeedSystem::new();

    let mut components: Vec<Box<dyn Component>> = Vec::new();
    components.push(Box::new(OrientationComponent {
        orientation: PI / 2.0,
    }));
    components.push(Box::new(PositionComponent {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    }));
    components.push(Box::new(SpeedComponent { speed: 0.0 }));
    components.push(Box::new(ThrottleComponent { throttle: 1.0 }));
    components.push(Box::new(BrakeComponent { brake: 0.1 }));
    components.push(Box::new(SteeringComponent { steering: 0.25 }));

    repository.register_entity(String::from("car"), components);

    let mut i = 0;
    while i < 10 {
        let tick = speed_system
            .tick(&mut repository)
            .and_then(|_| rotation_system.tick(&mut repository))
            .and_then(|_| movement_system.tick(&mut repository));

        if let Err(err) = tick {
            println!("ERROR {}", err);
        }
        i += 1;
    }
}
