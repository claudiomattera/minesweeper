// Copyright Claudio Mattera 2021.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::interface::draw_message_box;
use crate::input::Mouse;

use super::{State, Transition};

#[derive(Clone)]
pub struct PauseState {}

impl PauseState {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {}
    }

    pub fn draw(&self, _mouse: Option<Mouse>) {
        let x = 16;
        let y = 30;

        draw_message_box("Game paused\n\nClick to resume", x, y);
    }

    pub fn update(self, mouse: &Mouse) -> Transition {
        if mouse.left_clicked() {
            Transition::Pop
        } else {
            Transition::Replace(State::Pause(self))
        }
    }
}
