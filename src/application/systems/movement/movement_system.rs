use crate::{
    application::{
        entities_repository::entities_repository::EntitiesRepository, systems::system::System,
    },
    domain::components::{
        component::ComponentType, position::PositionComponent, speed::SpeedComponent,
    },
};

pub struct MovementSystem {
    entities_signature: Vec<ComponentType>,
}

impl MovementSystem {
    pub fn new() -> MovementSystem {
        MovementSystem {
            entities_signature: vec![
                ComponentType::Position,
                ComponentType::Speed,
                ComponentType::Orientation,
            ],
        }
    }

    fn move_entity(&self, entities_repository: &mut dyn EntitiesRepository, entity_id: &String) {
        let speed_component = *entities_repository
            .retrieve_entity_component(entity_id, &ComponentType::Speed)
            .and_then(|component| component.as_any().downcast_ref::<SpeedComponent>())
            .and_then(|speed_component| Some(speed_component))
            .expect("Unable to retrieve speed component");

        let found_pos = entities_repository
            .retrieve_entity_component_mut(entity_id, &ComponentType::Position)
            .and_then(|component| component.as_any_mut().downcast_mut::<PositionComponent>())
            .and_then(|position_component| Some(position_component))
            .expect("Unable to retrieve component");

        println!("Moving entity at {} at speed {}", found_pos.x, speed_component.speed);
        found_pos.x += speed_component.speed;
    }
}

impl System for MovementSystem {
    fn tick(&self, entities_repository: &mut dyn EntitiesRepository) -> Result<(), String> {
        let entity_ids =
            entities_repository.retrieve_entities_by_components(&self.entities_signature);

        entity_ids.iter().for_each(|id| {
            self.move_entity(entities_repository, id);
        });
        Result::Ok(())
    }
}
