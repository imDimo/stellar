use bevy::{input::mouse::MouseWheel, prelude::*};

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_camera)
            .add_systems(Update, update_camera);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Camera { ..default() }));
}

fn update_camera(
    mut camera_query: Query<&mut Transform, With<Camera2d>>, 
    input: Res<ButtonInput<KeyCode>>, 
    mut evr_scroll: EventReader<MouseWheel>
) {
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