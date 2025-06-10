use bevy::prelude::*;

use crate::stellar_core::{camera, ship, celestial_body};
//use crate::procedural_generation;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                    camera::CameraPlugin,
                    ship::ShipPlugin,
                    celestial_body::SolarSystemPlugin,
                )
            );
    }
}