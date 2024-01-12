use bevy::ecs::system::Res;

pub mod gamepad_handler {
    use bevy::input::{
        gamepad::{Gamepad, GamepadAxis, GamepadAxisType, GamepadButton, GamepadButtonType, Gamepads},
        Axis,
    };

    use super::*;

    pub fn get_leftstickx(axes: &Res<Axis<GamepadAxis>>, gamepad: Gamepad) -> Option<f32> {
        axes.get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX))
    }

    pub fn get_a_button(gamepads: Res<Gamepads>) -> Option<GamepadButton> {
        if let Some(gamepad) = gamepads.iter().next() {
            let a_button = GamepadButton {
                gamepad,
                button_type: GamepadButtonType::East,
            };
            Some(a_button)
        } else {
            None
        }
    }

    pub fn get_b_button(gamepads: Res<Gamepads>) -> Option<GamepadButton> {
        if let Some(gamepad) = gamepads.iter().next() {
            let b_button = GamepadButton {
                gamepad,
                button_type: GamepadButtonType::South,
            };
            Some(b_button)
        } else {
            None
        }
    }
}
