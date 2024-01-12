use bevy::ecs::system::{Res, Resource};

pub mod gamepad_handler {
    use bevy::input::gamepad::{Gamepad, GamepadButton, GamepadButtonType};

    use super::*;

    #[derive(Resource)]
    pub struct MyGamepad(Gamepad);

    pub fn get_a_button(my_gamepad: Option<Res<MyGamepad>>) -> Option<GamepadButton> {
        let gamepad = match get_gamepad(my_gamepad) {
            Ok(value) => value,
            Err(value) => return value,
        };
        let a_button = GamepadButton {
            gamepad,
            button_type: GamepadButtonType::East,
        };
        Some(a_button)
    }

    pub fn get_b_button(my_gamepad: Option<Res<MyGamepad>>) -> Option<GamepadButton> {
        let gamepad = match get_gamepad(my_gamepad) {
            Ok(value) => value,
            Err(value) => return value,
        };
        let b_button = GamepadButton {
            gamepad,
            button_type: GamepadButtonType::South,
        };
        Some(b_button)
    }

    fn get_gamepad(my_gamepad: Option<Res<MyGamepad>>) -> Result<Gamepad, Option<GamepadButton>> {
        let gamepad = if let Some(gp) = my_gamepad {
            gp.0
        } else {
            return Err(None);
        };
        Ok(gamepad)
    }
}
