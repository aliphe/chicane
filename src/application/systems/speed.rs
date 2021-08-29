use crate::{
    application::{
        entities_repository::entities_repository::EntitiesRepository,
        settings::{engine_settings::EngineSettings, physics_settings::PhysicsSettings},
        systems::system::System,
    },
    domain::components::{
        component::ComponentType, speed::SpeedComponent, throttle::ThrottleComponent,
    },
};

pub struct SpeedSystem<'a> {
    engine_settings: &'a EngineSettings,
    physics_settings: &'a PhysicsSettings,
    entities_signature: Vec<ComponentType>,
}

impl<'a> SpeedSystem<'a> {
    pub fn new(
        engine_settings: &'a EngineSettings,
        physics_settings: &'a PhysicsSettings,
    ) -> SpeedSystem<'a> {
        SpeedSystem {
            engine_settings,
            physics_settings,
            entities_signature: vec![ComponentType::Speed, ComponentType::Throttle],
        }
    }

    fn new_speed(&self, speed: f32, throttle: f32) -> f32 {
        // TODO use something like this baby f(x) = 1-(1-200*log(x+1))
        let max_speed = self.physics_settings.get_max_speed();

        max_speed.min(speed + throttle)
    }

    fn change_entity_speed(
        &self,
        entities_repository: &mut dyn EntitiesRepository,
        entity_id: &String,
    ) {
        let throttle_component = *entities_repository
            .retrieve_entity_component(entity_id, &ComponentType::Throttle)
            .and_then(|component| component.as_any().downcast_ref::<ThrottleComponent>())
            .and_then(|throttle_component| Some(throttle_component))
            .expect("Unable to retrieve throttle component");

        let speed_component = entities_repository
            .retrieve_entity_component_mut(entity_id, &ComponentType::Speed)
            .and_then(|component| component.as_any_mut().downcast_mut::<SpeedComponent>())
            .and_then(|speed_component| Some(speed_component))
            .expect("Unable to retrieve speed component");

        let new_speed = self.new_speed(speed_component.speed, throttle_component.throttle);

        println!(
            "[SpeedSystem       ] Changing speed of entity from {:.2} to {:.2} given throttle {:.2}",
            speed_component.speed,
            new_speed,
            throttle_component.throttle,
        );
        speed_component.speed = new_speed;
    }
}

impl<'a> System for SpeedSystem<'a> {
    fn tick(&self, entities_repository: &mut dyn EntitiesRepository) -> Result<(), String> {
        let entity_ids =
            entities_repository.retrieve_entities_by_components(&self.entities_signature);

        entity_ids.iter().for_each(|id| {
            self.change_entity_speed(entities_repository, id);
        });

        Result::Ok(())
    }
}
