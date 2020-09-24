pub struct DeltaTime {
    raw_time: u32,
    delta: f32,
}

impl DeltaTime {
    pub fn new(raw_time: u32) -> Self {
        DeltaTime {
            raw_time,
            delta: raw_time as f32 / 16700.0,
        }
    }

    pub fn get_raw(&self) -> u32 {
        self.raw_time
    }

    pub fn get_delta(&self) -> f32 {
        self.delta
    }
}
