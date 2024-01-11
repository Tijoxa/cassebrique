use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::cons::*;

#[derive(Component)]
pub struct Brick {
    pub hp: i32,
}
pub const BRICK_DIMENSION: Dimension = Dimension { x: 20., y: 20. };

pub fn setup_brick(mut commands: Commands) {
    // spawn bricks
    // change shape brick into rounded rectangle
    let shape = shapes::Rectangle {
        extents: Vec2 {
            x: BRICK_DIMENSION.x,
            y: BRICK_DIMENSION.y,
        },
        origin: RectangleOrigin::CustomCenter(Vec2 { x: 0., y: 0. }),
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
