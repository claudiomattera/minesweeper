// Copyright Claudio Mattera 2021.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::graphics::{draw_text, DrawColors, Tile};
use crate::interface::draw_box;
use crate::input::Mouse;

use super::{State, Transition};

#[derive(Clone)]
pub struct InstructionsState {
    index: usize,
}

const TILE_SIZE: u32 = 10;

impl InstructionsState {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self { index: 0 }
    }

    pub fn draw(&self, _mouse: Option<Mouse>) {
        let x = 10;
        let y = 10;

        draw_box(x, y, (160 - 4 - 2 * x) as u32, (160 - 4 - 2 * y) as u32);

        PAGES[self.index](x, y);
    }

    pub fn update(mut self, mouse: &Mouse) -> Transition {
        if mouse.left_clicked() {
            if self.index >= PAGES.len() - 1 {
                Transition::Pop
            } else {
                self.index += 1;
                Transition::Replace(State::Instructions(self))
            }
        } else {
            Transition::Replace(State::Instructions(self))
        }
    }
}

const PAGES: [fn(i32, i32)->(); 8] = [
    |x, y| {
        DrawColors.set(3);
        draw_text(
            "Click on a tile
to uncover it.",
            x + 3,
            y + 3,
        );

        Tile::Covered.draw(x + 10, y + 50, TILE_SIZE, false, 0);
        draw_text("->", x + 20, y + 51);

        Tile::Uncovered.draw(x + 40, y + 50, TILE_SIZE, false, 0);
    },
    |x, y| {
        DrawColors.set(3);
        draw_text(
            "If it was a mine
you lose!",
            x + 3,
            y + 3,
        );

        Tile::Covered.draw(x + 10, y + 50, TILE_SIZE, false, 0);
        draw_text("->", x + 20, y + 51);

        Tile::Uncovered.draw(x + 40, y + 50, TILE_SIZE, true, 0);
    },
    |x, y| {
        DrawColors.set(3);
        draw_text(
            "Right click on a
tile to flag it.",
            x + 3,
            y + 3,
        );

        Tile::Covered.draw(x + 10, y + 50, TILE_SIZE, false, 0);
        draw_text("->", x + 20, y + 51);

        Tile::Flagged.draw(x + 40, y + 50, TILE_SIZE, false, 0);

        DrawColors.set(3);
        draw_text(
            "Flagged tiles
cannot be
uncovered.",
            x + 3,
            y + 73,
        );
    },
    |x, y| {
        DrawColors.set(3);
        draw_text(
            "Numbers shown in
uncovered tiles
tell you how
many mines are
adjacent,
horizontally,
vertically,
or diagonally.",
            x + 3,
            y + 3,
        );

        Tile::Uncovered.draw(x + 10, y + 85, TILE_SIZE, false, 1);
        Tile::Uncovered.draw(x + 20, y + 85, TILE_SIZE, true, 0);
        Tile::Uncovered.draw(x + 30, y + 85, TILE_SIZE, false, 2);

        Tile::Uncovered.draw(x + 10, y + 95, TILE_SIZE, false, 2);
        Tile::Uncovered.draw(x + 20, y + 95, TILE_SIZE, false, 4);
        Tile::Uncovered.draw(x + 30, y + 95, TILE_SIZE, true, 0);

        Tile::Uncovered.draw(x + 10, y + 105, TILE_SIZE, true, 0);
        Tile::Uncovered.draw(x + 20, y + 105, TILE_SIZE, false, 3);
        Tile::Uncovered.draw(x + 30, y + 105, TILE_SIZE, true, 0);
    },
    |x, y| {
        DrawColors.set(3);
        draw_text(
            "When uncovering
a tile, all
adjacent tiles
are uncovered
if the number of
adjacent tiles
is the same as
the number of
adjacent mines.",
            x + 3,
            y + 3,
        );

        Tile::Covered.draw(x + 10, y + 85, TILE_SIZE, false, 0);
        Tile::Flagged.draw(x + 20, y + 85, TILE_SIZE, true, 0);
        Tile::Covered.draw(x + 30, y + 85, TILE_SIZE, false, 0);

        Tile::Covered.draw(x + 10, y + 95, TILE_SIZE, false, 0);
        Tile::Uncovered.draw(x + 20, y + 95, TILE_SIZE, false, 4);
        Tile::Flagged.draw(x + 30, y + 95, TILE_SIZE, true, 0);

        Tile::Flagged.draw(x + 10, y + 105, TILE_SIZE, true, 0);
        Tile::Covered.draw(x + 20, y + 105, TILE_SIZE, false, 0);
        Tile::Flagged.draw(x + 30, y + 105, TILE_SIZE, true, 0);

        DrawColors.set(3);
        draw_text("->", x + 50, y + 94);

        Tile::Uncovered.draw(x + 66 + 10, y + 85, TILE_SIZE, false, 1);
        Tile::Flagged.draw(x + 66 + 20, y + 85, TILE_SIZE, true, 0);
        Tile::Uncovered.draw(x + 66 + 30, y + 85, TILE_SIZE, false, 2);

        Tile::Uncovered.draw(x + 66 + 10, y + 95, TILE_SIZE, false, 2);
        Tile::Uncovered.draw(x + 66 + 20, y + 95, TILE_SIZE, false, 4);
        Tile::Flagged.draw(x + 66 + 30, y + 95, TILE_SIZE, true, 0);

        Tile::Flagged.draw(x + 66 + 10, y + 105, TILE_SIZE, true, 0);
        Tile::Uncovered.draw(x + 66 + 20, y + 105, TILE_SIZE, false, 3);
        Tile::Flagged.draw(x + 66 + 30, y + 105, TILE_SIZE, true, 0);
    },
    |x, y| {
        DrawColors.set(3);
        draw_text(
            "Make sure to
flag the correct
tiles!",
            x + 3,
            y + 3,
        );

        Tile::Covered.draw(x + 10, y + 85, TILE_SIZE, false, 0);
        Tile::Covered.draw(x + 20, y + 85, TILE_SIZE, true, 0);
        Tile::Flagged.draw(x + 30, y + 85, TILE_SIZE, false, 0);

        Tile::Covered.draw(x + 10, y + 95, TILE_SIZE, false, 0);
        Tile::Uncovered.draw(x + 20, y + 95, TILE_SIZE, false, 4);
        Tile::Flagged.draw(x + 30, y + 95, TILE_SIZE, true, 0);

        Tile::Flagged.draw(x + 10, y + 105, TILE_SIZE, true, 0);
        Tile::Covered.draw(x + 20, y + 105, TILE_SIZE, false, 0);
        Tile::Flagged.draw(x + 30, y + 105, TILE_SIZE, true, 0);

        DrawColors.set(3);
        draw_text("->", x + 50, y + 94);

        Tile::Uncovered.draw(x + 66 + 10, y + 85, TILE_SIZE, false, 1);
        Tile::Uncovered.draw(x + 66 + 20, y + 85, TILE_SIZE, true, 0);
        Tile::Flagged.draw(x + 66 + 30, y + 85, TILE_SIZE, false, 2);

        Tile::Uncovered.draw(x + 66 + 10, y + 95, TILE_SIZE, false, 2);
        Tile::Uncovered.draw(x + 66 + 20, y + 95, TILE_SIZE, false, 4);
        Tile::Flagged.draw(x + 66 + 30, y + 95, TILE_SIZE, true, 0);

        Tile::Flagged.draw(x + 66 + 10, y + 105, TILE_SIZE, true, 0);
        Tile::Uncovered.draw(x + 66 + 20, y + 105, TILE_SIZE, false, 3);
        Tile::Flagged.draw(x + 66 + 30, y + 105, TILE_SIZE, true, 0);
    },
    |x, y| {
        DrawColors.set(3);
        draw_text(
            "Simultaneous
left and right
clicks on
uncovered tiles
uncovers nearby
tiles.",
            x + 3,
            y + 3,
        );
        draw_text(
            "Try doing that
after you placed
more flags.",
            x + 3,
            y + 55,
        );

        Tile::Covered.draw(x + 10, y + 85, TILE_SIZE, false, 0);
        Tile::Flagged.draw(x + 20, y + 85, TILE_SIZE, true, 0);
        Tile::Uncovered.draw(x + 30, y + 85, TILE_SIZE, false, 2);

        Tile::Covered.draw(x + 10, y + 95, TILE_SIZE, false, 0);
        Tile::Uncovered.draw(x + 20, y + 95, TILE_SIZE, false, 4);
        Tile::Flagged.draw(x + 30, y + 95, TILE_SIZE, true, 0);

        Tile::Flagged.draw(x + 10, y + 105, TILE_SIZE, true, 0);
        Tile::Uncovered.draw(x + 20, y + 105, TILE_SIZE, false, 3);
        Tile::Flagged.draw(x + 30, y + 105, TILE_SIZE, true, 0);

        DrawColors.set(3);
        draw_text("->", x + 50, y + 94);

        Tile::Uncovered.draw(x + 66 + 10, y + 85, TILE_SIZE, false, 1);
        Tile::Flagged.draw(x + 66 + 20, y + 85, TILE_SIZE, true, 0);
        Tile::Uncovered.draw(x + 66 + 30, y + 85, TILE_SIZE, false, 2);

        Tile::Uncovered.draw(x + 66 + 10, y + 95, TILE_SIZE, false, 2);
        Tile::Uncovered.draw(x + 66 + 20, y + 95, TILE_SIZE, false, 4);
        Tile::Flagged.draw(x + 66 + 30, y + 95, TILE_SIZE, true, 0);

        Tile::Flagged.draw(x + 66 + 10, y + 105, TILE_SIZE, true, 0);
        Tile::Uncovered.draw(x + 66 + 20, y + 105, TILE_SIZE, false, 3);
        Tile::Flagged.draw(x + 66 + 30, y + 105, TILE_SIZE, true, 0);
    },
    |x, y| {
        DrawColors.set(3);
        draw_text(
            "The goal of the
game is to
uncover all
tiles.",
            x + 3,
            y + 3,
        );
        draw_text("Except mines ;)", x + 3, y + 40);
    },
];
