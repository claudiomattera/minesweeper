// Copyright Claudio Mattera 2021.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::assets::FONT_SPRITE;
use crate::graphics::{draw_horizontal_line, draw_rect, draw_vertical_line, DrawColors};

pub enum Character {
    Number(usize),
    Mine,
    Flag,
}

#[derive(Clone, Copy)]
pub enum Tile {
    Covered,
    Uncovered,
    Flagged,
}

impl Tile {
    pub fn draw(
        &self,
        x: i32,
        y: i32,
        tile_size: u32,
        is_mine: bool,
        neighbour_mines: usize,
    ) {
        self.draw_tile_border(x, y, tile_size);

        match self {
            Tile::Covered => {
                self.draw_tile_cover(x, y, tile_size);
            }
            Tile::Uncovered => {
                if is_mine {
                    self.draw_tile_character(x, y, tile_size, Character::Mine);
                } else if neighbour_mines > 0 {
                    self.draw_tile_character(x, y, tile_size, Character::Number(neighbour_mines));
                }
            }
            Tile::Flagged => {
                self.draw_tile_cover(x, y, tile_size);
                self.draw_tile_character(x, y, tile_size, Character::Flag);
            }
        }
    }

    fn draw_tile_border(&self, x: i32, y: i32, tile_size: u32) {
        DrawColors.set(0x2);
        draw_vertical_line(x, y, tile_size - 1);
        draw_horizontal_line(x, y, tile_size - 1);
    }

    fn draw_tile_cover(&self, x: i32, y: i32, tile_size: u32) {
        DrawColors.set(0x3);
        draw_rect(x + 1, y + 1, tile_size - 2, tile_size - 2);
    }

    fn draw_tile_character(&self, x: i32, y: i32, tile_size: u32, c: Character) {
        let offset = ((tile_size - 8) / 2) as i32;
        DrawColors.set(0x2240);
        match c {
            Character::Number(n) => {
                FONT_SPRITE.blit_sub(x + offset, y + offset, 8, 8, 8 * n as u32, 0)
            }
            Character::Mine => {
                DrawColors.set(0x1142);
                FONT_SPRITE.blit_sub(x + offset, y + offset, 8, 8, 8 * 7, 8 * 8);
            }
            Character::Flag => FONT_SPRITE.blit_sub(x + offset, y + offset, 8, 8, 8, 8),
        }
    }
}
