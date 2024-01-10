use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::cons::*;

#[derive(PartialEq, Eq, Debug)]
pub enum State {
    Plateau,
    Free,
}

#[derive(Component, PartialEq, Eq, Debug)]
pub struct Ball {
    pub state: State,
}
const BALL_RADIUS: f32 = 5.;

pub fn setup_ball(mut commands: Commands) {
    let shape = shapes::Circle {
        radius: BALL_RADIUS,
        center: Vec2 {
            x: 0.,
            y: PLATEAU_DIMENSION.y + BALL_RADIUS + 1.5 * THICKNESS - GAME_DIMENSION.y / 2.,
        },
    };

    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&shape),
            ..default()
        },
        Fill::color(Color::ORANGE_RED),
        Ball { state: State::Plateau },
    ));
}

pub fn update_ball_keyboard(keys: Res<Input<KeyCode>>, mut query: Query<&mut Ball>) {
    println!("salut");
    if keys.just_pressed(KeyCode::Space) {
        for mut ball in query.iter_mut() {
            if ball.state == State::Plateau {
                ball.state = State::Free;
                // TODO
            }
        }
    }
}
