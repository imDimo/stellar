use bevy::prelude::*;

pub mod ship;
pub mod navigation;
pub mod celestial_body;

#[derive(Component)]
pub struct Name(pub String);
