
use bevy::prelude::*;
use crate::procedural_generation::planet as gen;

pub struct SolarPlugin;
impl Plugin for SolarPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_solar_system)
            .add_systems(Update, update_solar_system);
    }
}

fn setup_solar_system(mut commands: Commands, asset_server : Res<AssetServer>) {
    let planet_image = asset_server.load("icon.png");
    let planet = gen::generate_planet(5.9e16, 551500.0, 1.0, 1.0);
    commands.spawn((
        planet.clone(),
        Sprite { image: planet_image, custom_size: Some(Vec2::splat(planet.radius as f32)), ..default() },
        Transform::from_xyz(1000.0, 0.0, 0.0)
        ));
}

fn update_solar_system(_bodies: Query<&mut CelestialBody>, mut _gizmos: Gizmos) {

}

#[allow(dead_code)]
#[derive(Clone, Component, Debug)]
pub struct CelestialBody {
    pub mass: f64,
    pub density: f64,
    pub radius: f64,
    pub surface_gravity: f64,
    pub atmos: f64
}
