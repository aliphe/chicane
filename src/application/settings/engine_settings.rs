pub struct EngineSettings {
    frames_per_second: i16,
}

impl EngineSettings {
    pub fn new() -> EngineSettings {
        EngineSettings {
            frames_per_second: 144,
        }
    }

    // pub fn get_frames_per_second(&self) -> i16 {
    //     self.frames_per_second
    // }

    pub fn get_frames_duration(&self) -> i16 {
        1 / self.frames_per_second
    }
}
