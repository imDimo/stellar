use bevy::prelude::*;

use crate::stellar_core::celestial_body::{satellite::Satellite, orbit::Orbit};

use crate::procedural_generation::{self, gen_star as gen};

#[derive(Clone, Component, Debug)]
pub struct Star {
    pub mass: f64,
    pub radius: f64,
    pub luminosity: f64,
    pub temperature: f64,
    pub lifespan: f64,
    pub spectral_type: String,
    pub orbit: Orbit,
}

impl Default for Star {
    fn default() -> Self {
        Star {
            mass: 1.0, radius: 1.0, luminosity: 1.0, temperature: 1.0, lifespan: 1.0, 
            spectral_type: ".".to_string(), orbit: Orbit::default(),
        }
    }
}

impl Satellite for Star {
    fn get_orbit(self: &Self) -> Orbit {
        self.orbit
    }

    fn set_orbit(self: &mut Self, orbit: Orbit) {
        self.orbit = orbit;
    }
}

impl Star {
    pub fn new(mass: f64, age: f64, metallicity: f64) -> Star {
        gen::generate_star(mass, age, metallicity)
    }

    pub fn get_bundle(
        star: Self, x: f32, y: f32, mut images: &mut ResMut<Assets<Image>>
    ) -> (Self, Sprite, Transform) {
        let radius = star.radius;
        let tex_size = (radius as u32 * 100).max(1);

        (
            star,
            Sprite { 
                image: procedural_generation::circle_texture(tex_size, tex_size, &mut images), 
                custom_size: Some(Vec2::splat((radius as f32 * 500.0).max(1000.0))),
                ..default()
            },
            Transform::from_xyz(x, y, 0.0)
        )

    }
}