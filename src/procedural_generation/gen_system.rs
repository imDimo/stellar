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

    //convert seed to a vec of bytes
    let mut nr = Vec::<u8>::new();
    let seed_nr = get_nr_from_str(seed);

    for x in seed_nr.to_string().chars() {
        nr.push(x.to_string().parse().unwrap());
    }

    let mut stars = Vec::<Star>::new();
    let mut planets = Vec::<Planet>::new();

    //helper closure to compact syntax
    //returns looping value at n index of vec
    let nth = |index| { 
        nr.iter().nth(index % nr.len()).unwrap().clone() as f64 
    };

    //cannot use for loop due to stricter control and jumping requirements
    let mut index = 0;
    while index < nr.len() {
        let digit = nth(index) as u8;

        //first digit is for star generation - TBC
        //begin star phase
        if index == 0 {
            let star_amount = match digit {
                0 | 1 | 2 | 3 => 1,
                4 | 5 | 6 => 2,
                7 | 8 => 3,
                9 => 4,
                _ => unreachable!() //heh
            };

            //for debugging, you can set this to a custom amount.
            //star_amount = 1; 
            
            //now actually create the star structs.
            //todo: fix star parameter ranges!
            for i in 0..star_amount {

                //index + 0: the 'number of stars'
                //index + 1: starting point for the star data
                //* 3: change 3 to the amount of parameters to pull out. Currently star()
                //takes 3 params (kilo_mass, age, metallicity) but if this changes this needs to be updated.
                //sidenote: maybe this is possible with macros?
                let base = index + 1 + i * 3;

                //NEED TO ADJUST THESE!! my stars keep exploding!
                stars.push(star(
                    get_rand_mass(0.01, 1.0, nth(index)) * 1e25,
                    get_rand_mass(0.01, 1.0,nth(base + 1)) * 1e4,
                    nth(base + 2) / 10.0,
                ));
            }
            //star phase complete, 'consume' all used digits and proceed to planet phase
            index += 1 + 3 * star_amount;
        }
        else {
            //begin planet phase
            match digit {
                6 => {}, //magic moon number, TODO add moons, TBC etc
                _ => planets.push(planet(   //TODO tweak values and factors
                    get_rand_mass(0.01, 1.0, nth(index + 1))  * 1e10,
                    nth(index + 2) * 200.0 + 3000.0,
                    rand::rng().random(),
                    rand::rng().random(),
                )),
            }
            //consume digits (index, index + 1, index + 2): 3 (:3)
            index += 3;
        }
    }
    dbg!(seed_nr);

    //idk change this to true to see a printout of the entire system
    if false {
        dbg!((stars, planets))
    }
    else {
        (stars, planets)
    }

}

fn get_nr_from_str(seed: &str) -> u128 {
    //this converts the string to base64 and then assigns each character the number of its index
    //if the seed is only numbers, it skips the process and just gets sent back as-is. for debugging
    //strictly speaking this means certain numbers (like 7-9) are less likely to appear.

    //truncate to not exceed max size of resulting u128
    let truncated = seed
        .char_indices()
        .nth(230)
        .map_or(seed, |(i, _)| &seed[..i]);

    //if seed is only numbers, skip and just send it back
    match truncated.parse::<u128>() {
        Ok(v) => return v,
        Err(_) => {}
    }

    //encode the string as base64
    let seed_b64 = BASE64_STANDARD.encode(truncated);
    //slightly custom alphabet. allows define anything else i want and change order
    let alphabet = "abcdefghijklmnopqrstuvwxyz.ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789+/=";

    //iterate and perform replacement
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

    //send back
    numeric_str.parse::<u128>().unwrap_or(0)
}

fn get_rand_mass(min: f64, max: f64, seed: f64) -> f64 {
    //prototype for pseudo randomness, dont use
    let x = f64::sin(seed) * 172410.0;
    return (x - x.floor() * (max - min + 1.1) + min).abs();
}