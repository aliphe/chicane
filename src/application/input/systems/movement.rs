use crate::domain::systems::movement::MovementSystem;

pub struct InputMovementSystem<MovementComponent> {
}

impl MovementSystem for InputMovementSystem<MovementComponent> {
  fn move_entity(component: MovementComponent) -> ();
}
