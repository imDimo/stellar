use bevy::prelude::*;
mod core;

fn main() {

    core::navigation::test();

    // App::new()
    //     .add_plugins(DefaultPlugins) 
    //     .add_systems(Startup, (add_ship))
    //     .add_systems(Update, list_ship_name)
    //     .run();

    
}

fn add_ship(mut commands: Commands) {
    println!("Adding ship");
    commands.spawn((core::ship::Ship, core::Name("USS Ligma".to_string())));
}

fn list_ship_name(query: Query<&core::Name, With<core::ship::Ship>>) {
    for name in &query {
        println!("Ship: {0}", name.0);
    }
}