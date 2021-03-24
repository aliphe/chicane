use super::component::{Component, ComponentType};

/**
 * This components stands for the first version, it is needed in order
 * to have a working project as quick as possible.
 */

pub struct MovementComponent {
    speed: i32,
    rotation: i32, // clockwise rotation from the north, in degrees
}

impl Component for MovementComponent {
    fn getIdentifier(&self) -> ComponentType {
        return ComponentType::Movement;
    }
}
