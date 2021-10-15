// Copyright Claudio Mattera 2021.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::graphics::{DrawColors, draw_text};

use crate::mouse::Mouse;

use crate::Map;

use crate::Timer;

use super::{GameOverState, GameWonState, PauseState, State, Transition};

#[derive(Clone)]
pub struct InGameState {
    map: Map,
    timer: Timer,
    mines: Vec<(usize, usize)>,
}

impl InGameState {
    pub fn new(map: Map, mines: Vec<(usize, usize)>) -> Self {
        Self {
            map,
            mines,
            timer: Timer::new(),
        }
    }

    pub fn draw(&self) {
        let map = &self.map;

        // Draw map
        map.draw(&self.mines);

        // Draw remaining mines count
        let flagged_tiles = map.count_flagged_tiles();
        let remaining_mines = self.mines.len() - flagged_tiles;
        let s = format!("Mines:{:02}", remaining_mines);
        DrawColors.set(0x03);
        draw_text(s, 160 - 64, 2);

        // Draw elapsed time
        let s = format!("Time:{:3}", self.timer.get());
        draw_text(s, 2, 2);
    }

    pub fn update(mut self, mouse: &Mouse) -> Transition {
        let has_found_all_mines = self.has_found_all_mines();
        let map = &mut self.map;

        if map.has_stepped_on_mine(&self.mines) {
            for (mx, my) in &self.mines {
                map.flag_tile(*mx, *my);
            }
            return Transition::Replace(State::GameOver(GameOverState::new(self.map, self.mines, self.timer)));
        }

        if has_found_all_mines {
            for (mx, my) in &self.mines {
                map.flag_tile(*mx, *my);
            }
            return Transition::Replace(State::GameWon(GameWonState::new(self.map, self.mines, self.timer)));
        }

        if mouse.left_clicked() {
            let (x, y) = mouse.coordinates();
            if map.mouse_to_tile(x, y).is_none() {
                return Transition::Push(
                    State::InGame(self),
                    State::Pause(PauseState::new()),
                );
            }
            map.handle_left_click(x, y, &self.mines);
        }

        if mouse.right_clicked() {
            let (x, y) = mouse.coordinates();
            map.handle_right_click(x, y);
        }

        if map.has_started() && !map.has_stepped_on_mine(&self.mines) && !has_found_all_mines {
            self.timer.update();
        }

        Transition::Replace(State::InGame(self))
    }

    fn has_found_all_mines(&self) -> bool {
        self.map.count_uncovered_tiles() + self.mines.len() == self.map.width() * self.map.height()
    }
}
