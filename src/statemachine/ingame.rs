// Copyright Claudio Mattera 2021.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::debug;
use crate::map::Map;
use crate::mouse::Mouse;

use super::{State, Transition};

#[derive(Clone, Copy)]
pub struct InGameState {
    map: Map<10>,
}

impl InGameState {
    pub fn new() -> Self {
        let seed = 0;
        debug!("Creating map with seed {}", seed);
        let width = 10;
        let height = 11;
        let map = Map::<10>::from_random_seed(seed, width, height, (10, 20));
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

    pub fn update(self, mouse: Option<&Mouse>) -> Transition {
        if let Some(mouse) = mouse {
            if mouse.left_clicked() {
                let (x, y) = mouse.coordinates();
                let mut map = self.map;
                map.handle_left_click(x, y);
                return Transition::Switch(State::InGame(InGameState { map }));
            }
            if mouse.right_clicked() {
                let (x, y) = mouse.coordinates();
                let mut map = self.map;
                map.handle_right_click(x, y);
                return Transition::Switch(State::InGame(InGameState { map }));
            }
        }
        Transition::Switch(State::InGame(self))
    }
}
