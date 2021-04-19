use std::any::Any;

use super::component::{Component, ComponentType};

pub struct OrientationComponent {
    pub orientation: f32, // 0-360
}

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
