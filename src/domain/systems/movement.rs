pub trait MovementSystem<MovementComponent> {
  fn move_entity(component: MovementComponent) -> ();
}
