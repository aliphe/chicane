use crate::domain::components::movement::MovementComponent;

use super::system::System;

pub struct MovementSystem<MovementComponent> {
}

impl System<MovementComponent> for MovementSystem {
    fn tick(&self) -> Result<(), String> {
        return ();
    }
}
