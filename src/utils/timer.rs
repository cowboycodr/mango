use std::time::{self, Instant, Duration};

pub struct Timer {
    start: Instant
}

impl Timer {
    pub fn start() -> Self {
        Self {
            start: Instant::now()
        }
    }

    pub fn end(&self) -> Duration {
        Instant::now().duration_since(self.start)
    }
}