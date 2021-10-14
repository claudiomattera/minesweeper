// Copyright Claudio Mattera 2021.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use once_cell::unsync::Lazy;

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

mod wasm4;
use wasm4::*;

static mut MAP: Lazy<Map<10>> = Lazy::new(|| {
    let seed = 0;
    debug!("Creating map with seed {}", seed);
    let width = 16;
    let height = 14;
    let map = Map::<10>::from_random_seed(seed, width, height, (0, 20));
    map
});

#[no_mangle]
fn start() {
}

#[no_mangle]
fn update() {
    let map = unsafe { &mut MAP };
    map.draw();

    if Mouse.left_clicked() {
        let (x, y) = Mouse.coordinates();
        map.handle_left_click(x, y);
    }
    if Mouse.right_clicked() {
        let (x, y) = Mouse.coordinates();
        map.handle_right_click(x, y);
    }

    let pos = Mouse.coordinates();
    let draw_colors = DrawColors::new();
    draw_colors.set(4);
    vline(pos.0 as i32, pos.1 as i32 - 1, 3);
    hline(pos.0 as i32 - 1, pos.1 as i32, 3);
    Mouse.update();
}
