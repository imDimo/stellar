use bevy::prelude::*;
use crate::stellar_core;

pub struct ShipPlugin;
impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_ship)
            .add_systems(Update, update_ship)
            .add_systems(Update, ship_controls);
    }
}

#[derive(Component, Debug)]
pub struct Ship {
    pub velocity: Vec2
}

#[derive(Component)]
pub struct ShipEngine {

}

fn setup_ship(mut commands: Commands, asset_server : Res<AssetServer>) {
    let ship_image: Handle<Image> = asset_server.load("ship.png");
    let engine_flame: Handle<Image> = asset_server.load("engine_flame.png");

    commands.spawn((
        Ship { velocity: Vec2 {x: 0.0, y: 0.1 }},
        Sprite { image: ship_image, custom_size: Some(Vec2::splat(8.)), ..default() },
        Transform::from_xyz(0.0, 0.0, 1.0)
    ))
    .with_child((
        ShipEngine {},
        Sprite { image: engine_flame, custom_size: Some(Vec2::splat(6.0)), ..default() },
        Transform::from_xyz(0.0, -12.0, 0.0).with_scale(Vec3 { x: 0.8, y: 2.0, z: 1.0 })
    ));
}

fn update_ship(
    mut ship_query: Query<(&mut stellar_core::ship::Ship, &mut Transform)>, 
    bodies_query: Query<(&stellar_core::celestial_body::CelestialBody, &Transform), Without<stellar_core::ship::Ship>>
) {
    let Ok((mut ship, mut transform)) = ship_query.get_single_mut() else {
        return;
    };

    let new_velocity = 
        stellar_core::navigation::calculate_acceleration(
            &transform.translation.xy(), &bodies_query.iter().collect()) + ship.velocity;

    let direction = new_velocity.normalize_or_zero();
    let angle = direction.y.atan2(direction.x) - 0.5 * std::f32::consts::PI;

    transform.rotation = Quat::from_rotation_z(angle);

    transform.translation.x += new_velocity.x;
    transform.translation.y += new_velocity.y;

    ship.velocity = new_velocity;
}

fn ship_controls(
    buttons: Res<ButtonInput<MouseButton>>,
    mut engine_query: Query<&mut Transform, With<ShipEngine>>,
    q_windows: Query<&Window, With<bevy::window::PrimaryWindow>>
) {

    let Ok(mut engine_transform) = engine_query.get_single_mut() else { return; };

    if buttons.pressed(MouseButton::Left) {
        let new_scale_y = lerp(engine_transform.scale.y, 2.0);
        let scale_delta = new_scale_y - engine_transform.scale.y;
        engine_transform.scale.y = new_scale_y;
    
        // Move up by half of the scale delta to keep the top fixed
        engine_transform.translation.y -= scale_delta * 3.0; // Half of 6.0 (sprite height)    
        let new_scale_y = lerp(engine_transform.scale.y, 2.0);
        let scale_delta = new_scale_y - engine_transform.scale.y;
        engine_transform.scale.y = new_scale_y;
    
        // Move up by half of the scale delta to keep the top fixed
        engine_transform.translation.y -= scale_delta * 3.0; // Half of 6.0 (sprite height)
    }
    else {
        let new_scale_y = lerp(engine_transform.scale.y, 0.0);
        let scale_delta = engine_transform.scale.y - new_scale_y;
        engine_transform.scale.y = new_scale_y;
    
        // Move down by half of the scale delta
        engine_transform.translation.y += scale_delta * 3.0;
    }
}

fn lerp(start: f32, end: f32) -> f32 {
    (end - start) / 4.0 + start
}