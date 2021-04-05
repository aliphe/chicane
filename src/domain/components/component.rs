#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ComponentType {
    Position,
    Orientation,
    Speed,
}

pub trait Component {
    fn get_identifier(&self) -> ComponentType;
}
