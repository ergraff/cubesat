pub struct Time {
    pub now: f64,
    pub step: f64,
    pub start: f64,
    pub end: f64,
}

impl Time {
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

pub fn foo() {
    println!("Hello from time.rs");
}
