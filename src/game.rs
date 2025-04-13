use bevy::prelude::*;



use crate::stellar_core::{camera, ship, celestial_body};
use crate::procedural_generation;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(
                (
                    camera::CameraPlugin,
                    ship::ShipPlugin,
                    celestial_body::SolarPlugin,
                )
            )
            .add_systems(Startup, setup_scene);
    }
}

fn setup_scene(mut commands: Commands) {
    commands.spawn(celestial_body::CelestialBody {mass: 5.9e10, position: Vec2 { x: 10.0, y: 0.0 }});
}