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

use crate::stellar_core;
use crate::stellar_core::celestial_body::CelestialBody as CelestialBody;
use stellar_core::navigation::G as G;

use rand_distr::{Distribution, Normal};

const EARTH_RADIUS: f64 = 6.371e6;
const EARTH_MASS: f64 = 5.972e24;
const EARTH_GRAVITY: f64 = 9.7803267715;
const STEFAN_BOLTZMANN: f64 = 5.670374419e-8;

pub fn generate_planet(mass: f64, density: f64, solar_flux: f64, magnetic_field: f64) -> CelestialBody {

    //in meters
    let radius = f64::powf((3.0 * mass) / (4.0 * core::f64::consts::PI * density), 1.0 / 3.0);
    //in Gs
    let surface_gravity = (G as f64 * mass) / radius.powf(2.0) / EARTH_GRAVITY;

    //in m/s
    let escape_velocity = ((2.0 * G as f64 * mass) / radius).sqrt();

    let temp_base = 278.0 * solar_flux.sqrt();
    let mean_mol_weight = 28.97;
    let gas_retention_factor = 
        escape_velocity / ((3.0 * 1.380649e-23 * temp_base / (mean_mol_weight * 1.66053906660e-27)).sqrt());

    let nrm = Normal::new(mass / EARTH_MASS, 2.1).unwrap();
    let v: f64 = nrm.sample(&mut rand::rng()).abs();

    // Use gas retention to modulate atmosphere retention (0.0 to 1.0 scale)
    let retention_efficiency = (gas_retention_factor / 10.0).clamp(0.0, 1.0);

    // Solar flux and magnetic field affect atmospheric erosion or shielding
    let erosion_factor = (solar_flux - magnetic_field).max(0.0);
    let erosion_penalty = erosion_factor * (1.0 - retention_efficiency);

    // Atmos modifier represents net gain/loss potential for atmosphere
    let atmos_modifier = ((v * retention_efficiency) - erosion_penalty).clamp(0.0, 5.0);

    // Final atmospheric pressure in atm
    let atmos_pressure = atmos_modifier;

    // Temperature estimate
    let greenhouse_effect = atmos_modifier * 33.0; // 33K is Earth's greenhouse contribution
    let temp = temp_base + greenhouse_effect;

    // Albedo: depends on clouds, surface type, etc.
    let albedo = (0.1 + 0.2 * (1.0 - magnetic_field).clamp(0.0, 1.0)) * (1.0 - 0.2 * atmos_pressure.clamp(0.0, 5.0));
    let equilibrium_temp = ((solar_flux * (1.0 - albedo)) / (4.0 * STEFAN_BOLTZMANN)).powf(0.25);

    // Solar day length: increase with size (simplified)
    let solar_day_length = 24.0 * (radius / EARTH_RADIUS).sqrt();

    // Magnetic field strength vs core and rotation
    let rotation_speed = 1.0 / solar_day_length;
    let magnetic_intensity = (mass / EARTH_MASS).sqrt() * rotation_speed;

    // Plate tectonics estimation (requires liquid core and sufficient mass)
    let tectonic_activity = if mass > 0.5 * EARTH_MASS && temp > 200.0 {
        "active"
    } else {
        "stagnant"
    };

    // Habitability score: crude, composite metric
    let mut habitability = 1.0;
    habitability *= if (temp >= 273.0 && temp <= 373.0) { 1.0 } else { 0.5 };
    habitability *= if atmos_pressure > 0.1 && atmos_pressure < 5.0 { 1.0 } else { 0.3 };
    habitability *= if magnetic_intensity > 0.2 { 1.0 } else { 0.2 };
    habitability *= if albedo > 0.1 && albedo < 0.7 { 1.0 } else { 0.7 };

    // Composition estimation (based on density)
    let composition = match density {
        d if d < 3000.0 => "gas giant or ice world",
        d if d < 5500.0 => "rocky with volatile-rich crust",
        _ => "rocky with metallic core",
    };

    println!("{composition} with {radius}m, {surface_gravity}G, {escape_velocity}m/s, {atmos_pressure}atm, albedo: {albedo}, {temp}K, {tectonic_activity} tectonic activity; {habitability} hab score.");

    CelestialBody { mass, density, radius, surface_gravity, atmos: atmos_pressure } 
}