use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::cons::*;

#[derive(Component)]
pub struct Brick {
    pub hp: i32,
}
pub const BRICK_DIMENSION: Dimension = Dimension { x: 60., y: 20. };

pub fn get_brick_radius() -> f32 {
    BRICK_DIMENSION.x.min(BRICK_DIMENSION.y) / 3.
}

pub fn setup_brick(mut commands: Commands) {
    // spawn bricks
    let shape = shapes::RoundedPolygon {
        points: vec![
            Vec2::new(-BRICK_DIMENSION.x / 2., -BRICK_DIMENSION.y / 2.),
            Vec2::new(BRICK_DIMENSION.x / 2., -BRICK_DIMENSION.y / 2.),
            Vec2::new(BRICK_DIMENSION.x / 2., BRICK_DIMENSION.y / 2.),
            Vec2::new(-BRICK_DIMENSION.x / 2., BRICK_DIMENSION.y / 2.),
        ],
        radius: get_brick_radius(),
        ..shapes::RoundedPolygon::default()
    };

    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&shape),
            ..default()
        },
        Fill::color(Color::CYAN),
        Brick { hp: 2 },
    ));
}

pub fn despawn_brick(mut commands: Commands, brick_query: Query<(Entity, &Brick)>) {
    for (entity, brick) in brick_query.iter() {
        if brick.hp <= 0 {
            commands.entity(entity).despawn();
        }
    }
}
