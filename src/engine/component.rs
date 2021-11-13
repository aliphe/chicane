use std::any::Any;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ComponentType {
    Position,
    Orientation,
    Speed,
    Throttle,
    Steering,
    Brake,
}

// impl ToRedisArg for ComponentType {}

#[typetag::serde(tag = "type")]
pub trait Component {
    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn get_identifier(&self) -> ComponentType;
}
