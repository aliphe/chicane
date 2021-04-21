use crate::{application::{entities_repository::entities_repository::EntitiesRepository, systems::system::System}, domain::components::{component::ComponentType, speed::SpeedComponent, throttle::ThrottleComponent}};

pub struct SpeedSystem {
    entities_signature: Vec<ComponentType>,
}

impl SpeedSystem {
    pub fn new() -> SpeedSystem {
        SpeedSystem {
            entities_signature: vec![
                ComponentType::Speed,
                ComponentType::Throttle,
            ],
        }
    }

    fn change_entity_speed(&self, entities_repository: &mut dyn EntitiesRepository, entity_id: &String) {
        let speed_component = *entities_repository
            .retrieve_entity_component(entity_id, &ComponentType::Speed)
            .and_then(|component| component.as_any().downcast_ref::<SpeedComponent>())
            .and_then(|speed_component| Some(speed_component))
            .expect("Unable to retrieve speed component");
            
        let throttle_component = *entities_repository
            .retrieve_entity_component(entity_id, &ComponentType::Throttle)
            .and_then(|component| component.as_any().downcast_ref::<ThrottleComponent>())
            .and_then(|throttle_component| Some(throttle_component))
            .expect("Unable to retrieve throttle component");
        
        
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
