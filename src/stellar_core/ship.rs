use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Ship {
    pub position: Vec2,
    pub velocity: Vec2
}

impl Ship {
    pub fn new() -> Ship {
        Ship { position: Vec2::new(100.0, -200.0), velocity: Vec2::new(0.0, 0.0) }
    }

    pub fn to_string(&self) -> String {
        format!("{}", &self)
    }
}

impl std::fmt::Display for Ship {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({0},{1}) ({2},{3})", self.position.x, self.position.y, self.velocity.x, self.velocity.y)
    }   
}