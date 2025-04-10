use bevy::prelude::*;
use crate::stellar_core;

pub fn list_ship_position(query: Single<&stellar_core::ship::Ship>) {
    println!("{}", query.to_string())
}

pub fn update_ship(mut ship: Single<&mut stellar_core::ship::Ship>, bodies: Query<&mut stellar_core::celestial_body::CelestialBody>) {
    let velocity = stellar_core::navigation::calculate_acceleration(&ship.position, &bodies.iter().collect());
    ship.velocity += velocity;
    ship.position += velocity;
}

pub fn camera_movement() {
    
}