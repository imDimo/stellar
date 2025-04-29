use bevy::prelude::*;

mod stellar_core;
mod procedural_generation;
mod game;

fn main() {
    
    //procedural_generation::planet::test_planet();
    //test_grav();
    //return;

    let mass: f64 = 382718848909990000.0;

    let radius: f64 = 35000.0;//f64::powf((3.0 * mass) / (4.0 * core::f64::consts::PI * density), 1.0 / 3.0);
    let surface_gravity = (stellar_core::navigation::G as f64 * mass) / radius.powf(2.0) / 9.7803267715;

    println!("{}", surface_gravity);
    
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
                .set(ImagePlugin::default_nearest()),

                game::GamePlugin,
            ))
        .add_systems(Startup, set_window_icon)
        .run();

}

fn set_window_icon(windows: NonSend<bevy::winit::WinitWindows>) {
    // here we use the `image` crate to load our icon data from a png file
    // this is not a very bevy-native solution, but it will do
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open("assets/icon.png")
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    let icon = winit::window::Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();

    // do it for all windows
    for window in windows.windows.values() {
        window.set_window_icon(Some(icon.clone()));
    }
}