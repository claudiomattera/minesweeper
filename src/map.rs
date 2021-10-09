// Copyright Claudio Mattera 2021.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use core::iter::Iterator;

use rand_core::{RngCore, SeedableRng};
use rand_xorshift::XorShiftRng;

use fixedvec::alloc_stack;
use fixedvec::FixedVec;

use crate::assets::FONT_SPRITE;
use crate::debug;
use crate::graphics::DrawColors;
use crate::wasm4::{hline, rect, vline};

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
    Flagged,
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
        let tiles = [Tile::Covered; TOTAL];
        Self {
            mines_positions,
            tiles,
        }
    }

    pub fn flag_tile(&mut self, mouse_x: usize, mouse_y: usize) {
        let (tx, ty) = (mouse_x / WIDTH, mouse_y / HEIGHT);
        match self.tile(tx, ty) {
            Tile::Uncovered => {}
            Tile::Covered => self.flag_individual_tile(tx, ty),
            Tile::Flagged => self.unflag_individual_tile(tx, ty)
        }
    }

    pub fn uncover_tile(&mut self, mouse_x: usize, mouse_y: usize) {
        let (initial_x, initial_y) = (mouse_x / WIDTH, mouse_y / HEIGHT);

        let mut preallocated_space = alloc_stack!([(usize, usize); TOTAL]);
        let mut tiles_to_uncover = FixedVec::new(&mut preallocated_space);
        tiles_to_uncover.push((initial_x, initial_y))
            .expect("Pushing to a full vector");

        while let Some((x, y)) = tiles_to_uncover.pop() {
            debug!("{} tiles to uncover", tiles_to_uncover.len());
            debug!("Uncovering tile {}x{}", x, y);

            match self.tile(x, y) {
                Tile::Uncovered => continue,
                Tile::Covered => {
                    self.uncover_individual_tile(x, y);
                    let neighbour_mines = self.count_neighbour_mines(x, y);
                    let neighbour_flags = self.count_neighbour_flags(x, y);
                    if neighbour_mines == neighbour_flags {
                        let x = x as i32;
                        let y = y as i32;
                        let candidates = [
                            (x + 1, y + 1),
                            (x + 1, y - 1),
                            (x - 1, y + 1),
                            (x - 1, y - 1),
                            (x, y + 1),
                            (x, y - 1),
                            (x + 1, y),
                            (x - 1, y),
                        ];
                        for (cx, cy) in candidates {
                            if cx >= 0 && cy >= 0 && cx < WIDTH as i32 && cy < HEIGHT as i32 {
                                let tile = (cx as usize, cy as usize);
                                if let None = tiles_to_uncover.iter().find(|t| **t == tile) {
                                    tiles_to_uncover.push(tile).expect("Pushing to a full vector");
                                }
                            }
                        }
                    }
                }
                Tile::Flagged => continue,
            }
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
                        draw_colors.set(0x3);
                        rect(x + 1, y + 1, 8, 8);
                    }
                    Tile::Uncovered => {
                        if let Some(_) = self.mines_positions.iter().find(|(mx, my)| (*mx, *my) == (tx, ty)) {
                            draw_colors.set(0x2240);
                            FONT_SPRITE.blit_sub(x + 1, y + 1, 8, 8, 8 * 7, 8 * 8);
                        } else {
                            let neighbour_mines = self.count_neighbour_mines(tx, ty);
                            if neighbour_mines > 0 {
                                draw_colors.set(0x2);
                                vline(x, y, 9);
                                hline(x, y, 9);
                                draw_colors.set(0x2240);
                                FONT_SPRITE.blit_sub(x + 1, y + 1, 8, 8, 8 * neighbour_mines as u32, 0);
                            } else {
                                draw_colors.set(0x2);
                                vline(x, y, 9);
                                hline(x, y, 9);
                                draw_colors.set(0x1);
                                rect(x + 1, y + 1, 8, 8);
                            }
                        }
                    }
                    Tile::Flagged => {
                        draw_colors.set(0x2);
                        vline(x, y, 9);
                        hline(x, y, 9);
                        draw_colors.set(0x3);
                        rect(x + 1, y + 1, 8, 8);
                        draw_colors.set(0x2240);
                        FONT_SPRITE.blit_sub(x + 1, y + 1, 8, 8, 8, 8);
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

    fn count_neighbour_flags(&self, x: usize, y: usize) -> usize {
        let x = x as i32;
        let y = y as i32;
        let mut count = 0;

        let candidates = [
            (x + 1, y + 1),
            (x + 1, y - 1),
            (x - 1, y + 1),
            (x - 1, y - 1),
            (x, y + 1),
            (x, y - 1),
            (x + 1, y),
            (x - 1, y),
        ];
        for (cx, cy) in candidates {
            if cx >= 0 && cy >= 0 && cx < WIDTH as i32 && cy < HEIGHT as i32 {
                if let Tile::Flagged = self.tile(cx as usize, cy as usize) {
                    count += 1;
                }
            }
        }

        count
    }

    fn tile(&self, x: usize, y: usize) -> &Tile {
        &self.tiles[x + y * WIDTH]
    }

    fn uncover_individual_tile(&mut self, x: usize, y: usize) {
        self.tiles[x + y * WIDTH] = Tile::Uncovered;
    }

    fn flag_individual_tile(&mut self, x: usize, y: usize) {
        self.tiles[x + y * WIDTH] = Tile::Flagged;
    }

    fn unflag_individual_tile(&mut self, x: usize, y: usize) {
        self.tiles[x + y * WIDTH] = Tile::Covered;
    }
}
