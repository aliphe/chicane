use std::any::Any;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ComponentType {
    Position,
    Orientation,
    Speed,
    Throttle,
    Steering,
}

pub trait Component {
    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn get_identifier(&self) -> ComponentType;
}
