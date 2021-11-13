use crate::{
    engine::{
        component::ComponentType, entities_repository::entities_repository::EntitiesRepository,
        system::System,
    },
    vehicle::components::{orientation::OrientationComponent, steering::SteeringComponent},
};

pub struct RotationSystem {
    entities_signature: Vec<ComponentType>,
}

impl RotationSystem {
    pub fn new() -> RotationSystem {
        RotationSystem {
            entities_signature: vec![ComponentType::Orientation, ComponentType::Steering],
        }
    }

    fn change_entity_rotation(
        &self,
        entities_repository: &mut dyn EntitiesRepository,
        entity_id: &String,
    ) {
        let steering_component = *entities_repository
            .retrieve_entity_component(entity_id, &ComponentType::Steering)
            .and_then(|component| component.as_any().downcast_ref::<SteeringComponent>())
            .and_then(|steering_component| Some(steering_component))
            .expect("Unable to retrieve steering component");

        let orientation_component = entities_repository
            .retrieve_entity_component_mut(entity_id, &ComponentType::Orientation)
            .and_then(|component| {
                component
                    .as_any_mut()
                    .downcast_mut::<OrientationComponent>()
            })
            .and_then(|orientation_component| Some(orientation_component))
            .expect("Unable to retrieve orientation component");

        let new_orientation =
            orientation_component.orientation + steering_component.steering / 10.0;

        println!(
            "[RotationSystem    ] Changing orientation of entity from {:.2} to {:.2} given steering {:.2}",
            orientation_component.orientation,
            new_orientation,
            steering_component.steering,
        );
        orientation_component.orientation = new_orientation;
    }
}

impl System for RotationSystem {
    fn tick(&self, entities_repository: &mut dyn EntitiesRepository) -> Result<(), String> {
        let entity_ids =
            entities_repository.retrieve_entities_by_components(&self.entities_signature);

        entity_ids.iter().for_each(|id| {
            self.change_entity_rotation(entities_repository, id);
        });

        Result::Ok(())
    }
}
