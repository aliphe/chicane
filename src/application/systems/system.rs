pub trait System {
  fn tick(&self) -> Result<(), String>;
}
