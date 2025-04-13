use bevy::prelude::*;
use super::celestial_body::CelestialBody;

const G: f32 = 6.6743015e-11;

pub fn calculate_acceleration(position: &Vec2, bodies: &Vec<&CelestialBody>) -> Vec2 {

    let mut accel = Vec2::new(0.0,0.0);

    for body in bodies {
        let dv = body.position - *position;

        let distance = dv.length();
        let accel_magnitude = G * body.mass / dv.length_squared();

        if distance == 0.0 {
            accel = Vec2::new(0.0, 0.0);
            println!("MEOW");
        }
        else {
            accel = accel + Vec2::new(accel_magnitude * dv.x / distance, accel_magnitude * dv.y / distance);
        }
    }

    return accel;
}