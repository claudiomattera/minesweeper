// Copyright Claudio Mattera 2021.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::mouse::Mouse;

use crate::graphics::DrawColors;
use crate::wasm4::{blit, text, BLIT_1BPP, BUTTON_1, GAMEPAD1};

use super::{State, Transition};

#[derive(Clone, Copy)]
pub struct MainMenuState {}

impl MainMenuState {
    pub fn new() -> Self {
        Self {}
    }

    pub fn draw(&self) {
        let draw_colors = DrawColors::new();
        draw_colors.set(2);
        text("Hello from Rust!", 10, 10);

        let gamepad = unsafe { *GAMEPAD1 };
        if gamepad & BUTTON_1 != 0 {
            draw_colors.set(4);
        }

        blit(&SMILEY, 76, 76, 8, 8, BLIT_1BPP);
        text("Press X to blink", 16, 90);
    }

    pub fn update(self, mouse: Option<&Mouse>) -> Transition {
        if mouse.map(Mouse::left_clicked).unwrap_or(false) {
            Transition::Switch(State::MainMenu(self))
        } else {
            Transition::Switch(State::MainMenu(self))
        }
    }
}

#[rustfmt::skip]
const SMILEY: [u8; 8] = [
    0b11000011,
    0b10000001,
    0b00100100,
    0b00100100,
    0b00000000,
    0b00100100,
    0b10011001,
    0b11000011,
];
