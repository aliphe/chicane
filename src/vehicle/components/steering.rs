use std::any::Any;

use serde::{Deserialize, Serialize};

use crate::engine::component::{Component, ComponentType};

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct SteeringComponent {
    pub steering: f32, // (-1; 1)
}

#[typetag::serde]
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
