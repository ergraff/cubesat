#[allow(unused_imports)]
use crate::time::*;

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
