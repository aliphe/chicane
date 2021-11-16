use std::any::Any;

use serde::{Deserialize, Serialize};

use crate::engine::component::{Component, ComponentType};

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct SpeedComponent {
    pub speed: f32,
}

#[typetag::serde]
impl Component for SpeedComponent {
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn get_identifier(&self) -> ComponentType {
        ComponentType::Speed
    }
}