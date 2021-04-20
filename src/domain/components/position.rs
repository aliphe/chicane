use std::any::Any;

use super::component::{Component, ComponentType};

#[derive(Clone, Copy)]
pub struct PositionComponent {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Component for PositionComponent {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn get_identifier(&self) -> ComponentType {
        ComponentType::Position
    }

}
