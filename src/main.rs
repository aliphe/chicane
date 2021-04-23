use std::f32::consts::PI;

use application::{
    entities_repository::{
        entities_repository::EntitiesRepository,
        memory_entities_repository::MemoryEntitiesRepository,
    },
    settings::{engine_settings::EngineSettings, physics_settings::PhysicsSettings},
    systems::{
        movement::movement_system::MovementSystem, rotation::rotation_system::RotationSystem,
        speed::speed_system::SpeedSystem, system::System,
    },
};
use domain::components::{
    component::Component, orientation::OrientationComponent, position::PositionComponent,
    speed::SpeedComponent, steering::SteeringComponent, throttle::ThrottleComponent,
};

mod application;
mod core;
mod domain;

fn main() {
    // Application-specific
    let engine_settings = EngineSettings::new();
    let physics_settings = PhysicsSettings::new();
    let mut repository = MemoryEntitiesRepository::new();

    // Systems
    let movement_system = MovementSystem::new();
    let rotation_system = RotationSystem::new();
    let speed_system = SpeedSystem::new(&engine_settings, &physics_settings);

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
