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
use graphics::DrawColors;

mod map;
use map::Map;

mod mouse;
use mouse::Mouse;

mod ticker;
use ticker::Ticker;

mod statemachine;
use statemachine::STATE_MACHINE;

mod timer;
use timer::Timer;

mod wasm4;
use wasm4::*;

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
}

fn draw_mouse_pointer() {
    let pos = Mouse.coordinates();
    DrawColors.set(4);
    vline(pos.0 as i32, pos.1 as i32 - 1, 3);
    hline(pos.0 as i32 - 1, pos.1 as i32, 3);
}
