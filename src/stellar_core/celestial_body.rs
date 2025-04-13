use bevy::prelude::*;
use bevy::color::palettes::basic as COLOR;

pub struct SolarPlugin;
impl Plugin for SolarPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_solar_system)
            .add_systems(Update, update_solar_system);
    }
}

fn setup_solar_system(mut commands: Commands) {
    commands.spawn(CelestialBody {mass: 5.9e10, position: Vec2 { x: 10.0, y: 0.0 }});
    commands.spawn(CelestialBody {mass: 2e11, position: Vec2 { x: -110.0, y: -100.0 }});
}

fn update_solar_system(bodies: Query<&mut CelestialBody>, mut gizmos: Gizmos) {
    for body in bodies.iter() {
        gizmos.circle_2d(body.position, (body.mass / 1.0e10) as f32, COLOR::WHITE);
    }
}

#[derive(Clone, Component, Debug)]
pub struct CelestialBody {
    pub mass: f32,
    pub position: Vec2
}
