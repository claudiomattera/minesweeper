// Copyright Claudio Mattera 2021.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::mouse::Mouse;
use crate::graphics::DrawColors;
use crate::wasm4::{rect, text};

use super::{State, Transition};

#[derive(Clone)]
pub struct PauseState {
}

impl PauseState {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn draw(&self) {
        let x = 16;
        let y = 30;

        DrawColors.set(0x44);
        rect(x + 3, y + 3, 160 - 2 * x as u32, 30);
        DrawColors.set(0x21);
        rect(x, y, 160 - 32, 30);

        DrawColors.set(0x03);
        text("Game paused", 20, y + 5);
        text("Click to resume", 20, y + 15);
    }

    pub fn update(self, mouse: &Mouse) -> Transition {
        if mouse.left_clicked() {
            Transition::Pop
        } else {
            Transition::Replace(State::Pause(self))
        }
    }
}