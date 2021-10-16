// Copyright Claudio Mattera 2021.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Game components

mod difficulty;
pub use difficulty::Difficulty;

mod highscores;
pub use highscores::{get_high_scores, save_high_score};

mod map;
pub use map::Map;
