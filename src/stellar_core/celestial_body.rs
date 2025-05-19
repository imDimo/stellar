
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

fn setup_solar_system(
    mut commands: Commands, 
    images: ResMut<Assets<Image>>,
    //asset_server : Res<AssetServer>
) {
    let planet_image = gen::generate_planet_texture(32, 32, images);
    let planet = gen::generate_planet(
        5.9e16, 1e30, 551500.0, 1.0, 1.0);
    let radius = planet.radius as f32;
    
    commands.spawn((
        planet,
        Sprite { image: planet_image, custom_size: Some(Vec2::splat(radius)), ..default() },
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
    pub atmos_pressure: f64,
    pub orbital_period: f64,
    pub surface_temperature: f64,
    pub atmosphere_composition: Vec<(String, f64)>,
    pub magnetic_field_strength: f64,
    pub tectonic_activity: String,
    pub habitability: f64,
}

impl Default for CelestialBody {
    fn default() -> Self {
        CelestialBody { 
            mass: 0.0, 
            density: 0.0, 
            radius: 0.0, 
            surface_gravity: 0.0, 
            atmos_pressure: 0.0, 
            orbital_period: 0.0, 
            surface_temperature: 0.0, 
            atmosphere_composition: vec![], 
            magnetic_field_strength: 0.0, 
            tectonic_activity: "".to_string(), 
            habitability: 0.0 
        }
    }
}
