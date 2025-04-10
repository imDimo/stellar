use bevy::prelude::*;

#[derive(Clone, Component, Debug)]
pub struct CelestialBody {
    pub mass: f32,
    pub position: Vec2
}

impl std::fmt::Display for CelestialBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{0}, ({1}, {2})", self.mass, self.position.x, self.position.y)
    }   
}