use std::any::Any;

use super::component::{Component, ComponentType};
#[derive(Clone, Copy)]
pub struct SteeringComponent {
    pub steering: f32, // (-1; 1)
}

impl Component for SteeringComponent {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn get_identifier(&self) -> ComponentType {
        ComponentType::Steering
    }
}
