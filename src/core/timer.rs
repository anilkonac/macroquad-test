pub struct Timer {
    accumulator: f32,
    rate: f32,
}

impl Timer {
    pub fn new(rate: f32) -> Timer {
        Self {
            accumulator: rate,
            rate,
        }
    }

    pub fn update(&mut self, dt: f32) -> bool {
        self.accumulator += dt;
        let diff = self.accumulator - self.rate;
        if diff > 0.0 {
            self.accumulator = diff;
            return true;
        }
        false
    }

    #[inline]
    pub fn reset(&mut self) {
        self.accumulator = self.rate;
    }
}
