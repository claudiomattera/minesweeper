// Copyright Claudio Mattera 2021.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::game::{Difficulty, Map};
use crate::graphics::Palette;
use crate::input::Mouse;
use crate::interface::{draw_elapsed_time, draw_remaining_mines_count};
use crate::sound::play_game_over_sound;
use crate::time::Timer;

use super::{GameOverState, GameWonState, PauseState, State, Transition};

const MAX_CLICK_AGE: u8 = 10;

#[derive(Clone)]
pub struct InGameState {
    difficulty: Difficulty,
    map: Map,
    timer: Timer,
    mines: Vec<(usize, usize)>,
    left_click_age: u8,
    right_click_age: u8,
}

impl InGameState {
    pub fn new(difficulty: Difficulty, map: Map, mines: Vec<(usize, usize)>) -> Self {
        Self {
            difficulty,
            map,
            mines,
            timer: Timer::new(),
            left_click_age: 0,
            right_click_age: 0,
        }
    }

    pub fn draw(&self, _mouse: Option<Mouse>) {
        Palette::Hollow.set();

        self.map.draw(&self.mines);

        let flagged_tiles = self.map.count_flagged_tiles();
        let remaining_mines = self.mines.len() - flagged_tiles;
        draw_remaining_mines_count(remaining_mines, 160 - 64, 2);

        let elapsed_time = self.timer.get();
        draw_elapsed_time(elapsed_time, 2, 2);
    }

    pub fn update(mut self, mouse: &Mouse) -> Transition {
        if self.map.has_stepped_on_mine(&self.mines) {
            for (mx, my) in &self.mines {
                self.map.flag_tile(*mx, *my);
            }

            play_game_over_sound();

            return Transition::Replace(State::GameOver(GameOverState::new(
                self.map, self.mines, self.timer,
            )));
        }

        if self.has_found_all_mines() {
            for (mx, my) in &self.mines {
                self.map.flag_tile(*mx, *my);
            }
            return Transition::Replace(State::GameWon(GameWonState::new(
                self.difficulty,
                self.map,
                self.mines,
                self.timer,
            )));
        }

        if mouse.left_clicked() {
            self.left_click_age = MAX_CLICK_AGE;
        }
        if mouse.right_clicked() {
            self.right_click_age = MAX_CLICK_AGE;
        }

        if self.left_click_age > 0 && self.right_click_age > 0 {
            let (x, y) = mouse.coordinates();
            self.map.handle_left_and_right_click(x, y, &self.mines);
        } else if mouse.left_clicked() {
            let (x, y) = mouse.coordinates();
            if self.map.mouse_to_tile(x, y).is_none() {
                return Transition::Push(State::InGame(self), State::Pause(PauseState::new()));
            }
            self.map.handle_left_click(x, y, &self.mines);
        } else if mouse.right_clicked() {
            let (x, y) = mouse.coordinates();
            self.map.handle_right_click(x, y);
        }

        self.timer.update();

        self.left_click_age = self.left_click_age.saturating_sub(1);
        self.right_click_age = self.right_click_age.saturating_sub(1);

        Transition::Replace(State::InGame(self))
    }

    fn has_found_all_mines(&self) -> bool {
        self.map.count_uncovered_tiles() + self.mines.len() == self.map.width() * self.map.height()
    }
}
