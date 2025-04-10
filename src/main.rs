use bevy::prelude::*;

mod stellar_core;
mod init;
mod update;
mod draw;

fn main() {

     App::new()
         .add_plugins(DefaultPlugins) 
         .add_systems(Startup, (init::add_ship, init::create_solar_system, init::create_camera))
         .add_systems(Update, (update::update_ship, update::camera_movement))
         .add_systems(Update, (draw::draw_ship, draw::draw_solar_system))
         .run();
}

