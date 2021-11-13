use crate::{
    engine::{
        component::ComponentType,
        entities_repository::entities_repository::{
            retrieve_entity_component, retrieve_entity_component_mut, EntitiesRepository,
        },
        system::System,
    },
    vehicle::components::{
        orientation::OrientationComponent, position::PositionComponent, speed::SpeedComponent,
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
        let speed_component = retrieve_entity_component::<SpeedComponent>(
            entities_repository,
            entity_id,
            ComponentType::Speed,
        );

        let orientation_component = retrieve_entity_component::<OrientationComponent>(
            entities_repository,
            entity_id,
            ComponentType::Orientation,
        );

        let position_component = retrieve_entity_component_mut::<PositionComponent>(
            entities_repository,
            entity_id,
            ComponentType::Position,
        );

        println!(
          "[MovementComponent ] Moving entity at ({:.2},{:.2}) (orientation {:.2}) at speed {:.2}",
         position_component.x,
         position_component.y,
         orientation_component.orientation,
         speed_component.speed,
      );
        position_component.x += speed_component.speed * orientation_component.orientation.cos();
        position_component.y += speed_component.speed * orientation_component.orientation.sin();
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
