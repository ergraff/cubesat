#[allow(unused_imports)]
use crate::vector::*;

#[test]
fn origin() {
    let origin = Vector3::origin();
    assert_eq!(origin.x, 0.0);
    assert_eq!(origin.y, 0.0);
    assert_eq!(origin.z, 0.0);
}

#[test]
fn unit_vectors() {
    let unit_x = Vector3::new(1.0, 0.0, 0.0);
    let unit_y = Vector3::new(0.0, 1.0, 0.0);
    let unit_z = Vector3::new(0.0, 0.0, 1.0);
    assert_eq!(unit_x.x, 1.0);
    assert_eq!(unit_y.y, 1.0);
    assert_eq!(unit_z.z, 1.0);
}

#[test]
fn negative() {
    let origin = Vector3::origin();
    let pos_unit_x = Vector3::new(1.0, 0.0, 0.0);
    let pos_unit_y = Vector3::new(0.0, 1.0, 0.0);
    let pos_unit_z = Vector3::new(0.0, 0.0, 1.0);
    let pos_all_one = Vector3::new(1.0, 1.0, 1.0);
    let neg_unit_x = Vector3::new(-1.0, 0.0, 0.0);
    let neg_unit_y = Vector3::new(0.0, -1.0, 0.0);
    let neg_unit_z = Vector3::new(0.0, 0.0, -1.0);
    let neg_all_one = Vector3::new(-1.0, -1.0, -1.0);
    assert_eq!(origin.negative(), origin);
    assert_eq!(pos_unit_x.negative(), neg_unit_x);
    assert_eq!(pos_unit_y.negative(), neg_unit_y);
    assert_eq!(pos_unit_z.negative(), neg_unit_z);
    assert_eq!(pos_all_one.negative(), neg_all_one);
}

#[test]
fn abs() {
    let origin = Vector3::origin();
    let pos_unit_x = Vector3::new(1.0, 0.0, 0.0);
    let pos_unit_y = Vector3::new(0.0, 1.0, 0.0);
    let pos_unit_z = Vector3::new(0.0, 0.0, 1.0);
    let pos_all_one = Vector3::new(1.0, 1.0, 1.0);
    let neg_unit_x = Vector3::new(-1.0, 0.0, 0.0);
    let neg_unit_y = Vector3::new(0.0, -1.0, 0.0);
    let neg_unit_z = Vector3::new(0.0, 0.0, -1.0);
    let neg_all_one = Vector3::new(-1.0, -1.0, -1.0);

    assert_eq!(origin.abs(), 0.0);
    assert_eq!(pos_unit_x.abs(), 1.0);
    assert_eq!(pos_unit_y.abs(), 1.0);
    assert_eq!(pos_unit_z.abs(), 1.0);
    assert_eq!(pos_all_one.abs(), 3.0_f64.sqrt());
    assert_eq!(neg_unit_x.abs(), 1.0);
    assert_eq!(neg_unit_y.abs(), 1.0);
    assert_eq!(neg_unit_z.abs(), 1.0);
    assert_eq!(neg_all_one.abs(), 3.0_f64.sqrt());
}

