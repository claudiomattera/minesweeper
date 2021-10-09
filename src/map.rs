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

const TILE_SIZE: u32 = 10;

#[derive(Clone, Copy)]
pub struct Map<const WIDTH: usize, const HEIGHT: usize, const TOTAL: usize, const MINES_COUNT: usize> {
    offset: (i32, i32),
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
    pub fn from_random_seed(seed: u64, offset: (i32, i32)) -> Self {
        let mut mines_positions = [(0, 0); MINES_COUNT];
        let mut generator = XorShiftRng::seed_from_u64(seed);
        for i in 0..MINES_COUNT {
            let mut x = generator.next_u32() as usize % WIDTH;
            let mut y = generator.next_u32() as usize % HEIGHT;
            while let Some(_) = mines_positions[0..i].iter().find(|pos| **pos == (x, y)) {
                x = generator.next_u32() as usize % WIDTH;
                y = generator.next_u32() as usize % HEIGHT;
            }
            mines_positions[i] = (x, y);
        }
        let tiles = [Tile::Covered; TOTAL];
        Self {
            offset,
            mines_positions,
            tiles,
        }
    }

    fn flag_tile(&mut self, tx: usize, ty: usize) {
        match self.tile(tx, ty) {
            Tile::Uncovered => {}
            Tile::Covered => self.flag_individual_tile(tx, ty),
            Tile::Flagged => self.unflag_individual_tile(tx, ty)
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

    fn uncover_tile(&mut self, initial_x: usize, initial_y: usize) {
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
        for tx in 0..WIDTH {
            for ty in 0..HEIGHT {
                let tile = self.tile(tx, ty);

                let x = tx as i32 * TILE_SIZE as i32;
                let y = ty as i32 * TILE_SIZE as i32;

                match tile {
                    Tile::Covered => {
                        self.draw_tile_border(x, y);
                        self.draw_tile_cover(x, y);
                    }
                    Tile::Uncovered => {
                        if let Some(_) = self.mines_positions.iter().find(|(mx, my)| (*mx, *my) == (tx, ty)) {
                            self.draw_tile_border(x, y);
                            self.draw_tile_character(x, y, Character::Mine);
                        } else {
                            let neighbour_mines = self.count_neighbour_mines(tx, ty);
                            if neighbour_mines > 0 {
                                self.draw_tile_border(x, y);
                                self.draw_tile_character(x, y, Character::Number(neighbour_mines));
                            } else {
                                self.draw_tile_border(x, y);
                            }
                        }
                    }
                    Tile::Flagged => {
                        self.draw_tile_border(x, y);
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

    fn mouse_to_tile(&self, mouse_x: i16, mouse_y: i16) -> Option<(usize, usize)> {
        let mouse_x = mouse_x - self.offset.0 as i16;
        let mouse_y = mouse_y - self.offset.1 as i16;
        if mouse_x < 0 || mouse_y < 0 {
            None
        } else if mouse_x / TILE_SIZE as i16 >= WIDTH as i16 || mouse_y / TILE_SIZE as i16 >= HEIGHT as i16 {
            None
        } else {
            let (x, y) = (mouse_x / TILE_SIZE as i16, mouse_y / TILE_SIZE as i16);
            Some((x as usize, y as usize))
        }
    }

    fn draw_tile_border(&self, x: i32, y: i32) {
        let draw_colors = DrawColors::new();
        draw_colors.set(0x2);
        vline(self.offset.0 + x, self.offset.1 + y, TILE_SIZE - 1);
        hline(self.offset.0 + x, self.offset.1 + y, TILE_SIZE - 1);
    }

    fn draw_tile_cover(&self, x: i32, y: i32) {
        let draw_colors = DrawColors::new();
        draw_colors.set(0x3);
        rect(self.offset.0 + x + 1, self.offset.1 + y + 1, TILE_SIZE - 2, TILE_SIZE - 2);
    }

    fn draw_tile_character(&self, x: i32, y: i32, c: Character) {
        let offset = ((TILE_SIZE - 8) / 2) as i32;
        let draw_colors = DrawColors::new();
        draw_colors.set(0x2240);
        match c {
            Character::Number(n) => FONT_SPRITE.blit_sub(self.offset.0 + x + offset, self.offset.1 + y + offset, 8, 8, 8 * n as u32, 0),
            Character::Mine => FONT_SPRITE.blit_sub(self.offset.0 + x + offset, self.offset.1 + y + offset, 8, 8, 8 * 7, 8 * 8),
            Character::Flag => FONT_SPRITE.blit_sub(self.offset.0 + x + offset, self.offset.1 + y + offset, 8, 8, 8, 8),
        }
    }
}

enum Character {
    Number(usize),
    Mine,
    Flag,
}
