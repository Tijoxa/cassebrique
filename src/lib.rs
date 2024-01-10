use anyhow::Result;
use bevy::{prelude::*, window::WindowResolution};
use bevy_prototype_lyon::prelude::*;

pub mod cons;
use cons::*;

pub mod plateau;
use plateau::*;

pub mod brick;
use brick::*;

pub mod ball;
use ball::*;

pub fn main_app() -> Result<()> {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Casse Brique".into(),
                resolution:
                    WindowResolution::new(WINDOW_DIMENSION.x, WINDOW_DIMENSION.y).with_scale_factor_override(1.0),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(ShapePlugin)
        .add_systems(Startup, (setup_system, setup_plateau, setup_brick, setup_ball))
        .add_systems(Update, (update_plateau_mouse, update_plateau_gamepad))
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
