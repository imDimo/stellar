use bevy::prelude::*;
use crate::stellar_core;

pub struct CoordsUIPlugin;
impl Plugin for CoordsUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_systems(Update, update)
        ;
    }
}

//marker struct to identify the span
#[derive(Component)]
struct CoordsUIMarker;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {

    let font = TextFont {
        font: asset_server.load("fonts/vcr_osd_mono.ttf"),
        font_size: 30.0,
        ..default()
    };

    commands.spawn((
        Text::new("Pos: "),
        font.clone(),
        TextLayout::new_with_justify(JustifyText::Center),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(60.0),
            left: Val::Px(5.0),
            ..default()
        },
    ))
    .with_child((
        TextSpan::default(),
        font,
        CoordsUIMarker
    ))
    ;

}

fn update(
    mut query: Query<&mut TextSpan, With<CoordsUIMarker>>,
    ship_query: Query<&mut Transform, (With<stellar_core::ship::Ship>, Without<Camera2d>)>
) {
    for mut span in &mut query {
        for ship in &ship_query {
            **span = format!("X{0}, Y{1}", ship.translation.x as i32, ship.translation.y as i32);
        }
    }
}
