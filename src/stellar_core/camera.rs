use bevy::{input::mouse::*, prelude::*};
use crate::stellar_core;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        //build plugin & add systems
        app
            .add_systems(Startup, setup_camera)
            .add_systems(Update, update_camera);
    }
}

//todo: overhaul with 3D cameras for spicy planet renders
fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Camera { ..default() }));
}

fn update_camera(
    mut camera_query: Query<&mut Transform, With<Camera2d>>, 
    mut evr_scroll: EventReader<MouseWheel>,
    mut ship_query: Query<&mut Transform, (With<stellar_core::ship::Ship>, Without<Camera2d>)>
) {
    let mut transform = camera_query.single_mut();
    let mut zoom: f32 = transform.scale.y;

    //iterate through scroll events
    for ev in evr_scroll.read() {
        zoom *= 1.0 - ev.y / 10.0;
    }

    //set camera position to ship position.
    transform.translation = ship_query.single().translation;
    //apply scale
    transform.scale = Vec3 { x: zoom, y: zoom, z: zoom };

    //set ship scale to something relative so that it scales with zoom
    ship_query.single_mut().scale = (transform.scale + 1.0) * 3.0;
}

//freecam function
#[allow(dead_code)] // <= if not currently used, dont generate clippy warnings
fn update_free_camera(
    mut camera_query: Query<&mut Transform, With<Camera2d>>, 
    input: Res<ButtonInput<KeyCode>>, 
    mut evr_scroll: EventReader<MouseWheel>,
    mut evr_motion: EventReader<MouseMotion>,
    buttons: Res<ButtonInput<MouseButton>>
) {
    let mut transform = camera_query.single_mut();
    let mut direction = Vec3::ZERO;
    let mut zoom: f32 = transform.scale.y;

    if input.pressed(KeyCode::ArrowUp) { direction.y += 1.0; }
    if input.pressed(KeyCode::ArrowDown) { direction.y -= 1.0; }
    if input.pressed(KeyCode::ArrowLeft) { direction.x -= 1.0; }
    if input.pressed(KeyCode::ArrowRight) { direction.x += 1.0; }

    //scroll wheel zoom
    for ev in evr_scroll.read() {
        zoom *= 1.0 - ev.y / 10.0;
    }

    //mouse drag to move camera. cheap & easy solution - not always pixel-perfect
    for ev in evr_motion.read() {
        if buttons.pressed(MouseButton::Left) {
            direction -= Vec3 {x: ev.delta.x / 8.0, y: -ev.delta.y / 8.0, z: direction.z };
        }
    }

    //apply translation and scale
    transform.translation += direction * zoom * 5.0;
    transform.scale = Vec3 { x: zoom, y: zoom, z: zoom };

}