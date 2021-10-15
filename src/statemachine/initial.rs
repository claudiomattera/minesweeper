// Copyright Claudio Mattera 2021.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::mouse::Mouse;

use super::{PreGameState, State, Transition};

#[derive(Clone)]
pub struct InitialState {
}

impl InitialState {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn draw(&self) {
    }

    pub fn update(self, _mouse: &Mouse) -> Transition {
        Transition::Push(
            State::Initial(self.clone()),
            State::PreGame(PreGameState::new()),
        )
    }
}