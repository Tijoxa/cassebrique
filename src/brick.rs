use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::cons::*;

#[derive(Component)]
pub struct Brick;
const BRICK_DIMENSION: Dimension = Dimension { x: 20., y: 20. };

pub fn setup_brick(mut commands: Commands) {
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
        Brick,
    ));
}