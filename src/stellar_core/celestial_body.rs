
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
    mut images: ResMut<Assets<Image>>,
) {

    let mut gen_p = |m, s, d, f, mg, x, y| {
        let planet = gen::generate_planet(m, s, d, f, mg);
        let radius = planet.radius;
        let tex_size = radius as u32 / 100;

        dbg!(radius);

        return (
            planet,
            Sprite { 
                image: gen::generate_planet_texture(tex_size, tex_size, &mut images), 
                custom_size: Some(Vec2::splat(radius as f32)),
                ..default()
            },
            Transform::from_xyz(x, y, 0.0)
        );
    };
    
    commands.spawn(gen_p(5.9e16, 1e30, 551500.0, 1.0, 1.0, 0.0, 0.0));
    commands.spawn(gen_p(1e15, 1e30, 551500.0, 1.0, 1.0, 4000.0, 12000.0));


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
