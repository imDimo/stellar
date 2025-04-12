use bevy::prelude::*;

mod stellar_core;
mod game;

fn main() {

    App::new()
        .add_plugins((
                DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Stellar"),
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        //resolution: Vec2::new(512., 512.).into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_linear()),

                game::GamePlugin,
            )
        )
        .run();

}

