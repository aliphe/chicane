use std::any::Any;

use super::component::{Component, ComponentType};
#[derive(Clone, Copy)]
pub struct OrientationComponent {
    pub orientation: f32, // Radiant (0-2Pi)
}

impl OrientationComponent {}

impl Component for OrientationComponent {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn get_identifier(&self) -> ComponentType {
        ComponentType::Orientation
    }
}
