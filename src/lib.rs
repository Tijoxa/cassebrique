use anyhow::Result;
use bevy::{prelude::*, window::WindowResolution};
use bevy_prototype_lyon::prelude::*;

pub mod cons;
use cons::*;

pub mod raquette;
use raquette::*;

pub mod brick;
use brick::*;

pub mod ball;
use ball::*;

pub fn main_app() -> Result<()> {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Casse Brique".into(),
                resolution: WindowResolution::new(WINDOW_DIMENSION.x, WINDOW_DIMENSION.y).with_scale_factor_override(1.0),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(ShapePlugin)
        .add_systems(Startup, (setup_system, setup_raquette, setup_brick, setup_ball))
        .add_systems(
            Update,
            (
                update_raquette_mouse,
                update_raquette_gamepad,
                update_ball_keyboard,
                move_ball_on_raquette,
                move_ball_ingame,
                move_ball_brick,
                move_ball_raquette,
            ),
        )
        .run();
    Ok(())
}

fn setup_system(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    let game_shape = shapes::Rectangle {
        extents: Vec2 {
            x: GAME_DIMENSION.x,
            y: GAME_DIMENSION.y,
        },
        origin: RectangleOrigin::CustomCenter(Vec2 { x: 0., y: 0. }),
    };
    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&game_shape),
            ..default()
        },
        Stroke::new(Color::ANTIQUE_WHITE, THICKNESS),
    ));
}
