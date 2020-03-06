use std::time::Duration;
use std::time::Instant;

/// Tracks the elapsed time during an active game
#[derive(new, Default)]
pub struct Stopwatch {
    #[new(default)]
    instant: Option<Instant>,
    #[new(default)]
    elapsed_final: Duration,
}

impl Stopwatch {
    /// Starts this `Stopwatch`
    pub fn start(&mut self) {
        *self = Self {
            instant: Some(Instant::now()),
            elapsed_final: Default::default(),
        };
    }

    /// Sets `elapsed_final` to the current elapsed time and sets `instant` to
    /// `None`. If `Stopwatch::elapsed` is called after this, the elapsed
    /// duration at the time of this call will be returned until
    /// `Stopwatch::reset` is called
    pub fn stop(&mut self) {
        self.elapsed_final = self.elapsed();
        self.instant = None;
    }

    /// Resets this stopwatch
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    /// Returns the elapsed `Duration` since `Stopwatch::start` was called. If
    /// this `Stopwatch` has been stopped with `Stopwatch::stop`, the elapsed
    /// duration at time of stopping is returned
    pub fn elapsed(&self) -> Duration {
        self.instant
            .map(|i| i.elapsed())
            .unwrap_or_else(|| self.elapsed_final)
    }
}

#[derive(new)]
pub struct ResetButton {
    #[new(default)]
    is_pressed: bool,
}

impl ResetButton {
    pub fn is_pressed(&self) -> bool {
        self.is_pressed
    }
}
