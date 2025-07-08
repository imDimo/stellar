use bevy::prelude::*;

use crate::stellar_core::celestial_body::{satellite::Satellite, orbit::Orbit};

use crate::procedural_generation::{self, gen_planet as gen};

#[derive(Clone, Component, Debug)]
pub struct Planet {
    pub mass: f64,
    pub density: f64,
    pub radius: f64,
    pub surface_gravity: f64,
    pub atmos_pressure: f64,
    pub surface_temperature: f64,
    pub atmosphere_composition: Vec<(String, f64)>,
    pub magnetic_field_strength: f64,
    pub tectonic_activity: String,
    pub habitability: f64,
    pub orbit: Orbit,
}

impl Default for Planet {
    fn default() -> Self {
        Planet { 
            mass: 0.0, 
            density: 0.0, 
            radius: 0.0, 
            surface_gravity: 0.0, 
            atmos_pressure: 0.0, 
            surface_temperature: 0.0, 
            atmosphere_composition: vec![], 
            magnetic_field_strength: 0.0, 
            tectonic_activity: "".to_string(), 
            habitability: 0.0,
            orbit: Orbit::default()
        }
    }
}

impl Satellite for Planet {
    fn get_orbit(self: &Self) -> Orbit {
        self.orbit
    }

    fn set_orbit(self: &mut Self, orbit: Orbit) {
        self.orbit = orbit;
    }
}

impl Planet {
    pub fn new(mass:f64, density: f64, solar_flux: f64, magnetic_field: f64) -> Planet {
        gen::generate_planet(mass, density, solar_flux, magnetic_field)
    }

    pub fn get_bundle(
        planet: Self, x: f32, y: f32, mut images: &mut ResMut<Assets<Image>>
    ) -> (Self, Sprite, Transform) {
        let radius = planet.radius;
        let tex_size = radius as u32 / 100;

        (
            planet,
            Sprite { 
                image: procedural_generation::circle_texture(
                    tex_size, tex_size, &mut images, 0, 225, 255, 255),
                custom_size: Some(Vec2::splat(radius as f32)),
                ..default()
            },
            Transform::from_xyz(x, y, 0.0).with_scale(Vec3::splat(0.05))
        )
    }
}