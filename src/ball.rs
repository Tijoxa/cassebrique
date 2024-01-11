use bevy::{math::vec3, prelude::*};
use bevy_prototype_lyon::prelude::*;
use rand::prelude::*;

use crate::{
    brick::{Brick, BRICK_DIMENSION},
    cons::*,
    raquette::Raquette,
};

#[derive(PartialEq, Eq, Debug)]
pub enum State {
    Raquette,
    Free,
}

#[derive(Component, Debug)]
pub struct Ball {
    pub state: State,
    pub v: Vec3,
}
const BALL_RADIUS: f32 = 5.;

#[derive(PartialEq, Eq, Debug)]
pub enum Region {
    Top,
    TopRight,
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
    Left,
    TopLeft,
    Unknown,
}

pub fn spawn_ball(mut commands: Commands, keys: Res<Input<KeyCode>>) {
    if !keys.just_pressed(KeyCode::Q) {
        return;
    }

    let shape = shapes::Circle {
        radius: BALL_RADIUS,
        center: Vec2::ZERO,
    };

    let translation_y = RAQUETTE_DIMENSION.y + BALL_RADIUS + 1.5 * THICKNESS - GAME_DIMENSION.y / 2. + 10.;

    let mut rng = thread_rng();
    let vx = rng.gen_range(-2.0..2.0);
    let vy = rng.gen_range(2.0..3.0);
    commands
        .spawn((ShapeBundle {
            path: GeometryBuilder::build_as(&shape),
            ..default()
        },))
        .insert(Transform::from_translation(Vec3::new(0., translation_y, 0.)))
        .insert(Fill::color(Color::ORANGE_RED))
        .insert(Ball {
            state: State::Raquette,
            v: vec3(vx, vy, 0.),
        });
}

pub fn update_ball_keyboard(mut query: Query<&mut Ball>, keys: Res<Input<KeyCode>>) {
    for mut ball in query.iter_mut() {
        if keys.just_pressed(KeyCode::Space) && ball.state == State::Raquette {
            ball.state = State::Free;
        }
    }
}

pub fn move_ball_on_raquette(
    mut ball_query: Query<(&Ball, &mut Transform), With<Ball>>,
    raquette_query: Query<(&Raquette, &Transform), Without<Ball>>,
) {
    for (ball, mut ball_t) in ball_query.iter_mut() {
        if ball.state == State::Raquette {
            let (_r, raquette_transform) = raquette_query.single();
            ball_t.translation.x = raquette_transform.translation.x;
        }
    }
}

