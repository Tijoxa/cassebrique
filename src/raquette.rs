use bevy::{input::mouse::MouseMotion, prelude::*};
use bevy_prototype_lyon::prelude::*;

use crate::cons::*;

#[derive(Component)]
pub struct Raquette;

pub const RAQUETTE_DIMENSION: Dimension = Dimension { x: 50., y: 10. };

pub fn get_raquette_radius() -> f32 {
    RAQUETTE_DIMENSION.x.min(RAQUETTE_DIMENSION.y) / 3.
}

pub fn setup_raquette(mut commands: Commands) {
    let shape = shapes::RoundedPolygon {
        points: vec![
            Vec2::new(-RAQUETTE_DIMENSION.x / 2., -RAQUETTE_DIMENSION.y / 2.),
            Vec2::new(RAQUETTE_DIMENSION.x / 2., -RAQUETTE_DIMENSION.y / 2.),
            Vec2::new(RAQUETTE_DIMENSION.x / 2., RAQUETTE_DIMENSION.y / 2.),
            Vec2::new(-RAQUETTE_DIMENSION.x / 2., RAQUETTE_DIMENSION.y / 2.),
        ],
        radius: RAQUETTE_DIMENSION.x.min(RAQUETTE_DIMENSION.y) / 3.,
        ..shapes::RoundedPolygon::default()
    };

    let translation_y = (RAQUETTE_DIMENSION.y - GAME_DIMENSION.y) / 2. + THICKNESS + 10.;

    commands
        .spawn((ShapeBundle {
            path: GeometryBuilder::build_as(&shape),
            ..default()
        },))
        .insert(Transform::from_translation(Vec3::new(0., translation_y, 0.)))
        .insert(Fill::color(Color::PINK))
        .insert(Raquette);
}

pub fn update_raquette_mouse(
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut motion_evr: EventReader<MouseMotion>,
    mut query: Query<&mut Transform, With<Raquette>>,
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
                        (-GAME_DIMENSION.x + RAQUETTE_DIMENSION.x) / 2. + THICKNESS,
                        (GAME_DIMENSION.x - RAQUETTE_DIMENSION.x) / 2. - THICKNESS,
                    );
                }
            }
        }
    }
}

pub fn update_raquette_gamepad(gamepads: Res<Gamepads>, axes: Res<Axis<GamepadAxis>>, mut query: Query<&mut Transform, With<Raquette>>) {
    for gamepad in gamepads.iter() {
        let left_stick_x = axes.get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX)).unwrap();

        for mut transform in query.iter_mut() {
            transform.translation.x = (transform.translation.x + 10. * left_stick_x.tan()).clamp(
                (-GAME_DIMENSION.x + RAQUETTE_DIMENSION.x) / 2. + THICKNESS,
                (GAME_DIMENSION.x - RAQUETTE_DIMENSION.x) / 2. - THICKNESS,
            );
        }
    }
}
