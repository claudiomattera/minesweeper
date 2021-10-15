// Copyright Claudio Mattera 2021.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::graphics::{draw_text, DrawColors};
use crate::highscores::save_high_score;
use crate::interface::draw_message_box;
use crate::mouse::Mouse;
use crate::Map;
use crate::Timer;

use super::{MainMenuState, State, Transition};

#[derive(Clone)]
pub struct GameWonState {
    map: Map,
    mines: Vec<(usize, usize)>,
    timer: Timer,
}

impl GameWonState {
    pub fn new(map: Map, mines: Vec<(usize, usize)>, timer: Timer) -> Self {
        save_high_score(mines.len() as u32, timer.get());
        Self { map, mines, timer }
    }

    pub fn draw(&self) {
        self.map.draw(&self.mines);

        // Draw remaining mines count
        let flagged_tiles = self.map.count_flagged_tiles();
        let remaining_mines = self.mines.len() - flagged_tiles;
        let s = format!("Mines:{:02}", remaining_mines);
        DrawColors.set(0x03);
        draw_text(s, 160 - 64, 2);

        // Draw elapsed time
        let s = format!("Time:{:3}", self.timer.get());
        draw_text(s, 2, 2);

        // Draw game state
        draw_message_box("VICTORY!!!", 30, 30);
    }

    pub fn update(self, mouse: &Mouse) -> Transition {
        if mouse.left_clicked() {
            return Transition::Replace(State::MainMenu(MainMenuState::new()));
        }

        Transition::Replace(State::GameWon(self))
    }
}
