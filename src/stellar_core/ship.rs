use bevy::prelude::*;
use crate::stellar_core;

mod thruster;
use crate::stellar_core::ship::thruster::EngineFlame as EngineFlame;
use crate::stellar_core::navigation::lerp as lerp;

pub struct ShipPlugin;
impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_ship)
            .add_systems(Update, update_ship)
            .add_systems(Update, stellar_core::ship::thruster::update_thrusters)
            .add_systems(Update, ship_controls);
    }
}

#[derive(Component, Debug)]
pub struct Ship {
    pub velocity: Vec2,
    pub angular: f32,
}

fn setup_ship(mut commands: Commands, asset_server : Res<AssetServer>) {
    //load textures
    let ship_image: Handle<Image> = asset_server.load("ship.png");
    let engine_flame: Handle<Image> = asset_server.load("engine_flame.png");

    //assemble the ship
    commands.spawn((
        Ship { velocity: Vec2 {x: 0.0, y: 0.1 }, angular: 0.01 },
        Sprite { image: ship_image, custom_size: Some(Vec2::splat(8.)), ..default() },
        Transform::from_xyz(0.0, 0.0, 1.0)
    ))
    .with_child( //the engine flame is a child because it allows custom placement of the plume
        thruster::get_thruster_bundle(&engine_flame, 0,
            Transform::from_xyz(0.0, -6.0, 0.0)
            .with_rotation(Quat::from_rotation_z(0.0))
            .with_scale(Vec3 { x: 0.8, y: 4.0, z: 1.0 })
    ))
    .with_child(
        thruster::get_thruster_bundle(&engine_flame, 1,
            Transform::from_xyz(3.0, 2.0, 0.0)
            .with_rotation(Quat::from_rotation_z(2.5))
            .with_scale(Vec3 { x: 0.4, y: 0.8, z: 1.0 })
    ))
    .with_child(
        thruster::get_thruster_bundle(&engine_flame, 2,
            Transform::from_xyz(-3.0, 2.0, 0.0)
            .with_rotation(Quat::from_rotation_z(-2.5))
            .with_scale(Vec3 { x: 0.4, y: 0.8, z: 1.0 })
    ))
    ;
}

//process gravity for the ship
fn update_ship(
    mut ship_query: Query<(&mut stellar_core::ship::Ship, &mut Transform)>, 
    bodies_query: Query<(&stellar_core::celestial_body::CelestialBody, &Transform), Without<stellar_core::ship::Ship>>
) {
    //unpack and error handle the tuple
    //this pattern isnt exactly necessary but its okay
    let Ok((mut ship, mut transform)) = ship_query.get_single_mut() else {
        return;
    };

    //calculate the velocity gravitational attraction produces at this point in space
    let new_velocity = 
        stellar_core::navigation::calculate_acceleration(&transform.translation.xy(), &bodies_query.iter().collect())
             + ship.velocity; //and add it to the current velocity

    //normalize this value for angle calculation
    let direction = new_velocity.normalize_or_zero();
    let target_angle = direction.y.atan2(direction.x); //rotate it by 90 degrees
    let current_angle = transform.rotation.z + ship.angular;

    //ship.angular *= (target_angle - current_angle) / 1.0;
    transform.rotation = Quat::from_rotation_z(current_angle + ship.angular - 0.5 * std::f32::consts::PI);

    transform.translation.x += new_velocity.x;
    transform.translation.y += new_velocity.y;

    ship.velocity = new_velocity;
}

fn ship_controls(
    buttons: Res<ButtonInput<MouseButton>>,
    mut ship_query: Query<(&mut Ship, &mut Transform)>,
    mut engines: Query<&mut EngineFlame, Without<Ship>>,
    q_windows: Query<&Window, With<bevy::window::PrimaryWindow>>
) {
    let (mut ship, mut transform) = ship_query.single_mut();
    let speed = 0.01 / ship.velocity.max_element();

    if buttons.pressed(MouseButton::Left) {
        engines.iter_mut().for_each(|mut e| {
            if e.id == 0 {
                e.active = true;
            }
        });
        ship.velocity *= 1.0 + speed;
    }
    else {
        engines.iter_mut().for_each(|mut e| {
            if e.id == 0 {
                e.active = false;
            }
        });
    }

    if buttons.pressed(MouseButton::Right) {
        engines.iter_mut().for_each(|mut e| {
            if e.id == 2 {
                e.active = true;
            }
        });
        //ship.velocity *= 1.0 - speed;
        ship.angular *= 1.0 + speed;
    }
    else {
        engines.iter_mut().for_each(|mut e| {
            if e.id == 1 || e.id == 2 {
                e.active = false;
            }
        });
    }
}
