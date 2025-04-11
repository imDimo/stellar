use bevy::prelude::*;
use bevy::color::palettes::basic::WHITE;
use crate::stellar_core;

pub fn draw_ship(ship: Single<&mut stellar_core::ship::Ship>, mut gizmos: Gizmos) {
    gizmos.arrow_2d(
        ship.position,
         ship.position + ship.velocity * 25.,
          WHITE);

}

pub fn draw_solar_system(bodies: Query<&mut stellar_core::celestial_body::CelestialBody>, mut gizmos: Gizmos) {

    for body in bodies.iter() {
        gizmos.circle_2d(body.position, (body.mass / 1.0e15) as f32, WHITE);
    }
}

pub fn draw_cursor(camera_query: Single<(&Camera, &GlobalTransform)>, windows: Query<&Window>, mut gizmos: Gizmos) {
    let (camera, camera_transform) = *camera_query;

    let Ok(window) = windows.get_single() else {
        return;
    };

    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    // Calculate a world position based on the cursor's position.
    let Ok(point) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return;
    };

    gizmos.arrow_2d(point, Vec2 { x: point.x - 20., y: point.y + 15. }, WHITE);
}