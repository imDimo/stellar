use std::any::Any;

use bevy::prelude::*;
use crate::stellar_core;

#[derive(Component)]
pub struct ShipPath {
    pips: Vec<(Sprite, Transform)>
}

impl ShipPath {
    pub fn new(amount: i32, image: &Handle<Image>) -> Self {
        let mut pips = Vec::<(Sprite, Transform)>::new();

        for _ in 0..amount {
            pips.push(( Sprite { image: image.clone(), ..default() }, Transform { ..default() }));
        }

        ShipPath { pips }
    }

    //unfinished and doesnt work
    pub fn update(
        mut query: Query<&Self>, 
        bodies_query: Query<(&stellar_core::celestial_body::CelestialBody, &Transform), Without<stellar_core::ship::Ship>>) 
    {
        let Ok(mut path) = query.get_single_mut() else { return };

        let mut last_pos = Vec2::new(0.0, 0.0);

        for (_, mut tr) in &path.pips {
            
        }
    }
}