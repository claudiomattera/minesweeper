// Copyright Claudio Mattera 2021.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![no_std]

use core::arch::wasm32;
use core::panic::PanicInfo;

#[cfg(feature = "buddy-alloc")]
mod alloc;

mod assets;

mod debug;

mod graphics;
use graphics::DrawColors;

mod map;

mod mouse;
use mouse::Mouse;

mod statemachine;
use statemachine::{Machine, STATE_MACHINE};

mod wasm4;
use wasm4::*;

#[no_mangle]
fn update() {
    let state_machine: &mut Machine = unsafe { &mut STATE_MACHINE };

    state_machine.draw();

    state_machine.update(Some(&Mouse));

    let pos = Mouse.coordinates();
    let draw_colors = DrawColors::new();
    draw_colors.set(4);
    vline(pos.0 as i32, pos.1 as i32 - 1, 3);
    hline(pos.0 as i32 - 1, pos.1 as i32, 3);
    Mouse.update();
}

#[panic_handler]
fn panic_handler(panic_info: &PanicInfo<'_>) -> ! {
    match panic_info.payload().downcast_ref::<&str>() {
        Some(cause) => debug!("Panicked due to: {}", cause),
        None => debug!("Panicked due to unknown cause"),
    }

    unsafe { wasm32::unreachable() }
}
