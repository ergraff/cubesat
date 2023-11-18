#[allow(unused_imports)]
use crate::component::*;

#[test]
fn panel_new() {
    let panel_zero = SolarPanel::new(0.0, 0.0, 0.0, 0.0);
    let panel_x = SolarPanel::new(0.0, 1.0, 0.0, 0.0);
    let panel_y = SolarPanel::new(0.0, 0.0, 1.0, 0.0);
    let panel_z = SolarPanel::new(0.0, 0.0, 0.0, 1.0);
    assert_eq!(panel_zero.orientation, vector::Vector3::new(0.0, 0.0, 0.0));
    assert_eq!(panel_x.orientation, vector::Vector3::new(1.0, 0.0, 0.0));
    assert_eq!(panel_y.orientation, vector::Vector3::new(0.0, 1.0, 0.0));
    assert_eq!(panel_z.orientation, vector::Vector3::new(0.0, 0.0, 1.0));
}

#[test]
fn power_generation() {
    let panel_pos_x = SolarPanel::new(1.0, 1.0, 0.0, 0.0);
    let panel_neg_x = SolarPanel::new(1.0, -1.0, 0.0, 0.0);
    let panel_pos_y = SolarPanel::new(1.0, 0.0, 1.0, 0.0);
    let panel_neg_y = SolarPanel::new(1.0, 0.0, -1.0, 0.0);
    let panel_pos_z = SolarPanel::new(1.0, 0.0, 0.0, 1.0);
    let panel_neg_z = SolarPanel::new(1.0, 0.0, 0.0, -1.0);
    let sun_pos_x = vector::Vector3::new(1.0, 0.0, 0.0);
    let sun_neg_x = vector::Vector3::new(-1.0, 0.0, 0.0);
    let sun_pos_y = vector::Vector3::new(0.0, 1.0, 0.0);
    let sun_neg_y = vector::Vector3::new(0.0, -1.0, 0.0);
    let sun_pos_z = vector::Vector3::new(0.0, 0.0, 1.0);
    let sun_neg_z = vector::Vector3::new(0.0, 0.0, -1.0);
    let rotation = vector::Vector3::new(0.0, 0.0, 0.0);

    assert_eq!(panel_pos_x.power_generation(&rotation, &sun_pos_x), 0.0);
    assert_eq!(panel_pos_x.power_generation(&rotation, &sun_neg_x), 1.0);
    assert_eq!(panel_pos_x.power_generation(&rotation, &sun_pos_y), 0.0);
    assert_eq!(panel_pos_x.power_generation(&rotation, &sun_neg_y), 0.0);
    assert_eq!(panel_pos_x.power_generation(&rotation, &sun_pos_z), 0.0);
    assert_eq!(panel_pos_x.power_generation(&rotation, &sun_neg_z), 0.0);

    assert_eq!(panel_neg_x.power_generation(&rotation, &sun_pos_x), 1.0);
    assert_eq!(panel_neg_x.power_generation(&rotation, &sun_neg_x), 0.0);
    assert_eq!(panel_neg_x.power_generation(&rotation, &sun_pos_y), 0.0);
    assert_eq!(panel_neg_x.power_generation(&rotation, &sun_neg_y), 0.0);
    assert_eq!(panel_neg_x.power_generation(&rotation, &sun_pos_z), 0.0);
    assert_eq!(panel_neg_x.power_generation(&rotation, &sun_neg_z), 0.0);

    assert_eq!(panel_pos_y.power_generation(&rotation, &sun_pos_x), 0.0);
    assert_eq!(panel_pos_y.power_generation(&rotation, &sun_neg_x), 0.0);
    assert_eq!(panel_pos_y.power_generation(&rotation, &sun_pos_y), 0.0);
    assert_eq!(panel_pos_y.power_generation(&rotation, &sun_neg_y), 1.0);
    assert_eq!(panel_pos_y.power_generation(&rotation, &sun_pos_z), 0.0);
    assert_eq!(panel_pos_y.power_generation(&rotation, &sun_neg_z), 0.0);

    assert_eq!(panel_neg_y.power_generation(&rotation, &sun_pos_x), 0.0);
    assert_eq!(panel_neg_y.power_generation(&rotation, &sun_neg_x), 0.0);
    assert_eq!(panel_neg_y.power_generation(&rotation, &sun_pos_y), 1.0);
    assert_eq!(panel_neg_y.power_generation(&rotation, &sun_neg_y), 0.0);
    assert_eq!(panel_neg_y.power_generation(&rotation, &sun_pos_z), 0.0);
    assert_eq!(panel_neg_y.power_generation(&rotation, &sun_neg_z), 0.0);

    assert_eq!(panel_pos_z.power_generation(&rotation, &sun_pos_x), 0.0);
    assert_eq!(panel_pos_z.power_generation(&rotation, &sun_neg_x), 0.0);
    assert_eq!(panel_pos_z.power_generation(&rotation, &sun_pos_y), 0.0);
    assert_eq!(panel_pos_z.power_generation(&rotation, &sun_neg_y), 0.0);
    assert_eq!(panel_pos_z.power_generation(&rotation, &sun_pos_z), 0.0);
    assert_eq!(panel_pos_z.power_generation(&rotation, &sun_neg_z), 1.0);

    assert_eq!(panel_neg_z.power_generation(&rotation, &sun_pos_x), 0.0);
    assert_eq!(panel_neg_z.power_generation(&rotation, &sun_neg_x), 0.0);
    assert_eq!(panel_neg_z.power_generation(&rotation, &sun_pos_y), 0.0);
    assert_eq!(panel_neg_z.power_generation(&rotation, &sun_neg_y), 0.0);
    assert_eq!(panel_neg_z.power_generation(&rotation, &sun_pos_z), 1.0);
    assert_eq!(panel_neg_z.power_generation(&rotation, &sun_neg_z), 0.0);
}

#[test]
fn eps() {
    let mut eps = Eps::new(-1.0, 20.0);
    assert_eq!(eps.consumption, -1.0);
    assert_eq!(eps.charge, 20.0);
    assert_eq!(eps.max_charge, 20.0);

    let timestep = 1.0;
    eps.update_capacity(1.0, timestep);
    assert_eq!(eps.charge, eps.max_charge);

    eps.update_capacity(-1.0, timestep);
    assert_eq!(
        eps.charge,
        (-1.0 * timestep + (eps.max_charge * time::HOUR)) / time::HOUR
    );
}

#[test]
fn component() {
    let comp = Component::new("ADCS", -1.0);
    assert_eq!(comp.name, "ADCS".to_string());
    assert_eq!(comp.consumption, -1.0);
}
