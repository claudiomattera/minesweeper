// Copyright Claudio Mattera 2021.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use rand_core::RngCore;

use crate::game::{Difficulty, Map};
use crate::graphics::Palette;
use crate::input::Mouse;
use crate::interface::{draw_elapsed_time, draw_remaining_mines_count};
use crate::random::RNG;

use super::{InGameState, State, Transition};

#[derive(Clone)]
pub struct PreGameState {
    map: Map,
    difficulty: Difficulty,
}

impl PreGameState {
    pub fn new(difficulty: Difficulty) -> Self {
        let width = 16;
        let height = 14;
        let offset = (0, 20);
        let tile_size = 10;
        Self {
            map: Map::new(width, height, tile_size, offset),
            difficulty,
        }
    }

    pub fn draw(&self, _mouse: Option<Mouse>) {
        Palette::Hollow.set();

        self.map.draw(&[]);

        let remaining_mines = self.difficulty.mines_count();
        draw_remaining_mines_count(remaining_mines, 160 - 64, 2);

        let elapsed_time = 0;
        draw_elapsed_time(elapsed_time, 2, 2);
    }

    pub fn update(mut self, mouse: &Mouse) -> Transition {
        if mouse.left_clicked() {
            let (mouse_x, mouse_y) = mouse.coordinates();
            if let Some((tx, ty)) = self.map.mouse_to_tile(mouse_x, mouse_y) {
                let mines = self.place_mines_randomly(self.map.width(), self.map.height(), tx, ty);

                self.map.uncover_tile(tx, ty, &mines);
                return Transition::Replace(State::InGame(InGameState::new(
                    self.difficulty,
                    self.map,
                    mines,
                )));
            }
        }

        Transition::Replace(State::PreGame(self))
    }

    fn place_mines_randomly(
        &self,
        width: usize,
        height: usize,
        forbidden_x: usize,
        forbidden_y: usize,
    ) -> Vec<(usize, usize)> {
        let mut mines = Vec::new();
        let generator = unsafe { &mut RNG };
        for i in 0..self.difficulty.mines_count() {
            let mut x = generator.next_u32() as usize % width;
            let mut y = generator.next_u32() as usize % height;
            while mines[0..i].iter().any(|pos| *pos == (x, y))
                || x == forbidden_x && y == forbidden_y
            {
                x = generator.next_u32() as usize % width;
                y = generator.next_u32() as usize % height;
            }
            mines.push((x, y));
        }
        mines
    }
}
