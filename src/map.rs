// Copyright Claudio Mattera 2021.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use core::iter::Iterator;

use rand_core::{RngCore, SeedableRng};
use rand_xorshift::XorShiftRng;

use crate::assets::FONT_SPRITE;
use crate::debug;
use crate::graphics::DrawColors;
use crate::wasm4::{hline, rect, vline};

const TILE_SIZE: u32 = 10;
const MAX_WIDTH: usize = 16;
const MAX_HEIGHT: usize = 16;
const MAX_SIZE: usize = MAX_WIDTH * MAX_HEIGHT;

pub struct Map<const MINES_COUNT: usize> {
    offset: (i32, i32),
    mines_positions: [(usize, usize); MINES_COUNT],
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
}

#[derive(Clone, Copy)]
enum Tile {
    Covered,
    Uncovered,
    Flagged,
}

impl<const MINES_COUNT: usize> Map<MINES_COUNT>
{
    pub fn from_random_seed(seed: u64, width: usize, height: usize, offset: (i32, i32)) -> Self {
        debug_assert!(width <= MAX_WIDTH);
        debug_assert!(height <= MAX_HEIGHT);

        let mut mines_positions = [(0, 0); MINES_COUNT];
        let mut generator = XorShiftRng::seed_from_u64(seed);
        for i in 0..MINES_COUNT {
            let mut x = generator.next_u32() as usize % width;
            let mut y = generator.next_u32() as usize % height;
            while mines_positions[0..i].iter().any(|pos| *pos == (x, y)) {
                x = generator.next_u32() as usize % width;
                y = generator.next_u32() as usize % height;
            }
            mines_positions[i] = (x, y);
        }
        let tiles = vec![Tile::Covered; width * height];
        Self {
            offset,
            mines_positions,
            tiles,
            width,
            height,
        }
    }

    fn flag_tile(&mut self, tx: usize, ty: usize) {
        match self.tile(tx, ty) {
            Tile::Uncovered => {}
            Tile::Covered => self.flag_individual_tile(tx, ty),
            Tile::Flagged => self.unflag_individual_tile(tx, ty),
        }
    }

    pub fn handle_left_click(&mut self, mouse_x: i16, mouse_y: i16) {
        if let Some((x, y)) = self.mouse_to_tile(mouse_x, mouse_y) {
            self.uncover_tile(x, y)
        }
    }

    pub fn handle_right_click(&mut self, mouse_x: i16, mouse_y: i16) {
        if let Some((x, y)) = self.mouse_to_tile(mouse_x, mouse_y) {
            self.flag_tile(x, y)
        }
    }

    pub fn count_remaining_mines(&self) -> usize {
        MINES_COUNT - self.count_flagged_mines()
    }

    fn count_flagged_mines(&self) -> usize {
        self.tiles
            .iter()
            .filter(|tile| {
                if let Tile::Flagged = tile {
                    true
                } else {
                    false
                }
            })
            .count()
    }

    fn uncover_tile(&mut self, initial_x: usize, initial_y: usize) {
        let mut tiles_to_uncover = Vec::new();
        tiles_to_uncover
            .push((initial_x, initial_y));

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
                            if cx >= 0 && cy >= 0 && cx < self.width as i32 && cy < self.height as i32 {
                                let tile = (cx as usize, cy as usize);
                                if !tiles_to_uncover.iter().any(|t| *t == tile) {
                                    tiles_to_uncover
                                        .push(tile);
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
        for tx in 0..self.width {
            for ty in 0..self.height {
                let tile = self.tile(tx, ty);

                let x = tx as i32 * TILE_SIZE as i32;
                let y = ty as i32 * TILE_SIZE as i32;

                self.draw_tile_border(x, y);

                match tile {
                    Tile::Covered => {
                        self.draw_tile_cover(x, y);
                    }
                    Tile::Uncovered => {
                        if self
                            .mines_positions
                            .iter()
                            .any(|(mx, my)| (*mx, *my) == (tx, ty))
                        {
                            self.draw_tile_character(x, y, Character::Mine);
                        } else {
                            let neighbour_mines = self.count_neighbour_mines(tx, ty);
                            if neighbour_mines > 0 {
                                self.draw_tile_character(x, y, Character::Number(neighbour_mines));
                            }
                        }
                    }
                    Tile::Flagged => {
                        self.draw_tile_cover(x, y);
                        self.draw_tile_character(x, y, Character::Flag);
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
            if cx >= 0 && cy >= 0 && cx < self.width as i32 && cy < self.height as i32 {
                if let Tile::Flagged = self.tile(cx as usize, cy as usize) {
                    count += 1;
                }
            }
        }

        count
    }

    fn tile(&self, x: usize, y: usize) -> &Tile {
        &self.tiles[x + y * self.width]
    }

    fn uncover_individual_tile(&mut self, x: usize, y: usize) {
        self.tiles[x + y * self.width] = Tile::Uncovered;
    }

    fn flag_individual_tile(&mut self, x: usize, y: usize) {
        self.tiles[x + y * self.width] = Tile::Flagged;
    }

    fn unflag_individual_tile(&mut self, x: usize, y: usize) {
        self.tiles[x + y * self.width] = Tile::Covered;
    }

    fn mouse_to_tile(&self, mouse_x: i16, mouse_y: i16) -> Option<(usize, usize)> {
        let mouse_x = mouse_x - self.offset.0 as i16;
        let mouse_y = mouse_y - self.offset.1 as i16;
        if mouse_x < 0 || mouse_y < 0 {
            None
        } else if mouse_x / TILE_SIZE as i16 >= self.width as i16
            || mouse_y / TILE_SIZE as i16 >= self.height as i16
        {
            None
        } else {
            let (x, y) = (mouse_x / TILE_SIZE as i16, mouse_y / TILE_SIZE as i16);
            Some((x as usize, y as usize))
        }
    }

    fn draw_tile_border(&self, x: i32, y: i32) {
        DrawColors.set(0x2);
        vline(self.offset.0 + x, self.offset.1 + y, TILE_SIZE - 1);
        hline(self.offset.0 + x, self.offset.1 + y, TILE_SIZE - 1);
    }

    fn draw_tile_cover(&self, x: i32, y: i32) {
        DrawColors.set(0x3);
        rect(
            self.offset.0 + x + 1,
            self.offset.1 + y + 1,
            TILE_SIZE - 2,
            TILE_SIZE - 2,
        );
    }

    fn draw_tile_character(&self, x: i32, y: i32, c: Character) {
        let offset = ((TILE_SIZE - 8) / 2) as i32;
        DrawColors.set(0x2240);
        match c {
            Character::Number(n) => FONT_SPRITE.blit_sub(
                self.offset.0 + x + offset,
                self.offset.1 + y + offset,
                8,
                8,
                8 * n as u32,
                0,
            ),
            Character::Mine => FONT_SPRITE.blit_sub(
                self.offset.0 + x + offset,
                self.offset.1 + y + offset,
                8,
                8,
                8 * 7,
                8 * 8,
            ),
            Character::Flag => FONT_SPRITE.blit_sub(
                self.offset.0 + x + offset,
                self.offset.1 + y + offset,
                8,
                8,
                8,
                8,
            ),
        }
    }
}

enum Character {
    Number(usize),
    Mine,
    Flag,
}
