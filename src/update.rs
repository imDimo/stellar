use bevy::prelude::*;
use crate::stellar_core;

pub fn list_ship_position(query: Single<&stellar_core::ship::Ship>) {
    println!("{}", query.to_string())
}

pub fn update_ship(mut ship: Single<&mut stellar_core::ship::Ship>, bodies: Query<&mut stellar_core::celestial_body::CelestialBody>) {
    let velocity = stellar_core::navigation::calculate_acceleration(&ship.position, &bodies.iter().collect());
    ship.velocity += velocity;
    ship.position += velocity;
}

pub fn camera_movement(mut camera_query: Query<&mut Transform, With<Camera2d>>, input: Res<ButtonInput<KeyCode>>) {
    let mut transform = camera_query.single_mut();
    let mut direction = Vec3::ZERO;

    if input.pressed(KeyCode::ArrowUp) { direction.y += 1.0; }
    if input.pressed(KeyCode::ArrowDown) { direction.y -= 1.0; }
    if input.pressed(KeyCode::ArrowLeft) { direction.x -= 1.0; }
    if input.pressed(KeyCode::ArrowRight) { direction.x += 1.0; }

    transform.translation += direction * 5.;

}

pub fn exit( input: Res<ButtonInput<KeyCode>>, mut app_exit_events: ResMut<Events<bevy::app::AppExit>>) {
    if input.pressed(KeyCode::Escape) { app_exit_events.send(bevy::app::AppExit::Success); }
}