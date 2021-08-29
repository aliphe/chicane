use std::any::Any;

use super::component::{Component, ComponentType};

#[derive(Clone, Copy)]
pub struct BrakeComponent {
    pub brake: f32,
}

impl Component for BrakeComponent {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn get_identifier(&self) -> ComponentType {
        ComponentType::Brake
    }
}
