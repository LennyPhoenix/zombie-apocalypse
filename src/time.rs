use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Time {
    hour: i32,
    day: i32,
}

impl Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Day {day}: {hour:0>2}:00 ({night})",
            day = self.day,
            hour = self.hour,
            night = if self.night() { "NIGHT" } else { "DAY" }
        )
    }
}

impl Time {
    pub fn day_zero() -> Self {
        Self { hour: 10, day: 0 }
    }

    pub fn advance(&mut self, time: i32) {
        self.hour += time;
        if self.hour > 23 {
            self.day += self.hour / 24;
            self.hour %= 24;
        }
    }

    pub fn night(&self) -> bool {
        self.hour < 7 || self.hour > 20
    }
}
