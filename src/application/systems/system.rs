use crate::domain::components::component::Component;

pub trait System<Selector> where Selector: Component {
  fn tick(&self) -> Result<(), String>;

  fn registerComponents(&self) -> ();
}
