// Copyright Claudio Mattera 2021.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::mouse::Mouse;
use crate::debug;
use crate::map::Map10x10x10;

use super::{State, Transition};

#[derive(Clone, Copy)]
pub struct InGameState {
    map: Map10x10x10,
}

impl InGameState {
    pub fn new() -> Self {
        let seed = 0;
        debug!("Creating map with seed {}", seed);
        let map = Map10x10x10::from_random_seed(seed);
        for (x, y) in map.mines_positions {
            debug!("Mine: {}, {}", x, y);
        }
        Self {
            map,
        }
    }

    pub fn draw(&self) {
        self.map.draw();
    }

    pub fn update(self, _mouse: Option<&Mouse>) -> Transition {
        Transition::Switch(State::InGame(self))
    }
}
