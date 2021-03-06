// Copyright Claudio Mattera 2021.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Interface primitives

use crate::fmt::format_number;
use crate::graphics::{draw_rect, draw_text, DrawColors};

/// Draw a message box
pub fn draw_message_box(text: &str, x: i32, y: i32) {
    let height = 15 + 8 * text.chars().filter(|c| *c == '\n').count() as u32;
    let width = 160 - 2 * x as u32;

    draw_box(x, y, width, height);

    DrawColors.set(0x03);
    draw_text(text, x + 4, y + 4);
}

/// Draw a box
pub fn draw_box(x: i32, y: i32, width: u32, height: u32) {
    DrawColors.set(0x44);
    draw_rect(x + 3, y + 3, width, height);
    DrawColors.set(0x21);
    draw_rect(x, y, width, height);
}

/// Draw the remaining mines count
pub fn draw_remaining_mines_count(remaining_mines: usize, x: i32, y: i32) {
    let x = x - 2;
    let y = y + 2;

    let s = "Mines:".to_owned() + &format_number(remaining_mines as u32, Some(2));
    DrawColors.set(0x03);
    draw_text(s, x, y);
}

/// Draw the elapsed time
pub fn draw_elapsed_time(elapsed_time: u32, x: i32, y: i32) {
    let x = x + 2;
    let y = y + 2;

    let s = "Time:".to_owned() + &format_number(elapsed_time as u32, Some(3));
    DrawColors.set(0x03);
    draw_text(s, x, y);
}
