use std::time::{Duration, Instant};

pub struct ButtonState {
    pressed_duration: Option<Instant>,
    handled: bool,
    key_down_was_noticed: bool
}

impl ButtonState {

    const HOLD_DURATION_MILLIS: u64 = 500; //todo change this value to make button hold more natural

    pub fn new() -> Self {
        Self {
            pressed_duration: None,
            handled: false,
            key_down_was_noticed: false
        }
    }

    pub fn key_down(&mut self) {
        if self.key_down_was_noticed {
            return;
        }
        self.pressed_duration = Some(Instant::now());
        self.key_down_was_noticed = true;
    }

    pub fn key_up(&mut self) {
        self.pressed_duration = None;
        self.handled = false;
        self.key_down_was_noticed = false;
    }

    pub fn should_handle_once(&self) -> bool {
        self.key_down_was_noticed && ! self.handled
    }

    pub fn handled_once(&mut self) {
        self.handled = true;
    }

    #[allow(dead_code)]
    pub fn is_short_pressed(&self) -> bool {
        match self.pressed_duration {
            Some(duration) => duration + Duration::from_millis(Self::HOLD_DURATION_MILLIS) > Instant::now(),
            None => false
        }
    }

    #[allow(dead_code)]
    pub fn is_long_pressed(&self) -> bool {
        match self.pressed_duration {
            Some(duration) => duration + Duration::from_millis(Self::HOLD_DURATION_MILLIS) <= Instant::now(),
            None => false
        }
    }

    #[allow(dead_code)]
    pub fn is_pressed(&self) -> bool {
        self.key_down_was_noticed
    }

}