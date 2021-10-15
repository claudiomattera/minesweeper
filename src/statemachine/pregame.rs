// Copyright Claudio Mattera 2021.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::mouse::Mouse;
use crate::graphics::{DrawColors, Tile};
use crate::wasm4::text;

use super::{InGameState, State, Transition};

#[derive(Clone)]
pub struct PreGameState {
}

impl PreGameState {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn draw(&self) {
        // Draw remaining mines count
        let remaining_mines = 50;
        let s = format!("Mines:{:02}", remaining_mines);
        DrawColors.set(0x03);
        text(&s, 160 - 64, 2);

        // Draw elapsed time
        let s = format!("Time:{:3}", 0);
        text(s, 2, 2);

        // Draw grid
        let width = 10;
        let height = 10;
        for tx in 0..width {
            for ty in 0..height {
                let tile = Tile::Covered;

                let x = tx as i32 * 10 as i32;
                let y = ty as i32 * 10 as i32;

                let is_mine = false;
                let neighbour_mines = 0;
                tile.draw(x, y, is_mine, neighbour_mines);
            }
        }
    }

    pub fn update(self, mouse: &Mouse) -> Transition {
        if mouse.left_clicked() {
            return Transition::Replace(
                State::InGame(InGameState::new()),
            )
        }

        Transition::Replace(
            State::PreGame(self),
        )
    }
}
