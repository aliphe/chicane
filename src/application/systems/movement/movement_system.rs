use crate::{
    application::{
        entities_repository::entities_repository::EntitiesRepository, systems::system::System,
    },
    domain::components::{
        component::{ComponentType},
        position::PositionComponent,
        speed::SpeedComponent,
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
        let mut speed = 0.0;
        let found_speed =
            entities_repository.retrieve_entity_component(entity_id, &ComponentType::Speed);
        if let Some(val) = found_speed {
            let comp_speed = val.as_any().downcast_ref::<SpeedComponent>();
            if let Some(s) = comp_speed {
                speed = s.speed;
            }
        }

        let found_pos =
            entities_repository.retrieve_entity_component_mut(entity_id, &ComponentType::Position);

        if let Some(val) = found_pos {
            let pos = val.as_any_mut().downcast_mut::<PositionComponent>();
            if let Some(p) = pos {
                println!("Moving entity at {} at speed {}", p.x, speed);
                p.x += speed;
            }
        }
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
