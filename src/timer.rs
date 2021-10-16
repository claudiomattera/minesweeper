// Copyright Claudio Mattera 2021.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Timer data types and function

use crate::Ticker;

/// A timer to keep track of elapsed time
#[derive(Clone)]
pub struct Timer(u32);

impl Timer {
    /// Create a new timer
    pub fn new() -> Self {
        Self(0)
    }

    /// Update the timer
    ///
    /// This function must be called at each frame.
    pub fn update(&mut self) {
        if Ticker.get() == 0 {
            self.0 += 1;
        }
    }

    /// Return the time since the time started in seconds
    pub fn get(&self) -> u32 {
        self.0
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}
