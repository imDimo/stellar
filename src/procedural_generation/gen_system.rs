use base64::prelude::*;
use rand::Rng;

use crate::stellar_core::celestial_body::{planet::Planet, star::Star};
use crate::procedural_generation::{gen_planet::generate_planet as planet, gen_star::generate_star as star};

pub fn test_gen_system() {

    let seed = "=a=b=cii";

    dbg!(seed.len());

    gen_system(seed);
}

pub fn gen_system(seed: &str) -> (Vec<Star>, Vec<Planet>){
    let mut nr = Vec::<i32>::new();
    let seed_nr = get_nr_from_str(seed);

    for x in seed_nr.to_string().chars() {
        nr.push(x.to_string().parse().unwrap());
    }

    let mut stars = Vec::<Star>::new();
    let mut planets = Vec::<Planet>::new();
    let nth = |index| { nr.iter().nth(index % nr.len()).unwrap().clone() as f64 };

    let mut index = 0;
    while index < nr.len() {
        let digit = nth(index) as i32;

        if index == 0 {
            let star_amount = match digit {
                0 | 1 | 2 | 3 => 1,
                4 | 5 | 6 => 2,
                7 | 8 => 3,
                9 => 4,
                _ => unreachable!()
            };

            //for debugging.
            //star_amount = 1; 
            
            for i in 0..star_amount {
                let base = index + 1 + i * 3;
                stars.push(star(
                    get_rand_mass(0.01, 1.0, nth(index)) * 1e25,
                    get_rand_mass(0.01, 1.0,nth(base + 1)) * 1e4,
                    nth(base + 2) / 10.0,
                ));
            }
            index += 1 + 3 * star_amount;
        }
        else {
            match digit {
                6 => {}, //magic moon number
                _ => planets.push(planet(
                    get_rand_mass(0.01, 1.0, nth(index))  * 1e10,
                    nth(index + 1) * 200.0 + 3000.0,
                    rand::rng().random(),
                    rand::rng().random(),
                )),
            }
        }

        index += 1;
    }
    dbg!(seed_nr);

    if false {
        dbg!((stars, planets))
    }
    else {
        (stars, planets)
    }

}

fn get_nr_from_str(seed: &str) -> u128 {
    //basically this converts the string to base64 and then assigns each character the number of its index
    //strictly speaking this means certain numbers (like 7-9) are less likely to appear.

    //truncate to not exceed max size
    let truncated = seed
        .char_indices()
        .nth(230)
        .map_or(seed, |(i, _)| &seed[..i]);

    match truncated.parse::<u128>() {
        Ok(v) => return v,
        Err(_) => {}
    }

    let seed_b64 = BASE64_STANDARD.encode(truncated);
    let alphabet = "abcdefghijklmnopqrstuvwxyz.ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789+/=";

    let mut numeric_str = String::new();
    for c in seed_b64.chars() {
        if let Some(i) = alphabet.find(c) {
            numeric_str.push_str(&i.to_string());
        }
    }

    //insert decimal in the middle
    // let len = numeric_str.len();
    // let mid = len / 2;
    // let (left, right) = numeric_str.split_at(mid);
    // let combined = format!("{}.{}", left, right);

    numeric_str.parse::<u128>().unwrap_or(0)
}

fn get_rand_mass(min: f64, max: f64, seed: f64) -> f64 {
    let x = f64::sin(seed) * 172410.0;
    return (x - x.floor() * (max - min + 1.1) + min).abs();
}