/*
    Note to self - these are the kinds of things this generator should be able to make.

    "Earth-Like-World / Land Rivers - A terran like planet with land, rivers and clouds.",
    "Ice World - Ice planet, with some water lakes, wind and clouds.",
    "Terran Dry - A mars-like rocky planet, close to its star, dried out of any water.",
    "Islands - Water planets covered in islands.",
    "No atmosphere - Moons or planets not protected by atmosphere.",
    "Gas Giant I - A cold planet, outside the frost line.",
    "Gas Giant II - A cold planet, outside the frost line, variation.",
    "Lava World - A protoplanet, perhaps too close to a star.",
*/

//use crate::stellar_core;
use crate::stellar_core::celestial_body::CelestialBody as CelestialBody;
use bevy::prelude::*;

use rand_distr::{Distribution, Normal};

const EARTH_RADIUS: f64 = 6.371e6;
const EARTH_MASS: f64 = 5.972e24;
const EARTH_GRAVITY: f64 = 9.7803267715;
const STEFAN_BOLTZMANN: f64 = 5.670374419e-8;
const G: f64 = 6.6743015e-11;

pub fn generate_planet(mass: f64, star_mass: f64, density: f64, solar_flux: f64, magnetic_field: f64) -> CelestialBody {

    //in meters
    let radius = f64::powf((3.0 * mass) / (4.0 * core::f64::consts::PI * density), 1.0 / 3.0);
    //in Gs
    let surface_gravity = (G as f64 * mass) / radius.powf(2.0) / EARTH_GRAVITY;

    //in m/s
    let escape_velocity = ((2.0 * G as f64 * mass) / radius).sqrt();

    //likely to re-do this later to settle the generator system
    let semi_major_axis = calculate_semi_major_axis(mass, star_mass, solar_flux);
    let orbital_period = calculate_orbital_period(semi_major_axis, star_mass, mass);

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
    return CelestialBody { 
        mass: mass, 
        density: density, 
        radius: radius, 
        surface_gravity: surface_gravity, 
        atmos_pressure: atmos_pressure, 
        orbital_period: orbital_period, 
        surface_temperature: temp, 
        atmosphere_composition: vec![(composition.to_string(), 1.0)], 
        magnetic_field_strength: magnetic_field, 
        tectonic_activity: tectonic_activity.to_string(), 
        habitability: habitability 
    };
}

//function to calculate semi-major axis using Kepler's Third Law
fn calculate_semi_major_axis(mass_planet: f64, mass_star: f64, orbital_period: f64) -> f64 {
    let numerator = (orbital_period.powi(2)) * G * (mass_star + mass_planet);
    let denominator = 4.0 * std::f64::consts::PI.powi(2);
    return (numerator / denominator).cbrt(); // Cube root to get the semi-major axis
}

//function to determine orbital period from semi-major axis and other parameters
fn calculate_orbital_period(semi_major_axis: f64, mass_star: f64, mass_planet: f64) -> f64 {
    let numerator = (semi_major_axis.powi(3)) * 4.0 * std::f64::consts::PI.powi(2);
    let denominator = G * (mass_star + mass_planet);
    return (numerator / denominator).sqrt();
}

//this is where tex gen will live. For now, simple orange outline.
pub fn generate_planet_texture(width: u32, height: u32, images: &mut ResMut<Assets<Image>>) -> Handle<Image> {

    //create an image buffer
    let mut imgbuf: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = 
        image::ImageBuffer::new(width, height);

    //calculate center and radius of circle
    let radius = width as f32 / 2.0;
    let c_x = width as f32 / 2.0;
    let c_y = height as f32 / 2.0;

    //iterate thru all pixels
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        //get squared distance from the center
        let distance_squared = (x as f32 - c_x).powi(2) + (y as f32 - c_y).powi(2);

        //if it falls in this small range, then color it whatever color
        if distance_squared >= (radius - 1.0).powi(2) && distance_squared <= radius.powi(2) {
            *pixel = image::Rgba([255, 165, 0, 255]);
        }
    }

    //convert the image::Image into bevy_image::image::Image via image::DynamicImage
    //lol
    let img = Image::from_dynamic(
        image::DynamicImage::ImageRgba8(imgbuf),
        false,
        bevy::asset::RenderAssetUsages::RENDER_WORLD | bevy::asset::RenderAssetUsages::MAIN_WORLD
    );
    //lastly add it to the thing 
    let img_handle = images.add(img);

    //to get the thing
    return img_handle;
}