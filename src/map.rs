// Copyright Claudio Mattera 2021.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use core::iter::Iterator;

use crate::debug;
use crate::graphics::Tile;

const TILE_SIZE: u32 = 10;
const MAX_WIDTH: usize = 16;
const MAX_HEIGHT: usize = 16;
const MAX_SIZE: usize = MAX_WIDTH * MAX_HEIGHT;

#[derive(Clone)]
pub struct Map {
    offset: (i32, i32),
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
}

impl Map {
    pub fn new(width: usize, height: usize, offset: (i32, i32)) -> Self {
        debug_assert!(width <= MAX_WIDTH);
        debug_assert!(height <= MAX_HEIGHT);

        let tiles = vec![Tile::Covered; width * height];
        Self {
            offset,
            tiles,
            width,
            height,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn has_started(&self) -> bool {
        true
    }

    pub fn has_stepped_on_mine(&self, mines: &[(usize, usize)]) -> bool {
        mines
            .iter()
            .map(|(x, y)| self.tile(*x, *y))
            .any(|tile| matches!(tile, Tile::Uncovered))
    }

    pub fn count_uncovered_tiles(&self) -> usize {
        self
            .tiles
            .iter()
            .filter(|tile| matches!(tile, Tile::Uncovered))
            .count()
    }

    fn flag_tile(&mut self, tx: usize, ty: usize) {
        match self.tile(tx, ty) {
            Tile::Uncovered => {}
            Tile::Covered => self.flag_individual_tile(tx, ty),
            Tile::Flagged => self.unflag_individual_tile(tx, ty),
        }
    }

    pub fn handle_left_click(&mut self, mouse_x: i16, mouse_y: i16, mines: &[(usize, usize)]) {
        if let Some((x, y)) = self.mouse_to_tile(mouse_x, mouse_y) {
            self.uncover_tile(x, y, mines)
        }
    }

    pub fn handle_right_click(&mut self, mouse_x: i16, mouse_y: i16) {
        if let Some((x, y)) = self.mouse_to_tile(mouse_x, mouse_y) {
            self.flag_tile(x, y)
        }
    }

    pub fn count_flagged_tiles(&self) -> usize {
        self.tiles
            .iter()
            .filter(|tile| matches!(tile, Tile::Flagged))
            .count()
    }

    pub fn uncover_tile(&mut self, initial_x: usize, initial_y: usize, mines: &[(usize, usize)]) {
        let mut tiles_to_uncover = vec![(initial_x, initial_y)];

        while let Some((x, y)) = tiles_to_uncover.pop() {
            debug!("{} tiles to uncover", tiles_to_uncover.len());
            debug!("Uncovering tile {}x{}", x, y);

            match self.tile(x, y) {
                Tile::Uncovered => continue,
                Tile::Covered => {
                    self.uncover_individual_tile(x, y);
                    let neighbour_mines = self.count_neighbour_mines(mines, x, y);
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
                            if cx >= 0
                                && cy >= 0
                                && cx < self.width as i32
                                && cy < self.height as i32
                            {
                                let tile = (cx as usize, cy as usize);
                                if !tiles_to_uncover.iter().any(|t| *t == tile) {
                                    tiles_to_uncover.push(tile);
                                }
                            }
                        }
                    }
                }
                Tile::Flagged => continue,
            }
        }
    }

    pub fn draw(&self, mines: &[(usize, usize)]) {
        for tx in 0..self.width {
            for ty in 0..self.height {
                let tile = self.tile(tx, ty);

                let x = tx as i32 * TILE_SIZE as i32;
                let y = ty as i32 * TILE_SIZE as i32;

                let is_mine = mines
                    .iter()
                    .any(|(mx, my)| (*mx, *my) == (tx, ty));

                let neighbour_mines = self.count_neighbour_mines(mines, tx, ty);
                tile.draw(self.offset.0 + x, self.offset.1 + y, is_mine, neighbour_mines);
            }
        }
    }

    fn count_neighbour_mines(&self, mines: &[(usize, usize)], x: usize, y: usize) -> usize {
        mines
            .iter()
            .filter(|(mx, my)| {
                let horizontally_adjacent = (*mx as i32 - x as i32).abs() <= 1;
                let vertically_adjacent = (*my as i32 - y as i32).abs() <= 1;
                horizontally_adjacent && vertically_adjacent
            })
            .count()
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

    pub fn mouse_to_tile(&self, mouse_x: i16, mouse_y: i16) -> Option<(usize, usize)> {
        let mouse_x = mouse_x - self.offset.0 as i16;
        let mouse_y = mouse_y - self.offset.1 as i16;
        if mouse_x < 0
            || mouse_y < 0
            || mouse_x / TILE_SIZE as i16 >= self.width as i16
            || mouse_y / TILE_SIZE as i16 >= self.height as i16
        {
            None
        } else {
            let (x, y) = (mouse_x / TILE_SIZE as i16, mouse_y / TILE_SIZE as i16);
            Some((x as usize, y as usize))
        }
    }
}
