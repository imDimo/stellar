use bevy::prelude::*;

pub mod ship;
pub mod navigation;
pub mod celestial_body;
pub mod camera;

#[derive(Component)]
pub struct Name(pub String);

#[derive(Component)]
pub struct Velocity(pub Vec2);
