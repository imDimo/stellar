use bevy::prelude::*;
use crate::stellar_core;

mod thruster;

use core::f32::consts::PI as PI;
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

#[macro_export]
macro_rules! debug {
    ( $tuple:expr ) => {
        println!("{:?}", $tuple);
    };
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
            Transform::from_xyz(-6.0, 0.0, 0.0)
            .with_rotation(Quat::from_rotation_z(1.5 * PI))
            .with_scale(Vec3 { x: 0.8, y: 4.0, z: 1.0 })
    ))
    .with_child(
        thruster::get_thruster_bundle(&engine_flame, 1,
            Transform::from_xyz(2.0, 3.0, 0.0)
            .with_rotation(Quat::from_rotation_z(0.5 * PI + 0.3))
            .with_scale(Vec3 { x: 0.4, y: 0.8, z: 1.0 })
    ))
    .with_child(
        thruster::get_thruster_bundle(&engine_flame, 2,
            Transform::from_xyz(2.0, -3.0, 0.0)
            .with_rotation(Quat::from_rotation_z(0.5 * PI - 0.3))
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

    transform.rotation *= Quat::from_rotation_z(ship.angular);

    transform.translation.x += new_velocity.x;
    transform.translation.y += new_velocity.y;

    ship.velocity = new_velocity;
}

fn ship_controls(
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut ship_query: Query<(&mut Ship, &mut Transform)>,
    mut engines: Query<&mut EngineFlame, Without<Ship>>,
    q_windows: Query<&Window, With<bevy::window::PrimaryWindow>>
) {
    let Ok((mut ship, transform)) = ship_query.get_single_mut() else { return };
    let Ok(window) = q_windows.get_single() else { return };

    if mouse_buttons.pressed(MouseButton::Right) {
        for mut e in engines.iter_mut() {
            if e.id > 0 {
                e.active = true;
            }
        }

        ship.angular *= 0.95;
        ship.velocity *= 0.95;
    }
    else {
        for mut e in engines.iter_mut() {
            if e.id > 0 {
                e.active = false;
            }
        }
    }

    if mouse_buttons.pressed(MouseButton::Left) {
        engines.iter_mut().find(|e| e.id == 0).map(|mut e| e.active = true);

        //unpack this safely incase user does something like click then drag mouse outside of the window.
        let Some(cursor_pos) = window.cursor_position() else { return };

        let world_pos = 
            transform.translation.xy() + cursor_pos - window.size() / 2.0;

        let velocity_modifier = Vec2 { 
            x: (world_pos.x - transform.translation.x), 
            y: -(world_pos.y - transform.translation.y) 
        } / 1.0e3;

        ship.velocity += velocity_modifier;

        if ship.velocity.length_squared() > 0.0 {
            let nrm = velocity_modifier.normalize();
            let target_angle = nrm.y.atan2(nrm.x); // x and y swapped for standard atan2
        
            // Extract the current angle from the transform's rotation
            let current_angle = transform.rotation.to_euler(EulerRot::XYZ).2;
        
            // Compute shortest angle difference
            let mut angle_diff = target_angle - current_angle;

            if angle_diff > PI {
                angle_diff -= 2.0 * PI;
            } 
            else if angle_diff < -PI {
                angle_diff += 2.0 * PI;
            }
        
            // Apply damped turning speed
            ship.angular = angle_diff * 0.1;
        }

    }
    else {
        engines.iter_mut().find(|e| e.id == 0).map(|mut e| e.active = false);
    }

}