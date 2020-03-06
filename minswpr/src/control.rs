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

/// Manages the state of a button on the screen
#[derive(new)]
pub struct Button {
    #[new(default)]
    is_pressed: bool,
    #[new(value = "true")]
    is_released: bool,
}

impl Button {
    /// Returns true if the button is currently pressed and should be drawn to
    /// reflect it's pressed state. Returns false otherwise
    pub fn is_pressed(&self) -> bool {
        self.is_pressed
    }

    /// Sets whether if the button is currently pressed and should be drawn to
    /// reflect it's pressed state
    pub fn set_pressed(&mut self, is_pressed: bool) {
        self.is_pressed = is_pressed
    }

    /// Returns true if the button has been released since being pressed. If a
    /// button is pressed and the cursor moves out of the button area, we still
    /// might want to consider the button pressed when the cursor re-enters the
    /// button area as long as the mouse button was not released in between
    pub fn is_released(&self) -> bool {
        self.is_released
    }

    /// Sets if the button has been released
    pub fn set_released(&mut self, is_released: bool) {
        self.is_released = is_released
    }
}
