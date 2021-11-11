extern crate redis;

use redis::streams::StreamMaxlen;
use crate::domain::components::brake::BrakeComponent;
use std::f32::consts::PI;

use application::{
    entities_repository::{
        entities_repository::EntitiesRepository,
        memory_entities_repository::MemoryEntitiesRepository,
    },
    settings::{physics_settings::PhysicsSettings},
    systems::{
        movement::MovementSystem, rotation::RotationSystem,
        speed::SpeedSystem, system::System,
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
    let client = redis::Client::open("redis://127.0.0.1/").expect("Unable to open redis client");
    let mut con = client.get_connection().expect("Unable to connect to redis database");

    let maxlen = StreamMaxlen::Approx(1000);

    let _ = redis::cmd("ADD").arg("stream").arg("*").arg(1).query::<String>(&mut con).expect("");
    redis::cmd("ADD").arg("stream").arg("*").arg(1).query::<String>(&mut con).expect("");
    redis::cmd("ADD").arg("stream").arg("*").arg(1).query::<String>(&mut con).expect("");
    redis::cmd("ADD").arg("stream").arg("*").arg(1).query::<String>(&mut con).expect("");
    redis::cmd("ADD").arg("stream").arg("*").arg(1).query::<String>(&mut con).expect("");
    redis::cmd("ADD").arg("stream").arg("*").arg(1).query::<String>(&mut con).expect("");
    let a = redis::cmd("LEN").arg("stream").query::<i32>(&mut con);
    println!("{:?}", a);

    // Application-specific
    let physics_settings = PhysicsSettings::new();
    let mut repository = MemoryEntitiesRepository::new();

    // Systems
    let movement_system = MovementSystem::new();
    let rotation_system = RotationSystem::new();
    let speed_system = SpeedSystem::new(&physics_settings);

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
