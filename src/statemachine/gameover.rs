// Copyright Claudio Mattera 2021.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::mouse::Mouse;
use crate::graphics::DrawColors;
use crate::Map;
use crate::Timer;
use crate::wasm4::text;

use super::{PreGameState, State, Transition};

#[derive(Clone)]
pub struct GameOverState {
    map: Map,
    mines: Vec<(usize, usize)>,
    timer: Timer,
}

impl GameOverState {
    pub fn new(map: Map, mines: Vec<(usize, usize)>, timer: Timer) -> Self {
        Self { map, mines, timer }
    }

    pub fn draw(&self) {
        self.map.draw(&self.mines);

        // Draw remaining mines count
        let flagged_tiles = self.map.count_flagged_tiles();
        let remaining_mines = self.mines.len() - flagged_tiles;
        let s = format!("Mines:{:02}", remaining_mines);
        DrawColors.set(0x03);
        text(&s, 160 - 64, 2);

        // Draw elapsed time
        let s = format!("Time:{:3}", self.timer.get());
        text(s, 2, 2);

        // Drat game state
        text("GAME OVER!!!", 2, 10);
    }

    pub fn update(self, mouse: &Mouse) -> Transition {
        if mouse.left_clicked() {
            return Transition::Replace(
                State::PreGame(PreGameState::new()),
            )
        }

        Transition::Replace(
            State::GameOver(self),
        )
    }
}
