use bevy::{input::mouse::MouseMotion, prelude::*};
use bevy_prototype_lyon::prelude::*;

use crate::{cons::*, resources::gamepad_handler};

#[derive(Component)]
pub struct Raquette;

pub const RAQUETTE_DIMENSION: Vec2 = Vec2 { x: 50., y: 10. };

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

pub const RAQUETTE_X_CLAMP: (f32, f32) = (
    (-GAME_DIMENSION.x + THICKNESS + RAQUETTE_DIMENSION.x) / 2.,
    (GAME_DIMENSION.x - THICKNESS - RAQUETTE_DIMENSION.x) / 2.,
);

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
                    transform.translation.x = world_position.x.clamp(RAQUETTE_X_CLAMP.0, RAQUETTE_X_CLAMP.1);
                }
            }
        }
    }
}

pub fn update_raquette_gamepad(gamepads: Res<Gamepads>, axes: Res<Axis<GamepadAxis>>, mut query: Query<&mut Transform, With<Raquette>>) {
    for gamepad in gamepads.iter() {
        let left_stick_x = match gamepad_handler::get_leftstickx(&axes, gamepad) {
            Some(value) => value,
            None => return,
        };

        for mut transform in query.iter_mut() {
            transform.translation.x = (transform.translation.x + 10. * left_stick_x.tan()).clamp(RAQUETTE_X_CLAMP.0, RAQUETTE_X_CLAMP.1);
        }
    }
}