#[test]
fn dot() {
    let origin = Vector3::origin();
    let pos_unit_x = Vector3::new(1.0, 0.0, 0.0);
    let pos_unit_y = Vector3::new(0.0, 1.0, 0.0);
    let pos_unit_z = Vector3::new(0.0, 0.0, 1.0);
    let pos_all_one = Vector3::new(1.0, 1.0, 1.0);
    let neg_unit_x = Vector3::new(-1.0, 0.0, 0.0);
    let neg_unit_y = Vector3::new(0.0, -1.0, 0.0);
    let neg_unit_z = Vector3::new(0.0, 0.0, -1.0);
    let neg_all_one = Vector3::new(-1.0, -1.0, -1.0);

    assert_eq!(origin.dot(&origin), 0.0);

    assert_eq!(pos_unit_x.dot(&pos_unit_x), 1.0);
    assert_eq!(pos_unit_x.dot(&pos_unit_y), 0.0);
    assert_eq!(pos_unit_x.dot(&pos_unit_z), 0.0);
    assert_eq!(pos_unit_x.dot(&pos_all_one), 1.0);
    assert_eq!(pos_unit_x.dot(&origin), 0.0);
    assert_eq!(pos_unit_y.dot(&pos_unit_x), 0.0);
    assert_eq!(pos_unit_y.dot(&pos_unit_y), 1.0);
    assert_eq!(pos_unit_y.dot(&pos_unit_z), 0.0);
    assert_eq!(pos_unit_y.dot(&pos_all_one), 1.0);
    assert_eq!(pos_unit_y.dot(&origin), 0.0);
    assert_eq!(pos_unit_z.dot(&pos_unit_x), 0.0);
    assert_eq!(pos_unit_z.dot(&pos_unit_y), 0.0);
    assert_eq!(pos_unit_z.dot(&pos_unit_z), 1.0);
    assert_eq!(pos_unit_z.dot(&pos_all_one), 1.0);
    assert_eq!(pos_unit_z.dot(&origin), 0.0);

    assert_eq!(neg_unit_x.dot(&neg_unit_x), 1.0);
    assert_eq!(neg_unit_x.dot(&neg_unit_y), 0.0);
    assert_eq!(neg_unit_x.dot(&neg_unit_z), 0.0);
    assert_eq!(neg_unit_x.dot(&neg_all_one), 1.0);
    assert_eq!(neg_unit_x.dot(&origin), 0.0);
    assert_eq!(neg_unit_y.dot(&neg_unit_x), 0.0);
    assert_eq!(neg_unit_y.dot(&neg_unit_y), 1.0);
    assert_eq!(neg_unit_y.dot(&neg_unit_z), 0.0);
    assert_eq!(neg_unit_y.dot(&neg_all_one), 1.0);
    assert_eq!(neg_unit_y.dot(&origin), 0.0);
    assert_eq!(neg_unit_z.dot(&neg_unit_x), 0.0);
    assert_eq!(neg_unit_z.dot(&neg_unit_y), 0.0);
    assert_eq!(neg_unit_z.dot(&neg_unit_z), 1.0);
    assert_eq!(neg_unit_z.dot(&neg_all_one), 1.0);
    assert_eq!(neg_unit_z.dot(&origin), 0.0);

    assert_eq!(pos_unit_x.dot(&neg_unit_x), -1.0);
    assert_eq!(pos_unit_x.dot(&neg_unit_y), 0.0);
    assert_eq!(pos_unit_x.dot(&neg_unit_z), 0.0);
    assert_eq!(pos_unit_x.dot(&neg_all_one), -1.0);
    assert_eq!(pos_unit_y.dot(&neg_unit_x), 0.0);
    assert_eq!(pos_unit_y.dot(&neg_unit_y), -1.0);
    assert_eq!(pos_unit_y.dot(&neg_unit_z), 0.0);
    assert_eq!(pos_unit_y.dot(&neg_all_one), -1.0);
    assert_eq!(pos_unit_z.dot(&neg_unit_x), 0.0);
    assert_eq!(pos_unit_z.dot(&neg_unit_y), 0.0);
    assert_eq!(pos_unit_z.dot(&neg_unit_z), -1.0);
    assert_eq!(pos_unit_z.dot(&neg_all_one), -1.0);

    assert_eq!(pos_all_one.dot(&pos_all_one), 3.0);
    assert_eq!(pos_all_one.dot(&neg_all_one), -3.0);
    assert_eq!(neg_all_one.dot(&neg_all_one), 3.0);
    assert_eq!(pos_all_one.dot(&origin), 0.0);
    assert_eq!(neg_all_one.dot(&origin), 0.0);
}

