// Copyright Claudio Mattera 2021.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::Ticker;

pub struct Timer(u32);

impl Timer {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn update(&mut self) {
        if Ticker.get() == 0 {
            self.0 += 1;
        }
    }

    pub fn get(&self) -> u32 {
        self.0
    }
}
