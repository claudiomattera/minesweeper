// Copyright Claudio Mattera 2021.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub struct Ticker;

pub static mut TICKER_COUNTER: u8 = 0;

impl Ticker {
    pub fn update(&mut self) {
        unsafe {
            TICKER_COUNTER += 1;
            TICKER_COUNTER %= 60;
        }
    }

    pub fn get(&self) -> u8 {
        unsafe { TICKER_COUNTER }
    }
}
