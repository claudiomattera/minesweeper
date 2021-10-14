// Copyright Claudio Mattera 2021.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use once_cell::unsync::Lazy;

use crate::graphics::DrawColors;

use crate::mouse::Mouse;

use crate::Map;

use crate::Timer;

use crate::wasm4::text;

use super::{State, Transition};

static mut TIMER: Lazy<Timer> = Lazy::new(Timer::new);

static mut MAP: Lazy<Map<50>> = Lazy::new(|| {
    let width = 16;
    let height = 14;
    Map::new(width, height, (0, 20))
});


#[derive(Clone)]
pub struct InGameState {
}

impl InGameState {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn draw(&self) {
        let map = unsafe { &mut MAP };
        let timer = unsafe { &mut TIMER };

        // Draw map
        map.draw();

        // Draw remaining mines count
        let remaining_mines = map.count_remaining_mines();
        let s = format!("Mines:{:02}", remaining_mines);
        DrawColors.set(0x03);
        text(&s, 160 - 64, 2);

        // Draw elapsed time
        let s = format!("Time:{:3}", timer.get());
        text(s, 2, 2);

        // Draw game status
        if map.has_stepped_on_mine() {
            text("GAME OVER!!!", 2, 10);
        }

        if map.has_found_all_mines() {
            text("GAME WON!!!", 2, 10);
        }
    }

    pub fn update(self, _mouse: &Mouse) -> Transition {
        let map = unsafe { &mut MAP };
        let timer = unsafe { &mut TIMER };

        if !map.has_stepped_on_mine() && !map.has_found_all_mines() {
            if Mouse.left_clicked() {
                let (x, y) = Mouse.coordinates();
                map.handle_left_click(x, y);
            }
            if Mouse.right_clicked() {
                let (x, y) = Mouse.coordinates();
                map.handle_right_click(x, y);
            }
        }

        if map.has_started() && !map.has_stepped_on_mine() && !map.has_found_all_mines() {
            timer.update();
        }

        Transition::Replace(State::InGame(self))
    }
}
