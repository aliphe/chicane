#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ComponentType {
    Position,
    Orientation,
}

pub trait Component {
    fn get_identifier(&self) -> ComponentType;
}
