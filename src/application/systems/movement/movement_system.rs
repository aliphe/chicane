use crate::{
    application::{
        entities_repository::entities_repository::EntitiesRepository, systems::system::System,
    },
    domain::components::{
        component::ComponentType, orientation::OrientationComponent, position::PositionComponent,
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
        let speed_component = *entities_repository
            .retrieve_entity_component(entity_id, &ComponentType::Speed)
            .and_then(|component| component.as_any().downcast_ref::<SpeedComponent>())
            .and_then(|speed_component| Some(speed_component))
            .expect("Unable to retrieve speed component");

        let orientation_component = *entities_repository
            .retrieve_entity_component(entity_id, &ComponentType::Orientation)
            .and_then(|component| component.as_any().downcast_ref::<OrientationComponent>())
            .and_then(|orientation_component| Some(orientation_component))
            .expect("Unable to retrieve orientation component");

        let position_component = entities_repository
            .retrieve_entity_component_mut(entity_id, &ComponentType::Position)
            .and_then(|component| component.as_any_mut().downcast_mut::<PositionComponent>())
            .and_then(|position_component| Some(position_component))
            .expect("Unable to retrieve position component");

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
