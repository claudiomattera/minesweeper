// Copyright Claudio Mattera 2021.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Binary assets

use crate::graphics::Sprite;

include!(concat!(env!("OUT_DIR"), "/sprites.rs"));

pub const FONT_SPRITE: Sprite = Sprite::new(
    sprites::FONT_WIDTH,
    sprites::FONT_HEIGHT,
    sprites::FONT_FLAGS,
    &sprites::FONT,
);
