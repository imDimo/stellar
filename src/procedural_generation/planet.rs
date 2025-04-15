/*
    "Earth-Like-World / Land Rivers - A terran like planet with land, rivers and clouds.",
    "Ice World - Ice planet, with some water lakes, wind and clouds.",
    "Terran Dry - A mars-like rocky planet, close to its star, dried out of any water.",
    "Islands - Water planets covered in islands.",
    "No atmosphere - Moons or planets not protected by atmosphere.",
    "Gas Giant I - A cold planet, outside the frost line.",
    "Gas Giant II - A cold planet, outside the frost line, variation.",
    "Lava World - A protoplanet, perhaps too close to a star.",
    "Asteroid - Fragment of matter roaming in space.",
    "Star - Huge hydrogen converters.",
*/

//use bevy::prelude::*;
use crate::stellar_core::{self, celestial_body::CelestialBody};

pub fn generate_planet(mass: f64, density: f64, solar_flux: f64, magnetic_field: f64) -> stellar_core::celestial_body::CelestialBody {
    let radius = f64::powf((3.0 * mass) / (4.0 * core::f64::consts::PI * density), 1.0 / 3.0);
    let surface_gravity = (stellar_core::navigation::G as f64 * mass) / radius.powf(2.0) / 9.7803267715;

    let atmos_modifier = f64::min((1.0 - f64::min((solar_flux - magnetic_field).abs(), 0.0)) * surface_gravity, 1.0);
    let atmos = rand::random_range(0.0..(2.0 + atmos_modifier)) * atmos_modifier;
    
    CelestialBody { mass, density, radius, surface_gravity, atmos } 
}