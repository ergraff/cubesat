#[path = "./tests/time.rs"]
mod tests;

use serde::Deserialize;

pub static SECOND: f64 = 1.0;
pub static MINUTE: f64 = 60.0 * SECOND;
pub static HOUR: f64 = 60.0 * MINUTE;
pub static DAY: f64 = 24.0 * HOUR;

#[derive(Debug, PartialEq, Copy, Clone, Deserialize)]
pub struct Time {
    pub now: f64,
    pub step: f64,
    pub start: f64,
    pub end: f64,
}

impl Time {
    #[allow(unused)]
    pub fn new(start_time: f64, end_time: f64, time_step: f64) -> Self {
        Time {
            now: start_time,
            step: time_step,
            start: start_time,
            end: end_time, // 1 Day
        }
    }

    pub fn next(&mut self) {
        self.now += self.step;
    }
}
