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

use bevy::prelude::*;

pub fn test_planet() {
    planet(5.9e24, 5515.0, 1.0); //earth!
}

fn planet(mass: f64, density: f64, solar_flux: f64) {
    const G: f64 = 6.6743015e-11;
    let radius = f64::powf((3.0 * mass) / (4.0 * core::f64::consts::PI * density), 1.0 / 3.0);
    let surface_gravity = (G * mass) / radius.powf(2.0) / 9.7803267715;
    
    println!("{:#?}", (radius, surface_gravity));
}

/* 
#[derive(Debug, Clone)]
enum PlanetClass {
    EarthLike,
    IceWorld,
    TerranDry,
    Islands,
    NoAtmosphere,
    GasGiant,
    IceGiant,
    LavaWorld,
}

#[derive(Debug)]
struct Planet {
    name: String,
    class: PlanetClass,
    radius: f32,
    density: f32,
    mass: f64,
    temperature: i32,
    surface_texture: Vec<Vec2>,
}

impl Planet {
    fn generate_class() -> PlanetClass {
        let classes = [
            PlanetClass::EarthLike,
            PlanetClass::IceWorld,
            PlanetClass::TerranDry,
            PlanetClass::Islands,
            PlanetClass::NoAtmosphere,
            PlanetClass::GasGiant,
            PlanetClass::IceGiant,
            PlanetClass::LavaWorld,
        ];

        use rand::Rng;
        let mut rng = rand::thread_rng();
        //classes[rng.gen_range(0..8)];

        PlanetClass::Islands
    }

    fn calculate_mass(radius: f32, density: f32) -> f64 {
        // Volume of a sphere (4/3 * π * r^3)
        let volume = (4.0 / 3.0f32) * std::f32::consts::PI as f32 * radius.powi(3);
        (volume as f64 * density as f64) / 1000.0 // Convert to kg
    }

    fn generate_temperature(class: &PlanetClass) -> i32 {
        match class {
            PlanetClass::EarthLike => 15,
            PlanetClass::IceWorld => -20,
            PlanetClass::TerranDry => 20,
            PlanetClass::Islands => 22,
            PlanetClass::NoAtmosphere => -170,
            PlanetClass::GasGiant => -180,
            PlanetClass::IceGiant => -150,
            PlanetClass::LavaWorld => 1200,
        }
    }

    fn generate_surface_texture(&self) -> Vec<Vec2> {
        let mut img = ImageBuilder::new(64, 64).with_rgba8().build().unwrap();
        let noise_map = Self::generate_noise_map(64);

        for y in 0..64 {
            for x in 0..64 {
                // Generate height and temperature values using Perlin noise
                let height = noise_map.get_height(x as f32, y as f32) * 0.5 + 0.5;
                let temp = noise_map.get_temp(x as f32, y as f32);

                // Convert to color based on height and temperature
                let r = (height + temp).sin() * 127.0 + 128.0;
                let g = height.cos() * 127.0 + 128.0;
                let b = temp.sin() * 127.0 + 128.0;

                img[(x, y)] = (r as u8, g as u8, b as u8, 255);
            }
        }

        // Convert image to Vec<Vec2> for texture coordinates
        let mut surface_texture = Vec::with_capacity(64 * 64);
        for y in 0..64 {
            for x in 0..64 {
                let (r, g, b, _) = img[(x, y)];
                surface_texture.push(Vec2::new((x as f32) / 64.0, (y as f32) / 64.0));
            }
        }

        surface_texture
    }

    fn generate_noise_map(size: usize) -> Simplex {
        let mut simplex = Simplex::new(123);
        simplex.set_scale(0.05);
        simplex
    }
}

fn main() {
    // Generate a random planet
    let planet = Planet {
        name: "Random Generated Planet".to_string(),
        class: Planet::generate_class(),
        radius: 6000.0, // Earth-like radius as reference
        density: 5514.0, // Average planetary density (kg/m^3)
        mass: Planet::calculate_mass(6000.0, 5514.0),
        temperature: Planet::generate_temperature(&planet.class),
        surface_texture: planet.generate_surface_texture(),
    };

    // Log the generated planet's properties
    println!("Planet Name: {}", planet.name);
    println!("Class: {:?}", planet.class);
    println!("Radius: {:.2} km", planet.radius / 1000.0);
    println!("Density: {:.2} kg/m³", planet.density);
    println!("Mass: {:.2} x 10^24 kg", planet.mass / 1e24);
    println!("Temperature: {}°C", planet.temperature);

    // Save the surface texture as a PNG file
    let mut img = ImageBuilder::new(64, 64).with_rgba8().build().unwrap();
    for y in 0..64 {
        for x in 0..64 {
            img[(x, y)] = (255, 255, 255, 255);
        }
    }
    img.save("planet_surface.png").unwrap();
}

*/