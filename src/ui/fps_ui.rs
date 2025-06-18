use bevy::{
    prelude::*,
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin}
};

//plugin to wrap bevy functionality
pub struct FPSUIPlugin;
impl Plugin for FPSUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_systems(Update, update)
        ;
    }
}

//marker struct to identify the span
#[derive(Component)]
struct FPSMarker;

//set up the FPS text
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {

    let font = TextFont {
        font: asset_server.load("fonts/vcr_osd_mono.ttf"),
        font_size: 30.0,
        ..default()
    };

    commands.spawn((
        Text::new("fps: "),
        font.clone(),
        TextLayout::new_with_justify(JustifyText::Center),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            left: Val::Px(5.0),
            ..default()
        },
    ))
    .with_child((
        TextSpan::default(),
        font,
        FPSMarker
    ))
    ;

}

//update the FPS value with the frames from Diagnostics
fn update(mut query: Query<&mut TextSpan, With<FPSMarker>>, diagnostics: Res<DiagnosticsStore>,) {
    for mut span in &mut query {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                // Update the value of the second section
                **span = format!("{value:.2}");
            }
        }
    }
}
