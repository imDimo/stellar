use bevy::{input::mouse::MouseWheel, prelude::*};
use crate::stellar_core;

pub fn list_ship_position(query: Single<&stellar_core::ship::Ship>) {
    println!("{}", query.to_string())
}

pub fn update_ship(mut ship: Single<&mut stellar_core::ship::Ship>, bodies: Query<&mut stellar_core::celestial_body::CelestialBody>) {
    let velocity = 
    stellar_core::navigation::calculate_acceleration(&ship.position, &bodies.iter().collect()) / 10000. + ship.velocity;
    ship.position += velocity;
    ship.velocity = velocity;
}

pub fn camera_movement(
    mut camera_query: Query<&mut Transform, With<Camera2d>>, 
    input: Res<ButtonInput<KeyCode>>, 
    mut evr_scroll: EventReader<MouseWheel>) {
    let mut transform = camera_query.single_mut();
    let mut direction = Vec3::ZERO;
    let mut zoom: f32 = transform.scale.y;

    if input.pressed(KeyCode::ArrowUp) { direction.y += 1.0; }
    if input.pressed(KeyCode::ArrowDown) { direction.y -= 1.0; }
    if input.pressed(KeyCode::ArrowLeft) { direction.x -= 1.0; }
    if input.pressed(KeyCode::ArrowRight) { direction.x += 1.0; }

    for ev in evr_scroll.read() {
        zoom *= 1.0 - ev.y / 10.0;
    }

    transform.translation += direction * zoom * 5.0;
    transform.scale = Vec3 { x: zoom, y: zoom, z: zoom };

}

pub fn exit( input: Res<ButtonInput<KeyCode>>, mut app_exit_events: ResMut<Events<bevy::app::AppExit>>) {
    if input.pressed(KeyCode::Escape) { app_exit_events.send(bevy::app::AppExit::Success); }
}