// Copyright Claudio Mattera 2021.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use rand_core::{RngCore, SeedableRng};
use rand_xorshift::XorShiftRng;

use crate::game::{Difficulty, Map};
use crate::graphics::{draw_text, DrawColors, Palette};
use crate::mouse::Mouse;
use crate::wasm4::get_random_seed;

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

        // Draw remaining mines count
        let remaining_mines = self.difficulty.mines_count();
        let s = format!("Mines:{:2}", remaining_mines);
        DrawColors.set(0x03);
        draw_text(s, 160 - 64, 2);

        // Draw elapsed time
        let s = format!("Time:{:3}", 0);
        draw_text(s, 2, 2);
    }

    pub fn update(mut self, mouse: &Mouse) -> Transition {
        if mouse.left_clicked() {
            let (mouse_x, mouse_y) = mouse.coordinates();
            if let Some((tx, ty)) = self.map.mouse_to_tile(mouse_x, mouse_y) {
                let seed = get_random_seed();
                let mines = self.place_mines_from_random_seed(
                    seed,
                    self.map.width(),
                    self.map.height(),
                    tx,
                    ty,
                );

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

    fn place_mines_from_random_seed(
        &self,
        seed: u32,
        width: usize,
        height: usize,
        forbidden_x: usize,
        forbidden_y: usize,
    ) -> Vec<(usize, usize)> {
        let seed = (seed as u64) << 32 | seed as u64;
        let mut mines = Vec::new();
        let mut generator = XorShiftRng::seed_from_u64(seed);
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
