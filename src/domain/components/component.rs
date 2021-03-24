pub enum ComponentType {
  Movement,
}

pub trait Component {
  fn getIdentifier(&self) -> ComponentType;
}
