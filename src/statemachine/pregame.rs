// Copyright Claudio Mattera 2021.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use rand_core::{RngCore, SeedableRng};
use rand_xorshift::XorShiftRng;

use crate::mouse::Mouse;
use crate::graphics::{DrawColors, Tile};
use crate::Map;
use crate::wasm4::text;

use super::{InGameState, State, Transition};

const MINES_COUNT: usize = 5;

#[derive(Clone)]
pub struct PreGameState {
    map: Map,
}

impl PreGameState {
    pub fn new() -> Self {
        let width = 16;
        let height = 14;
        Self {
            map: Map::new(width, height, (0, 20)),
        }
    }

    pub fn draw(&self) {
        self.map.draw(&[]);

        // Draw remaining mines count
        let remaining_mines = 50;
        let s = format!("Mines:{:02}", remaining_mines);
        DrawColors.set(0x03);
        text(&s, 160 - 64, 2);

        // Draw elapsed time
        let s = format!("Time:{:3}", 0);
        text(s, 2, 2);
    }

    pub fn update(self, mouse: &Mouse) -> Transition {
        if mouse.left_clicked() {
            let (mouse_x, mouse_y) = mouse.coordinates();
            if let Some((tx, ty)) = self.map.mouse_to_tile(mouse_x, mouse_y) {
                let mut map = self.map;
                let mines = Self::place_mines_from_random_seed(0, map.width(), map.height(), tx, ty);
                map.uncover_tile(tx, ty, &mines);
                return Transition::Replace(
                    State::InGame(InGameState::new(map, mines)),
                )
            }
        }

        Transition::Replace(
            State::PreGame(self),
        )
    }

    fn place_mines_from_random_seed(
        seed: u64,
        width: usize,
        height: usize,
        forbidden_x: usize,
        forbidden_y: usize,
    ) -> Vec<(usize, usize)> {
        let mut mines = Vec::new();
        let mut generator = XorShiftRng::seed_from_u64(seed);
        for i in 0..MINES_COUNT {
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
