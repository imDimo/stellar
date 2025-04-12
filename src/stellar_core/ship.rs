use bevy::prelude::*;
use crate::stellar_core;

pub struct ShipPlugin;
impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_ship)
            .add_systems(Update, update_ship);
    }
}

#[derive(Component, Debug)]
pub struct Ship {
    pub velocity: Vec2
}

fn setup_ship(mut commands: Commands, asset_server : Res<AssetServer>) {
    let ship_image = asset_server.load("ship.png");
    commands.spawn((
        Sprite { image: ship_image, ..Default::default() },
        Ship { velocity: Vec2 {x: 0.0, y: 0.5 }}
    ));
}

fn update_ship(
    mut ship_query: Query<(&mut stellar_core::ship::Ship, &mut Transform)>, 
    bodies: Query<&mut stellar_core::celestial_body::CelestialBody>
) {
    let (mut ship, mut transform) = ship_query.get_single_mut().unwrap();
    let new_velocity = 
        stellar_core::navigation::calculate_acceleration(
            &transform.translation.xy(), &bodies.iter().collect()) + ship.velocity;

    transform.translation.x += new_velocity.x;
    transform.translation.y += new_velocity.y;

    ship.velocity = new_velocity;
}