// Copyright Claudio Mattera 2021.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[cfg(feature = "buddy-alloc")]
mod alloc;

mod assets;

mod debug;

mod graphics;

mod mouse;
use mouse::Mouse;

mod statemachine;
use statemachine::{STATE_MACHINE, Machine};

mod wasm4;
use wasm4::*;

#[no_mangle]
fn update() {
    let state_machine: &mut Machine = unsafe { &mut STATE_MACHINE };

    state_machine.draw();

    state_machine.update(Some(&Mouse));

    let mouse_position = Mouse.coordinates();
    vline(mouse_position.0 as i32, 0, SCREEN_SIZE);
    hline(0, mouse_position.1 as i32, SCREEN_SIZE);
    if Mouse.left_clicked() {
        trace("Left clicked");
    }
    if Mouse.right_clicked() {
        trace("Right clicked");
    }
    if Mouse.middle_clicked() {
        trace("Middle clicked");
    }
    Mouse.update();
}