#[test]
fn angle_to() {
    let origin = Vector3::origin();
    let pos_unit_x = Vector3::new(1.0, 0.0, 0.0);
    let pos_unit_y = Vector3::new(0.0, 1.0, 0.0);
    let pos_unit_z = Vector3::new(0.0, 0.0, 1.0);
    let pos_all_one = Vector3::new(1.0, 1.0, 1.0);
    let neg_unit_x = Vector3::new(-1.0, 0.0, 0.0);
    let neg_unit_y = Vector3::new(0.0, -1.0, 0.0);
    let neg_unit_z = Vector3::new(0.0, 0.0, -1.0);
    let neg_all_one = Vector3::new(-1.0, -1.0, -1.0);

    assert!(origin.angle_to(&origin).is_nan());
    assert!(origin.angle_to(&pos_unit_x).is_nan());
    assert!(origin.angle_to(&pos_unit_y).is_nan());
    assert!(origin.angle_to(&pos_unit_z).is_nan());
    assert!(origin.angle_to(&pos_all_one).is_nan());
    assert!(origin.angle_to(&neg_unit_x).is_nan());
    assert!(origin.angle_to(&neg_unit_y).is_nan());
    assert!(origin.angle_to(&neg_unit_z).is_nan());
    assert!(origin.angle_to(&neg_all_one).is_nan());

    assert_eq!(pos_unit_x.angle_to(&pos_unit_x), 0.0);
    assert_eq!(
        pos_unit_x.angle_to(&pos_unit_y),
        std::f64::consts::FRAC_PI_2
    );
    assert_eq!(
        pos_unit_x.angle_to(&pos_unit_z),
        std::f64::consts::FRAC_PI_2
    );
    assert_eq!(
        pos_unit_x.angle_to(&pos_all_one),
        (1.0 / (3.0_f64.sqrt())).acos()
    );
    assert_eq!(pos_unit_x.angle_to(&neg_unit_x), std::f64::consts::PI);
    assert_eq!(pos_unit_y.angle_to(&neg_unit_y), std::f64::consts::PI);
    assert_eq!(pos_unit_z.angle_to(&neg_unit_z), std::f64::consts::PI);

    assert_eq!(pos_all_one.angle_to(&neg_all_one), std::f64::consts::PI);
    assert_eq!(neg_all_one.angle_to(&pos_all_one), std::f64::consts::PI);
}

#[test]
fn rot_x() {
    let origin = Vector3::origin();
    let pos_unit_x = Vector3::new(1.0, 0.0, 0.0);
    let pos_unit_y = Vector3::new(0.0, 1.0, 0.0);
    let pos_unit_z = Vector3::new(0.0, 0.0, 1.0);
    let neg_unit_x = Vector3::new(-1.0, 0.0, 0.0);
    let neg_unit_y = Vector3::new(0.0, -1.0, 0.0);
    let neg_unit_z = Vector3::new(0.0, 0.0, -1.0);

    assert_eq!(origin.rot_x(std::f64::consts::FRAC_PI_2), origin);
    assert_eq!(origin.rot_x(std::f64::consts::PI), origin);

    assert_eq!(pos_unit_x.rot_x(std::f64::consts::PI), pos_unit_x);
    assert_eq!(neg_unit_x.rot_x(std::f64::consts::PI), neg_unit_x);

    assert_eq!(pos_unit_y.rot_x(std::f64::consts::FRAC_PI_2), pos_unit_z);
    assert_eq!(pos_unit_y.rot_x(std::f64::consts::PI), neg_unit_y);

    assert_eq!(pos_unit_z.rot_x(std::f64::consts::FRAC_PI_2), neg_unit_y);
    assert_eq!(pos_unit_z.rot_x(std::f64::consts::PI), neg_unit_z);
}

