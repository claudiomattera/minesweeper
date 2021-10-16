// Copyright Claudio Mattera 2021.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Game map

use core::iter::Iterator;

use crate::graphics::Tile;

const TILE_SIZE: u32 = 10;
const MAX_WIDTH: usize = 16;
const MAX_HEIGHT: usize = 16;

/// Represent the game map
///
/// The game map is a minefield.
/// It contains a matrix of tiles, which can be either covered, uncovered or
/// flagged.
///
/// It also contains the offset to which the map is drawn.
#[derive(Clone)]
pub struct Map {
    offset: (i32, i32),
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
}

impl Map {
    /// Create a new map with given size and drawing offset
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

    /// Return the map's width
    pub fn width(&self) -> usize {
        self.width
    }

    /// Return the map's height
    pub fn height(&self) -> usize {
        self.height
    }

    pub fn has_started(&self) -> bool {
        true
    }

    /// Check whether an uncovered tile contains a mine
    pub fn has_stepped_on_mine(&self, mines: &[(usize, usize)]) -> bool {
        mines
            .iter()
            .map(|(x, y)| self.tile(*x, *y))
            .any(|tile| matches!(tile, Tile::Uncovered))
    }

    /// Count the uncovered tiles
    pub fn count_uncovered_tiles(&self) -> usize {
        self.tiles
            .iter()
            .filter(|tile| matches!(tile, Tile::Uncovered))
            .count()
    }

    /// Flag a tile
    pub fn flag_tile(&mut self, tx: usize, ty: usize) {
        match self.tile(tx, ty) {
            Tile::Uncovered => {}
            Tile::Covered => self.flag_individual_tile(tx, ty),
            Tile::Flagged => {},
        }
    }

    /// Flip a tile
    ///
    /// This function flip the flagged status, from flagged to covered and
    /// viceversa
    pub fn flip_flagged_tile(&mut self, tx: usize, ty: usize) {
        match self.tile(tx, ty) {
            Tile::Uncovered => {}
            Tile::Covered => self.flag_individual_tile(tx, ty),
            Tile::Flagged => self.unflag_individual_tile(tx, ty),
        }
    }

    /// Handle mouse left clicks on the map
    pub fn handle_left_click(&mut self, mouse_x: i16, mouse_y: i16, mines: &[(usize, usize)]) {
        if let Some((x, y)) = self.mouse_to_tile(mouse_x, mouse_y) {
            self.uncover_tile(x, y, mines)
        }
    }

    /// Handle mouse right clicks on the map
    pub fn handle_right_click(&mut self, mouse_x: i16, mouse_y: i16) {
        if let Some((x, y)) = self.mouse_to_tile(mouse_x, mouse_y) {
            self.flip_flagged_tile(x, y)
        }
    }

    /// Handle simultaneous mouse left and right clicks on the map
    pub fn handle_left_and_right_click(&mut self, mouse_x: i16, mouse_y: i16, mines: &[(usize, usize)]) {
        if let Some((x, y)) = self.mouse_to_tile(mouse_x, mouse_y) {
            let neighbour_mines = self.count_neighbour_mines(mines, x, y);
            let neighbour_flags = self.count_neighbour_flags(x, y);
            if neighbour_mines == neighbour_flags {
                let candidates = self.find_neighbouring_uncoverable_tiles(x, y, mines);
                self.uncover_tiles(candidates, mines);
            }
        }
    }

    /// Count the flagged tiles
    pub fn count_flagged_tiles(&self) -> usize {
        self.tiles
            .iter()
            .filter(|tile| matches!(tile, Tile::Flagged))
            .count()
    }

    /// Recursively uncover a tile and its neighbours
    ///
    /// Once a tile is uncovered, the number of neighbouring mines and the
    /// number of neighbouring flagged tiles are compared.
    /// If they are the same, all non-flagged neighbouring tiles are also
    /// recursively uncovered.
    pub fn uncover_tile(&mut self, initial_x: usize, initial_y: usize, mines: &[(usize, usize)]) {
        let tiles = vec![(initial_x, initial_y)];
        self.uncover_tiles(tiles, mines)
    }

    /// Recursively uncover tiles and their neighbours
    ///
    /// Once a tile is uncovered, the number of neighbouring mines and the
    /// number of neighbouring flagged tiles are compared.
    /// If they are the same, all non-flagged neighbouring tiles are also
    /// recursively uncovered.
    pub fn uncover_tiles(
        &mut self,
        mut tiles_to_uncover: Vec<(usize, usize)>,
        mines: &[(usize, usize)],
    ) {
        while let Some((x, y)) = tiles_to_uncover.pop() {
            match self.tile(x, y) {
                Tile::Uncovered => continue,
                Tile::Covered => {
                    self.uncover_individual_tile(x, y);
                    let neighbour_mines = self.count_neighbour_mines(mines, x, y);
                    let neighbour_flags = self.count_neighbour_flags(x, y);
                    if neighbour_mines == neighbour_flags {
                        let candidates = self.find_neighbouring_uncoverable_tiles(x, y, mines);
                        for (cx, cy) in candidates {
                            let tile = (cx as usize, cy as usize);
                            if !tiles_to_uncover.iter().any(|t| *t == tile) {
                                tiles_to_uncover.push(tile);
                            }
                        }
                    }
                }
                Tile::Flagged => continue,
            }
        }
    }

    fn find_neighbouring_uncoverable_tiles(
        &self,
        x: usize,
        y: usize,
        mines: &[(usize, usize)],
    ) -> Vec<(usize, usize)> {
        // Must be a signed type to check for negative indices
        let x = x as i32;
        let y = y as i32;
        vec![
                (x + 1, y + 1),
                (x + 1, y - 1),
                (x - 1, y + 1),
                (x - 1, y - 1),
                (x, y + 1),
                (x, y - 1),
                (x + 1, y),
                (x - 1, y),
            ]
            .into_iter()
            .filter(|(cx, cy)| {
                *cx >= 0
                && *cy >= 0
                && *cx < self.width as i32
                && *cy < self.height as i32
            })
            .map(|(cx, cy)| (cx as usize, cy as usize))
            .collect()
    }

    /// Draw the map
    pub fn draw(&self, mines: &[(usize, usize)]) {
        for tx in 0..self.width {
            for ty in 0..self.height {
                let tile = self.tile(tx, ty);

                let x = tx as i32 * TILE_SIZE as i32;
                let y = ty as i32 * TILE_SIZE as i32;

                let is_mine = mines.iter().any(|(mx, my)| (*mx, *my) == (tx, ty));

                let neighbour_mines = self.count_neighbour_mines(mines, tx, ty);
                tile.draw(
                    self.offset.0 + x,
                    self.offset.1 + y,
                    is_mine,
                    neighbour_mines,
                );
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

    /// Map mouse coordinates to tile coordinates
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
