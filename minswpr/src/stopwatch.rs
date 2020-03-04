use std::time::Duration;
use std::time::Instant;

#[derive(new, Default)]
pub struct Stopwatch {
    #[new(default)]
    instant: Option<Instant>,
    #[new(default)]
    elapsed_final: Duration,
}

impl Stopwatch {
    pub fn start(&mut self) {
        *self = Self {
            instant: Some(Instant::now()),
            elapsed_final: Default::default(),
        };
    }

    pub fn stop(&mut self) {
        self.elapsed_final = self.elapsed();
        self.instant = None;
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn elapsed(&self) -> Duration {
        self.instant
            .map(|i| i.elapsed())
            .unwrap_or_else(|| self.elapsed_final)
    }
}
