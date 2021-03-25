use super::component::{Component, ComponentType};

pub struct PositionComponent {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Component for PositionComponent {
    fn get_identifier(&self) -> ComponentType {
        ComponentType::Position
    }
}
