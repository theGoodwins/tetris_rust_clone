//! inputs.rs
//! Handles both gamepad and keyboard input using gilrs.
//! If a gamepad is active, its events call public methods on GameState;
//! otherwise, GameState’s own keyboard fallback (process_input) is used.

use gilrs::{Gilrs, GamepadId, Button, Event};
use macroquad::prelude::*;
use crate::GameState;

pub struct InputManager {
    gilrs: Gilrs,
    active_gamepad: Option<GamepadId>,
}

impl InputManager {
    pub fn new() -> Self {
        let gilrs = Gilrs::new().unwrap();
        for (_id, gamepad) in gilrs.gamepads() {
            println!("Gamepad {}: {:?}", gamepad.name(), gamepad.power_info());
        }
        Self {
            gilrs,
            active_gamepad: None,
        }
    }

    pub fn update(&mut self) {
        while let Some(Event { id, event, .. }) = self.gilrs.next_event() {
            println!("Gamepad event from {:?}: {:?}", id, event);
            self.active_gamepad = Some(id);
        }
    }

    pub fn process_input(&mut self, game: &mut GameState, delta: f32) {
        self.update();
        if let Some(gamepad) = self.active_gamepad.map(|id| self.gilrs.gamepad(id)) {
            if gamepad.is_pressed(Button::South) {
                game.process_hard_drop_input();
            }
            if gamepad.is_pressed(Button::DPadLeft) {
                game.process_horizontal_input(delta, -1);
            }
            if gamepad.is_pressed(Button::DPadRight) {
                game.process_horizontal_input(delta, 1);
            }
            if gamepad.is_pressed(Button::DPadDown) {
                game.process_soft_drop_input();
            }
            if gamepad.is_pressed(Button::West) {
                game.process_rotation_input(false);
            }
            if gamepad.is_pressed(Button::East) {
                game.process_rotation_input(true);
            }
            if gamepad.is_pressed(Button::Start) {
                game.toggle_pause();
            }
            if gamepad.is_pressed(Button::Select) {
                game.process_hold_input();
            }
        } else {
            // Fall back to keyboard input.
            game.process_input(delta);
        }
    }
}
