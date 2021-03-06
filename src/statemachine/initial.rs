// Copyright Claudio Mattera 2021.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::input::Mouse;

use super::{MainMenuState, State, Transition};

#[derive(Clone)]
pub struct InitialState {}

impl InitialState {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {}
    }

    pub fn draw(&self, _mouse: Option<Mouse>) {}

    pub fn update(self, _mouse: &Mouse) -> Transition {
        Transition::Push(State::Initial(self), State::MainMenu(MainMenuState::new()))
    }
}
