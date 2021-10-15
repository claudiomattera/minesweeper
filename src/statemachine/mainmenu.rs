// Copyright Claudio Mattera 2021.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::graphics::{draw_rect, draw_text, DrawColors};
use crate::mouse::Mouse;

use super::{PreGameState, State, Transition};

#[derive(Clone)]
pub struct MainMenuState {}

const WIDTH: u32 = 160 - 6;
const HEIGHT: u32 = 14;

impl MainMenuState {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {}
    }

    pub fn draw(&self) {
        let title = "MINESWEEPER";
        DrawColors.set(0x02);
        draw_text(title, 1 + (160 - 8 * title.len() as i32) / 2, 1);
        self.draw_menu_entry(0, "Start an easy game");
        self.draw_menu_entry(1, "Start a medium game");
        self.draw_menu_entry(2, "Start a hard game");
    }

    pub fn update(self, mouse: &Mouse) -> Transition {
        if mouse.left_clicked() {
            let (mouse_x, mouse_y) = Mouse.coordinates();
            for index in 0..=2 {
                if self.is_mouse_inside_entry(index, mouse_x, mouse_y) {
                    let mines_count = self.mines_count_for_difficulty(index);
                    return Transition::Replace(State::PreGame(PreGameState::new(mines_count)))
                }
            }
        }

        Transition::Replace(State::MainMenu(self))
    }

    fn entry_to_coordinates(&self, index: usize) -> (i32, i32) {
        (3, 15 + index as i32 * 15)
    }

    fn is_mouse_inside_entry(&self, index: usize, mouse_x: i16, mouse_y: i16) -> bool {
        let (x, y) = self.entry_to_coordinates(index);
        x <= mouse_x as i32 && mouse_x as i32 <= x + WIDTH as i32
            && y <= mouse_y as i32 && mouse_y as i32 <= y + HEIGHT as i32
    }

    fn draw_menu_entry(&self, index: usize, text: &str) {
        let (x, y) = self.entry_to_coordinates(index);

        let (mouse_x, mouse_y) = Mouse.coordinates();
        let is_highlighted = self.is_mouse_inside_entry(index, mouse_x, mouse_y);

        if is_highlighted {
            DrawColors.set(0x02);
        } else {
            DrawColors.set(0x01);
        }
        draw_rect(x, y, WIDTH, HEIGHT);

        if is_highlighted {
            DrawColors.set(0x01);
        } else {
            DrawColors.set(0x03);
        }
        draw_text(text, x + 1, y + 3);
    }

    fn mines_count_for_difficulty(&self, index: usize) -> usize {
        match index {
            0 => 10,
            1 => 30,
            2 => 60,
            _ => unreachable!(),
        }
    }
}
