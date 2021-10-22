// Copyright Claudio Mattera 2021.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Difficulty structures and functions

/// Represent the game difficulty
#[derive(Clone, Copy)]
pub enum Difficulty {
    /// An easy game with 10 mines
    Easy,

    /// A medium game with 30 mines
    Medium,

    /// A hard game with 50 mines
    Hard,
}

impl Difficulty {
    /// Return the mines count for this difficulty
    pub fn mines_count(&self) -> usize {
        match self {
            Difficulty::Easy => 10,
            Difficulty::Medium => 30,
            Difficulty::Hard => 50,
        }
    }
}

impl AsRef<str> for Difficulty {
    fn as_ref(&self) -> &str {
        match self {
            Difficulty::Easy => "Easy",
            Difficulty::Medium => "Medium",
            Difficulty::Hard => "Hard",
        }
    }
}