pub fn move_ball_ingame(mut query: Query<(&mut Ball, &mut Transform)>) {
    for (mut ball, mut ball_t) in query.iter_mut() {
        if ball.state == State::Free {
            ball_t.translation += ball.v;

            let left_border = -GAME_DIMENSION.x / 2. + THICKNESS;
            let right_border = GAME_DIMENSION.x / 2. - THICKNESS;
            let bottom_border = -GAME_DIMENSION.y / 2. + THICKNESS;
            let top_border = GAME_DIMENSION.y / 2. - THICKNESS;

            if ball_t.translation.x <= left_border || ball_t.translation.x >= right_border {
                ball.v.x *= -1.0;
            }
            if ball_t.translation.y <= bottom_border || ball_t.translation.y >= top_border {
                ball.v.y *= -1.0;
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn move_ball(
    ball_t: &Mut<'_, Transform>,
    x2: f32,
    x3: f32,
    y3: f32,
    y4: f32,
    x4: f32,
    y2: f32,
    y1: f32,
    x1: f32,
    ball: &mut Mut<'_, Ball>,
) -> Option<()> {
    let region = match (ball_t.translation.x, ball_t.translation.y) {
        _ if (x2..x3).contains(&ball_t.translation.x) && (y3..y4).contains(&ball_t.translation.y) => Region::Top,
        _ if (x3..x4).contains(&ball_t.translation.x) && (y3..y4).contains(&ball_t.translation.y) => Region::TopRight,
        _ if (x3..x4).contains(&ball_t.translation.x) && (y2..y3).contains(&ball_t.translation.y) => Region::Right,
        _ if (x3..x4).contains(&ball_t.translation.x) && (y1..y2).contains(&ball_t.translation.y) => Region::BottomRight,
        _ if (x2..x3).contains(&ball_t.translation.x) && (y1..y2).contains(&ball_t.translation.y) => Region::Bottom,
        _ if (x1..x2).contains(&ball_t.translation.x) && (y1..y2).contains(&ball_t.translation.y) => Region::BottomLeft,
        _ if (x1..x2).contains(&ball_t.translation.x) && (y2..y3).contains(&ball_t.translation.y) => Region::Left,
        _ if (x1..x2).contains(&ball_t.translation.x) && (y3..y4).contains(&ball_t.translation.y) => Region::TopLeft,
        _ if (x2..x3).contains(&ball_t.translation.x) && (y2..y3).contains(&ball_t.translation.y) => {
            let m1 = (y3 - y2) / (x3 - x2);
            let m2 = -m1;

            let mb1 = (ball_t.translation.y - y2) / (ball_t.translation.x - x2);
            let mb2 = (ball_t.translation.y - y3) / (ball_t.translation.x - x2);

            match (mb1 < m1, mb2 < m2) {
                (true, true) => Region::Bottom,
                (true, false) => Region::Right,
                (false, false) => Region::Top,
                (false, true) => Region::Left,
            }
        }
        _ => Region::Unknown,
    };

    match region {
        Region::Right => {
            ball.v.x = ball.v.x.abs();
        }
        Region::Left => {
            ball.v.x = -ball.v.x.abs();
        }
        Region::Top => {
            ball.v.y = ball.v.y.abs();
        }
        Region::Bottom => {
            ball.v.y = -ball.v.y.abs();
        }
        Region::TopRight | Region::BottomRight | Region::BottomLeft | Region::TopLeft => {
            let corner = match region {
                Region::TopRight => vec3(x3, y3, 0.),
                Region::BottomRight => vec3(x3, y2, 0.),
                Region::BottomLeft => vec3(x2, y2, 0.),
                Region::TopLeft => vec3(x2, y3, 0.),
                _ => {
                    unreachable!();
                }
            };
            let cb = (ball_t.translation - corner).abs();
            if cb.length() >= BALL_RADIUS {
                return None;
            }

            let normale = cb.normalize();
            let dot_normale = ball.v.dot(normale);
            let tangente = Vec3::new(-normale.y, normale.x, 0.0).normalize();
            let dot_tangente = ball.v.dot(tangente);

            ball.v = dot_normale * normale - dot_tangente * tangente;
        }
        Region::Unknown => return None,
    }
    Some(())
}

pub fn move_ball_brick(
    mut ball_query: Query<(&mut Ball, &mut Transform), With<Ball>>,
    mut brick_query: Query<(&mut Brick, &Transform), Without<Ball>>,
) {
    for (mut ball, ball_t) in ball_query.iter_mut() {
        if ball.state == State::Free {
            for (mut brick, brick_t) in brick_query.iter_mut() {
                let x2 = brick_t.translation.x - BRICK_DIMENSION.x / 2.;
                let x3 = brick_t.translation.x + BRICK_DIMENSION.x / 2.;
                let y2 = brick_t.translation.y - BRICK_DIMENSION.y / 2.;
                let y3 = brick_t.translation.y + BRICK_DIMENSION.y / 2.;

                let x1 = x2 - BALL_RADIUS;
                let x4 = x3 + BALL_RADIUS;
                let y1 = y2 - BALL_RADIUS;
                let y4 = y3 + BALL_RADIUS;

                if move_ball(&ball_t, x2, x3, y3, y4, x4, y2, y1, x1, &mut ball).is_some() {
                    brick.hp -= 1;
                    return;
                }
            }
        }
    }
}

pub fn move_ball_raquette(
    mut ball_query: Query<(&mut Ball, &mut Transform), With<Ball>>,
    raquette_query: Query<(&Raquette, &Transform), Without<Ball>>,
) {
    for (mut ball, ball_t) in ball_query.iter_mut() {
        if ball.state == State::Free {
            for (_raquette, raquette_t) in raquette_query.iter() {
                let x2 = raquette_t.translation.x - RAQUETTE_DIMENSION.x / 2.;
                let x3 = raquette_t.translation.x + RAQUETTE_DIMENSION.x / 2.;
                let y2 = raquette_t.translation.y - RAQUETTE_DIMENSION.y / 2.;
                let y3 = raquette_t.translation.y + RAQUETTE_DIMENSION.y / 2.;

                let x1 = x2 - BALL_RADIUS;
                let x4 = x3 + BALL_RADIUS;
                let y1 = y2 - BALL_RADIUS;
                let y4 = y3 + BALL_RADIUS;

                move_ball(&ball_t, x2, x3, y3, y4, x4, y2, y1, x1, &mut ball);
            }
        }
    }
}
