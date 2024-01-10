use bevy::{input::mouse::MouseMotion, prelude::*};
use bevy_prototype_lyon::prelude::*;

use crate::cons::*;

#[derive(Component)]
pub struct Plateau;

pub fn setup_plateau(mut commands: Commands) {
    let shape = shapes::Rectangle {
        extents: Vec2 {
            x: PLATEAU_DIMENSION.x,
            y: PLATEAU_DIMENSION.y,
        },
        origin: RectangleOrigin::CustomCenter(Vec2 {
            x: 0.,
            y: (PLATEAU_DIMENSION.y - GAME_DIMENSION.y) / 2. + THICKNESS,
        }),
    };
    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&shape),
            ..default()
        },
        Fill::color(Color::PINK),
        Stroke::new(Color::PURPLE, THICKNESS),
        Plateau,
    ));
}

pub fn update_plateau_mouse(
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut motion_evr: EventReader<MouseMotion>,
    mut query: Query<&mut Transform, With<Plateau>>,
) {
    for ev in motion_evr.read() {
        if ev.delta.x.abs() > 0.1 {
            let window = windows.single();
            let (camera, camera_transform) = camera_q.single();

            if let Some(world_position) = window
                .cursor_position()
                .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
            {
                for mut transform in query.iter_mut() {
                    transform.translation.x = world_position.x.clamp(
                        (-GAME_DIMENSION.x + PLATEAU_DIMENSION.x) / 2. + THICKNESS,
                        (GAME_DIMENSION.x - PLATEAU_DIMENSION.x) / 2. - THICKNESS,
                    );
                }
            }
        }
    }
}

pub fn update_plateau_gamepad(
    gamepads: Res<Gamepads>,
    axes: Res<Axis<GamepadAxis>>,
    mut query: Query<&mut Transform, With<Plateau>>,
) {
    for gamepad in gamepads.iter() {
        let left_stick_x = axes
            .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX))
            .unwrap();

        for mut transform in query.iter_mut() {
            transform.translation.x = (transform.translation.x + 10. * left_stick_x.tan()).clamp(
                (-GAME_DIMENSION.x + PLATEAU_DIMENSION.x) / 2. + THICKNESS,
                (GAME_DIMENSION.x - PLATEAU_DIMENSION.x) / 2. - THICKNESS,
            );
        }
    }
}