#[test]
fn rot_y() {
    let origin = Vector3::origin();
    let pos_unit_x = Vector3::new(1.0, 0.0, 0.0);
    let pos_unit_y = Vector3::new(0.0, 1.0, 0.0);
    let pos_unit_z = Vector3::new(0.0, 0.0, 1.0);
    let neg_unit_x = Vector3::new(-1.0, 0.0, 0.0);
    let neg_unit_y = Vector3::new(0.0, -1.0, 0.0);
    let neg_unit_z = Vector3::new(0.0, 0.0, -1.0);

    assert_eq!(origin.rot_y(std::f64::consts::FRAC_PI_2), origin);
    assert_eq!(origin.rot_y(std::f64::consts::PI), origin);

    assert_eq!(pos_unit_x.rot_y(std::f64::consts::FRAC_PI_2), neg_unit_z);
    assert_eq!(pos_unit_x.rot_y(std::f64::consts::PI), neg_unit_x);

    assert_eq!(pos_unit_y.rot_y(std::f64::consts::PI), pos_unit_y);
    assert_eq!(neg_unit_y.rot_y(std::f64::consts::PI), neg_unit_y);

    assert_eq!(pos_unit_z.rot_y(std::f64::consts::FRAC_PI_2), pos_unit_x);
    assert_eq!(pos_unit_z.rot_y(std::f64::consts::PI), neg_unit_z);
}

#[test]
fn rot_z() {
    let origin = Vector3::origin();
    let pos_unit_x = Vector3::new(1.0, 0.0, 0.0);
    let pos_unit_y = Vector3::new(0.0, 1.0, 0.0);
    let pos_unit_z = Vector3::new(0.0, 0.0, 1.0);
    let neg_unit_x = Vector3::new(-1.0, 0.0, 0.0);
    let neg_unit_y = Vector3::new(0.0, -1.0, 0.0);
    let neg_unit_z = Vector3::new(0.0, 0.0, -1.0);

    assert_eq!(origin.rot_z(std::f64::consts::FRAC_PI_2), origin);
    assert_eq!(origin.rot_z(std::f64::consts::PI), origin);

    assert_eq!(pos_unit_x.rot_z(std::f64::consts::FRAC_PI_2), pos_unit_y);
    assert_eq!(pos_unit_x.rot_z(std::f64::consts::PI), neg_unit_x);

    assert_eq!(pos_unit_y.rot_z(std::f64::consts::FRAC_PI_2), neg_unit_x);
    assert_eq!(pos_unit_y.rot_z(std::f64::consts::PI), neg_unit_y);

    assert_eq!(pos_unit_z.rot_z(std::f64::consts::PI), pos_unit_z);
    assert_eq!(neg_unit_z.rot_z(std::f64::consts::PI), neg_unit_z);
}

#[test]
fn rot_xyz() {
    let origin = Vector3::origin();
    let pos_unit_x = Vector3::new(1.0, 0.0, 0.0);
    let pos_unit_y = Vector3::new(0.0, 1.0, 0.0);
    let pos_unit_z = Vector3::new(0.0, 0.0, 1.0);
    let neg_unit_z = Vector3::new(0.0, 0.0, -1.0);

    assert_eq!(
        origin
            .rot_x(std::f64::consts::PI)
            .rot_y(std::f64::consts::PI)
            .rot_z(std::f64::consts::PI),
        origin
    );
    assert_eq!(
        pos_unit_x
            .rot_x(std::f64::consts::FRAC_PI_2)
            .rot_y(std::f64::consts::FRAC_PI_2)
            .rot_z(std::f64::consts::FRAC_PI_2),
        neg_unit_z
    );
    assert_eq!(
        pos_unit_y
            .rot_x(std::f64::consts::FRAC_PI_2)
            .rot_y(std::f64::consts::FRAC_PI_2)
            .rot_z(std::f64::consts::FRAC_PI_2),
        pos_unit_y
    );
    assert_eq!(
        pos_unit_z
            .rot_x(std::f64::consts::FRAC_PI_2)
            .rot_y(std::f64::consts::FRAC_PI_2)
            .rot_z(std::f64::consts::FRAC_PI_2),
        pos_unit_x
    );
}
