use bevy::prelude::*;



use crate::stellar_core::{camera, ship, celestial_body};
//use crate::procedural_generation;

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

fn setup_scene(mut commands: Commands, mut gizmos: Gizmos) {
    gizmos.circle_2d(Vec2::splat(0.0), 10.0 as f32, bevy::color::palettes::basic::WHITE);
}