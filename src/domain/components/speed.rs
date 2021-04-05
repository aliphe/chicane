use super::component::{Component, ComponentType};

pub struct SpeedComponent {
    pub speed: f32,
}

impl Component for SpeedComponent {
    fn get_identifier(&self) -> ComponentType {
        ComponentType::Speed
    }
}
