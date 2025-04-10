use bevy::prelude::*;
use crate::stellar_core;

pub fn add_ship(mut commands: Commands) {
    println!("Adding ship");
    commands.spawn((stellar_core::ship::Ship::new(), stellar_core::Name("USS Ligma".to_string())));
}

pub fn create_solar_system(mut commands: Commands) {
    println!("Adding earth & moon");
    commands.spawn( stellar_core::celestial_body::CelestialBody {mass: 5.9e15, position: Vec2 { x: -10.0, y: 0.0 }});
    commands.spawn(stellar_core::celestial_body::CelestialBody {mass: 1.0e15, position: Vec2 { x: 10.0, y: 10.0 }});
    commands.spawn(stellar_core::celestial_body::CelestialBody {mass: 5.0e15, position: Vec2 { x: -300.0, y: 40.0 }});
}

pub fn create_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}