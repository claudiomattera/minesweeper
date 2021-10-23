// Copyright Claudio Mattera 2021.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::game::Map;
use crate::graphics::Palette;
use crate::input::Mouse;
use crate::interface::{draw_elapsed_time, draw_message_box, draw_remaining_mines_count};
use crate::time::Timer;

use super::{MainMenuState, State, Transition};

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

    pub fn draw(&self, _mouse: Option<Mouse>) {
        Palette::Rustic.set();

        self.map.draw(&self.mines);

        let flagged_tiles = self.map.count_flagged_tiles();
        let remaining_mines = self.mines.len() - flagged_tiles;
        draw_remaining_mines_count(remaining_mines, 160 - 64, 2);

        let elapsed_time = self.timer.get();
        draw_elapsed_time(elapsed_time, 2, 2);

        draw_message_box("GAME OVER!!!", 30, 30);
    }

    pub fn update(self, mouse: &Mouse) -> Transition {
        if mouse.left_clicked() {
            return Transition::Replace(State::MainMenu(MainMenuState::new()));
        }

        Transition::Replace(State::GameOver(self))
    }
}
