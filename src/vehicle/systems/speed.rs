use crate::{
    engine::entities_repository::entities_repository::{
        retrieve_entity_component, retrieve_entity_component_mut, EntitiesRepository,
    },
    vehicle::components::{
        brake::BrakeComponent, component::ComponentType, speed::SpeedComponent,
        throttle::ThrottleComponent,
    },
};

use super::system::System;

pub struct SpeedSystem {
    entities_signature: Vec<ComponentType>,
}

impl SpeedSystem {
    pub fn new() -> SpeedSystem {
        SpeedSystem {
            entities_signature: vec![
                ComponentType::Speed,
                ComponentType::Throttle,
                ComponentType::Brake,
            ],
        }
    }

    fn new_speed(&self, speed: f32, throttle: f32, brake: f32) -> f32 {
        // TODO use something like tis baby f(x) = 1-(1-200*log(x+1))
        let max_speed: f32 = 400.0;

        max_speed.min(speed + throttle * 10.0 - brake * 20.0)
    }

    fn change_entity_speed(
        &self,
        entities_repository: &mut dyn EntitiesRepository,
        entity_id: &String,
    ) {
        let throttle_component = retrieve_entity_component::<ThrottleComponent>(
            entities_repository,
            entity_id,
            ComponentType::Throttle,
        );
        let brake_component = retrieve_entity_component::<BrakeComponent>(
            entities_repository,
            entity_id,
            ComponentType::Brake,
        );

        let speed_component = retrieve_entity_component_mut::<SpeedComponent>(
            entities_repository,
            entity_id,
            ComponentType::Speed,
        );

        let new_speed = self.new_speed(
            speed_component.speed,
            throttle_component.throttle,
            brake_component.brake,
        );

        println!(
            "[SpeedSystem       ] Changing speed of entity from {:.2} to {:.2} given throttle {:.2} and brake {:.2}",
            speed_component.speed,
            new_speed,
            throttle_component.throttle,
            brake_component.brake,
        );
        speed_component.speed = new_speed;
    }
}

impl System for SpeedSystem {
    fn tick(&self, entities_repository: &mut dyn EntitiesRepository) -> Result<(), String> {
        let entity_ids =
            entities_repository.retrieve_entities_by_components(&self.entities_signature);

        entity_ids.iter().for_each(|id| {
            self.change_entity_speed(entities_repository, id);
        });

        Result::Ok(())
    }
}
