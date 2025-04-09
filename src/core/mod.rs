use bevy::prelude::*;

pub mod ship;
pub mod navigation;

#[derive(Component)]
pub struct Name(pub String);