/*
    Note to self - these are the kinds of things this generator should be able to make:

    "Earth-Like-World / Land Rivers - A terran like planet with land, rivers and clouds.",
    "Ice World - Ice planet, with some water lakes, wind and clouds.",
    "Terran Dry - A mars-like rocky planet, close to its star, dried out of any water.",
    "Islands - Water planets covered in islands.",
    "No atmosphere - Moons or planets not protected by atmosphere.",
    "Gas Giant I - A cold planet, outside the frost line.",
    "Gas Giant II - A cold planet, outside the frost line, variation.",
    "Lava World - A protoplanet, perhaps too close to a star.",
*/

use crate::stellar_core::celestial_body::planet::Planet;
use crate::stellar_core::celestial_body::orbit::Orbit;

use rand_distr::{Distribution, Normal};

pub const EARTH_RADIUS: f64 = 6.371e6;
pub const EARTH_MASS: f64 = 5.972e24;
pub const EARTH_GRAVITY: f64 = 9.7803267715;
pub const STEFAN_BOLTZMANN: f64 = 5.670374419e-8;
pub const G: f64 = 6.6743015e-11;

pub fn generate_planet(mass: f64, density: f64, solar_flux: f64, magnetic_field: f64) -> Planet {

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

    //we wanna distribute the possible atmospheres normally, to get some more interesting generation
    let nrm = Normal::new(mass / EARTH_MASS, 2.1).unwrap();
    let v: f64 = nrm.sample(&mut rand::rng()).abs();

    //use gas retention to modulate atmosphere retention (0.0 to 1.0 scale)
    let retention_efficiency = (gas_retention_factor / 10.0).clamp(0.0, 1.0);

    //solar flux and magnetic field affect atmospheric erosion or shielding
    let erosion_factor = (solar_flux - magnetic_field).max(0.0);
    let erosion_penalty = erosion_factor * (1.0 - retention_efficiency);

    //atmos modifier represents net gain/loss potential for atmosphere
    let atmos_modifier = ((v * retention_efficiency) - erosion_penalty).clamp(0.0, 5.0);

    //final atmospheric pressure in atm
    let atmos_pressure = atmos_modifier;

    //temperature estimate
    let greenhouse_effect = atmos_modifier * 33.0; //33K is Earth's greenhouse contribution
    let temp = temp_base + greenhouse_effect;

    //albedo: depends on clouds, surface type, etc.
    let albedo = (0.1 + 0.2 * (1.0 - magnetic_field).clamp(0.0, 1.0)) * (1.0 - 0.2 * atmos_pressure.clamp(0.0, 5.0));
    let _equilibrium_temp = ((solar_flux * (1.0 - albedo)) / (4.0 * STEFAN_BOLTZMANN)).powf(0.25);

    //solar day length: just increase with size
    let solar_day_length = 24.0 * (radius / EARTH_RADIUS).sqrt();

    //magnetic field strength vs core and rotation
    let rotation_speed = 1.0 / solar_day_length;
    let magnetic_intensity = (mass / EARTH_MASS).sqrt() * rotation_speed;

    //plate tectonics estimation (requiring liquid core and sufficient mass etc)
    let tectonic_activity = if mass > 0.5 * EARTH_MASS && temp > 200.0 {
        "active"
    } else {
        "stagnant"
    };

    //habitability score: crude, composite metric
    let mut habitability = 1.0;
    habitability *= if temp >= 273.0 && temp <= 373.0 { 1.0 } else { 0.5 };
    habitability *= if atmos_pressure > 0.1 && atmos_pressure < 5.0 { 1.0 } else { 0.3 };
    habitability *= if magnetic_intensity > 0.2 { 1.0 } else { 0.2 };
    habitability *= if albedo > 0.1 && albedo < 0.7 { 1.0 } else { 0.7 };

    //composition estimation (based on density)
    let composition = match density {
        d if d < 3000.0 => "gas giant or ice world",
        d if d < 5500.0 => "rocky with volatile-rich crust",
        _ => "rocky with metallic core",
    };

    //dbg!((composition, radius, surface_gravity, escape_velocity, atmos_pressure, albedo, temp, _equilibrium_temp, tectonic_activity, habitability, orbital_period, semi_major_axis));

    //CelestialBody { ..default() }
    return Planet { 
        mass: mass, 
        density: density, 
        radius: radius, 
        surface_gravity: surface_gravity, 
        atmos_pressure: atmos_pressure, 
        surface_temperature: temp, 
        atmosphere_composition: vec![(composition.to_string(), 1.0)], 
        magnetic_field_strength: magnetic_field, 
        tectonic_activity: tectonic_activity.to_string(), 
        habitability: habitability,
        orbit: Orbit::default()
    };
}