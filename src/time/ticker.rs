// Copyright Claudio Mattera 2021.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Engine ticker

/// A ticker to keep track of the current frame
///
/// WASM-4 engine runs at 60 frames per second.
/// This object can be used to find out what is the current frame number.
pub struct Ticker;

static mut TICKER_COUNTER: u8 = 0;

impl Ticker {
    /// Update the ticker
    ///
    /// This function must be called at the end of each frame.
    pub fn update(&mut self) {
        unsafe {
            TICKER_COUNTER += 1;
            TICKER_COUNTER %= 60;
        }
    }

    /// Get the current frame number
    pub fn get(&self) -> u8 {
        unsafe { TICKER_COUNTER }
    }
}
