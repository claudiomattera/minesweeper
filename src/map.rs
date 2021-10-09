// Copyright Claudio Mattera 2021.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use rand_core::{RngCore, SeedableRng};
use rand_xorshift::XorShiftRng;

use crate::assets::FONT_SPRITE;
use crate::graphics::DrawColors;
use crate::wasm4::{hline, vline};

pub type Map10x10x10 = Map<10, 10, 100, 10>;

#[derive(Clone, Copy)]
pub struct Map<const WIDTH: usize, const HEIGHT: usize, const TOTAL: usize, const MINES_COUNT: usize> {
    pub mines_positions: [(usize, usize); MINES_COUNT],
    tiles: [Tile; TOTAL],
}

#[derive(Clone, Copy)]
enum Tile {
    Covered,
    Uncovered,
}

impl <const WIDTH: usize, const HEIGHT: usize, const TOTAL: usize, const MINES_COUNT: usize> Map<WIDTH, HEIGHT, TOTAL, MINES_COUNT> {
    pub fn from_random_seed(seed: u64) -> Self {
        let mut mines_positions = [(0, 0); MINES_COUNT];
        let mut generator = XorShiftRng::seed_from_u64(seed);
        for i in 0..MINES_COUNT {
            let x = generator.next_u32() as usize;
            let y = generator.next_u32() as usize;
            mines_positions[i] = (x % WIDTH, y % HEIGHT);
        }
        let mut tiles = [Tile::Covered; TOTAL];
        tiles[12] = Tile::Uncovered;
        tiles[44] = Tile::Uncovered;
        tiles[63] = Tile::Uncovered;
        tiles[43] = Tile::Uncovered;
        Self {
            mines_positions,
            tiles,
        }
    }

    pub fn draw(&self) {
        let draw_colors = DrawColors::new();
        for tx in 0..WIDTH {
            for ty in 0..HEIGHT {
                let tile = self.tile(tx, ty);

                let x = tx as i32 * 10;
                let y = ty as i32 * 10;

                match tile {
                    Tile::Covered => {
                        draw_colors.set(0x2);
                        vline(x, y, 9);
                        hline(x, y, 9);
                    }
                    Tile::Uncovered => {
                        let neighbour_mines = self.count_neighbour_mines(tx, ty);
                        draw_colors.set(0x2240);
                        FONT_SPRITE.blit_sub(x + 1, y + 1, 8, 8, 8 * neighbour_mines as u32, 0);
                    }
                }
            }
        }
    }

    fn count_neighbour_mines(&self, x: usize, y: usize) -> usize {
        let mut count = 0;
        for (mx, my) in self.mines_positions {
            if (mx as i32 - x as i32).abs() <= 1 && (my as i32 - y as i32).abs() <= 1 {
                count += 1;
            }
        }
        count
    }

    fn tile(&self, x: usize, y: usize) -> &Tile {
        &self.tiles[x + y * WIDTH]
    }
}
