use super::component::{Component, ComponentType};

pub struct OrientationComponent {
    pub orientation: f32, // 0-360
}

impl Component for OrientationComponent {
    fn get_identifier(&self) -> ComponentType {
        ComponentType::Orientation
    }
}
