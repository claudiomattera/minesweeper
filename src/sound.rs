// Copyright Claudio Mattera 2021.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Sound primitives

#[cfg(feature = "sound")]
use crate::wasm4::tone;

/// Play the game-over sound
#[cfg(feature = "sound")]
pub fn play_game_over_sound() {
    let frequency1 = 360;
    let frequency2 = 150;
    let attack = 0;
    let decay = 0;
    let sustain = 18;
    let release = 0;
    let volume = 100;
    let channel = 0;
    let mode = 0;
    tone(
        frequency1 | (frequency2 << 16),
        (attack << 24) | (decay << 16) | sustain | (release << 8),
        volume,
        channel | (mode << 2),
    );
}

#[cfg(not(feature = "sound"))]
pub fn play_game_over_sound() {}
