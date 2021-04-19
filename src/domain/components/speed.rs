use std::any::Any;

use super::component::{Component, ComponentType};

pub struct SpeedComponent {
    pub speed: f32,
}

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
