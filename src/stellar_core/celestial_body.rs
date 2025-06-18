pub mod satellite;
pub mod orbit;
pub mod planet;
pub mod star;

use bevy::prelude::*;
use planet::Planet;

use crate::{procedural_generation::gen_system::gen_system, stellar_core::celestial_body::star::Star};

pub struct SolarSystemPlugin;
impl Plugin for SolarSystemPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_solar_system)
            .add_systems(Update, update_solar_system);
    }
}

fn setup_solar_system(
    mut commands: Commands, 
    mut images: ResMut<Assets<Image>>,
) {

    //todo: there is a bug where something crashes this. try different seeds
    let (stars, planets) = gen_system("meow kitty");

    for (i, star) in stars.into_iter().enumerate() {
        commands.spawn(
            Star::get_bundle(star, i as f32 * 1000.0, 10.0, &mut images)
        );
    }

    for (i, planet) in planets.into_iter().enumerate() {
        commands.spawn(
            Planet::get_bundle(planet, i as f32 * 1000.0, 1000.0, &mut images)
        );
    }


}

fn update_solar_system(_bodies: Query<&mut Planet>, mut _gizmos: Gizmos) {

}

