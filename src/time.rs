pub static SECOND: f64 = 1.0;
pub static MINUTE: f64 = 60.0 * SECOND;
pub static HOUR: f64 = 60.0 * MINUTE;

#[derive(Debug, PartialEq, Copy, Clone)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_zero() {
        let zero = Time::new(0.0, 0.0, 0.0);
        assert_eq!(zero.now, 0.0);
        assert_eq!(zero.step, 0.0);
        assert_eq!(zero.start, 0.0);
        assert_eq!(zero.end, 0.0);
    }

    #[test]
    fn hundred_steps() {
        let mut time = Time::new(0.0, 100.0, 1.0);
        for i in 0..100 {
            assert_eq!(time.now, i as f64);
            time.next();
        }
    }
}
