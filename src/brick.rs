use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::cons::*;

#[derive(Component)]
pub struct Brick {
    pub hp: i32,
}
pub const BRICK_DIMENSION: Vec2 = Vec2 { x: 60., y: 20. };
pub const NUMBER_BRICKS: usize = 20;

pub fn get_brick_radius() -> f32 {
    BRICK_DIMENSION.x.min(BRICK_DIMENSION.y) / 3.
}

pub fn setup_brick(mut commands: Commands) {
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

    let base_x = (-GAME_DIMENSION.x + THICKNESS + BRICK_DIMENSION.x) / 2. + 10.;
    let base_y = (GAME_DIMENSION.y - THICKNESS - BRICK_DIMENSION.y) / 2. - 20.;

    for i in 0..NUMBER_BRICKS {
        let mut x_buffer = base_x + (i as f32) * (BRICK_DIMENSION.x + 15.);
        let mut y_buffer = base_y;

        while x_buffer > -base_x {
            x_buffer += 2. * base_x;
            y_buffer -= BRICK_DIMENSION.y + 25.;
        }

        commands
            .spawn((ShapeBundle {
                path: GeometryBuilder::build_as(&shape),
                ..default()
            },))
            .insert(Fill::color(Color::CYAN))
            .insert(Brick { hp: 2 })
            .insert(Transform::from_translation(Vec3::new(x_buffer, y_buffer, 0.)));
    }
}

pub fn despawn_brick(mut commands: Commands, brick_query: Query<(Entity, &Brick)>) {
    for (entity, brick) in brick_query.iter() {
        if brick.hp <= 0 {
            commands.entity(entity).despawn();
        }
    }
}

pub fn update_color_brick(mut brick_query: Query<(&Brick, &mut Fill)>) {
    for (brick, mut fill) in brick_query.iter_mut() {
        match brick.hp {
            1 => fill.color = Color::RED,
            2 => fill.color = Color::CYAN,
            _ => (),
        }
    }
}
