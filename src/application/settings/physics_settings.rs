pub struct PhysicsSettings {
    max_speed: f32,
}

impl PhysicsSettings {
    pub fn new() -> PhysicsSettings {
        PhysicsSettings { max_speed: 372.5 }
    }

    pub fn get_max_speed(&self) -> f32 {
        self.max_speed
    }
}
