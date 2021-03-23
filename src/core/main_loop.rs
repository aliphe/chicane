use crate::{application::input::systems::movement::InputMovementSystem, domain::components::movement::MovementComponent};

pub trait MainLoop {
  fn next(&mut self) -> bool;
}
