use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::stellar_core;

#[derive(Component)]
pub struct ShipPath;

impl Plugin for ShipPath {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, ShipPath::new)
            .add_systems(Update, ShipPath::update)
        ;
    }
}

impl ShipPath {
    pub fn new(mut commands: Commands) {
        let path_points = vec![Vec2::ZERO];

        ShipPath::spawn_path(&mut commands, &path_points);
    }

    fn spawn_path(commands: &mut Commands, points: &[Vec2]) {
        let segment_count = points.len().saturating_sub(1);
    
        for (i, window) in points.windows(2).enumerate() {
            let start = window[0];
            let end = window[1];
    
            let mut path_builder = PathBuilder::new();
            path_builder.move_to(start);
            path_builder.line_to(end);
            let path = path_builder.build();
    
            let t = i as f32 / segment_count.max(1) as f32; // normalized [0,1]
            let alpha = 1.0 - t.powf(1.5); // nonlinear fade (adjust curve as needed)
    
            commands.spawn((
                ShapeBundle {
                    path,
                    ..default()
                },
                Stroke::new(
                    Color::linear_rgba(0.0, 0.88, 1.0, alpha), // cyan fade
                    5.0,
                ),
                ShipPath,
            ));
        }
    }

    fn update(
        mut commands: Commands,
        query: Query<Entity, (With<ShipPath>, Without<stellar_core::ship::Ship>)>,
        ship: Query<&stellar_core::ship::Ship>
    ) {
        // Despawn old path entity
        for entity in &query {
            commands.entity(entity).despawn_recursive();
        }
    
        // Spawn the new path
        ShipPath::spawn_path(&mut commands, &ship.single().future_path);
    }
}