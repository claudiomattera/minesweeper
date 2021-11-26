// Copyright Claudio Mattera 2021.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Game main loop

use crate::graphics::{draw_horizontal_line, draw_vertical_line, DrawColors};
use crate::input::Mouse;
use crate::statemachine::STATE_MACHINE;
use crate::time::{BigTicker, Ticker};

#[no_mangle]
fn start() {}

#[no_mangle]
fn update() {
    let state_machine = unsafe { &mut STATE_MACHINE };

    state_machine.draw();
    state_machine.update(&Mouse);

    draw_mouse_pointer();

    Mouse.update();
    Ticker.update();
    BigTicker.update();
}

fn draw_mouse_pointer() {
    let pos = Mouse.coordinates();
    DrawColors.set(4);
    draw_vertical_line(pos.0 as i32, pos.1 as i32 - 1, 3);
    draw_horizontal_line(pos.0 as i32 - 1, pos.1 as i32, 3);
}
