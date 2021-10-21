// Copyright Claudio Mattera 2021.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Mouse

use crate::wasm4::{MOUSE_BUTTONS, MOUSE_X, MOUSE_Y};

/// A mouse
#[derive(Clone, Copy)]
pub struct Mouse;

static mut PREVIOUS_BUTTONS: u8 = 0;

impl Mouse {
    /// Update previous status of buttons
    ///
    /// The previous status of mouse buttons is used to detect clicks.
    /// A click happens when the previous status was down and the current
    /// status is up.
    pub fn update(&self) {
        unsafe { PREVIOUS_BUTTONS = *MOUSE_BUTTONS }
    }

    /// Get the coordinates
    pub fn coordinates(&self) -> (i16, i16) {
        unsafe { (*MOUSE_X, *MOUSE_Y) }
    }

    /// Get the X coordinate
    pub fn x(&self) -> i16 {
        unsafe { *MOUSE_X }
    }

    /// Get the Y coordinate
    pub fn y(&self) -> i16 {
        unsafe { *MOUSE_Y }
    }

    /// Check whether left button is pressed
    pub fn left_pressed(&self) -> bool {
        left_pressed(unsafe { *MOUSE_BUTTONS })
    }

    /// Check whether left button is pressed
    pub fn right_pressed(&self) -> bool {
        right_pressed(unsafe { *MOUSE_BUTTONS })
    }

    /// Check whether left button is pressed
    pub fn middle_pressed(&self) -> bool {
        middle_pressed(unsafe { *MOUSE_BUTTONS })
    }

    /// Check whether left button was clicked
    pub fn left_clicked(&self) -> bool {
        let current_unpressed = !left_pressed(unsafe { *MOUSE_BUTTONS });
        let previously_pressed = left_pressed(unsafe { PREVIOUS_BUTTONS });
        current_unpressed && previously_pressed
    }

    /// Check whether right button was clicked
    pub fn right_clicked(&self) -> bool {
        let current_unpressed = !right_pressed(unsafe { *MOUSE_BUTTONS });
        let previously_pressed = right_pressed(unsafe { PREVIOUS_BUTTONS });
        current_unpressed && previously_pressed
    }

    /// Check whether middle button was clicked
    pub fn middle_clicked(&self) -> bool {
        let current_unpressed = !middle_pressed(unsafe { *MOUSE_BUTTONS });
        let previously_pressed = middle_pressed(unsafe { PREVIOUS_BUTTONS });
        current_unpressed && previously_pressed
    }
}

fn left_pressed(value: u8) -> bool {
    (value & 0b001) != 0
}

fn right_pressed(value: u8) -> bool {
    (value & 0b010) != 0
}

fn middle_pressed(value: u8) -> bool {
    (value & 0b100) != 0
}
