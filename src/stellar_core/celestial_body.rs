use bevy::prelude::*;

pub struct CelestialPlugin;
impl Plugin for CelestialPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_camera)
            .add_systems(Update, update_camera);
    }
}

#[derive(Clone, Component, Debug)]
pub struct CelestialBody {
    pub mass: f32,
    pub position: Vec2
}
