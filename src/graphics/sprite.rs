// Copyright Claudio Mattera 2021.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Sprite functions and data structures

use crate::wasm4::{blit, blit_sub};

/// A sprite
pub struct Sprite {
    pub width: u32,
    pub height: u32,
    pub flags: u32,
    pub data: &'static [u8],
}

impl Sprite {
    /// Create a new sprite
    pub fn new(width: u32, height: u32, flags: u32, data: &'static [u8]) -> Self {
        Self {
            width,
            height,
            flags,
            data,
        }
    }

    /// Draw the sprite to the screen
    ///
    /// The sprite is drawn at the point `x`, `y`.
    pub fn blit(&self, x: i32, y: i32) {
        blit(self.data, x, y, self.width, self.height, self.flags);
    }

    /// Draw a region of the sprite to the screen
    ///
    /// The region is drawn at the point `x`, `y`.
    /// The region starts at the point `src_x`, `src_y` and has width `width`
    /// and height `height`.
    pub fn blit_sub(&self, x: i32, y: i32, width: u32, height: u32, src_x: u32, src_y: u32) {
        blit_sub(
            self.data, x, y, width, height, src_x, src_y, self.width, self.flags,
        );
    }
}
