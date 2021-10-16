// Copyright Claudio Mattera 2021.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Functions for loading and saving high scores

use crate::debug;
use crate::difficulty::Difficulty;
use crate::wasm4::{diskr, diskw};

/// Get the high scores
///
/// The result is a list of optional times in seconds, one for each difficulty.
pub fn get_high_scores() -> [Option<u16>; 3] {
    let mut highscores = [None, None, None];

    let mut buffer: [u8; 3 * 3] = [0; 3 * 3];
    let bytes_read = unsafe { diskr(&mut buffer as *mut u8, buffer.len() as u32) };
    if bytes_read == buffer.len() as u32 {
        for i in 0..3 {
            let stored = buffer[i * 3] > 0;
            if stored {
                let time = (buffer[i * 3 + 1] as u16) | (buffer[i * 3 + 2] as u16) << 8;
                highscores[i] = Some(time);
            }
        }
    }

    highscores
}

/// Save a new highscore
///
/// The score is saved only if better than the current one, i.e. if the time is
/// smaller.
pub fn save_high_score(difficulty: Difficulty, time: u16) {
    let mut highscores = get_high_scores();

    let index = match difficulty {
        Difficulty::Easy => 0,
        Difficulty::Medium => 1,
        Difficulty::Hard => 2,
    };

    let new_time = highscores[index].map_or(time, |old_time| std::cmp::min(old_time, time));
    highscores[index] = Some(new_time);

    let mut buffer: [u8; 3 * 3] = [0; 3 * 3];

    for (i, time) in highscores.iter().enumerate() {
        if let Some(time) = time {
            buffer[i * 3] = 0xff;
            buffer[i * 3 + 1] = *time as u8;
            buffer[i * 3 + 2] = (time >> 8) as u8;
        }
    }

    let bytes_written = unsafe { diskw(buffer.as_mut_ptr(), buffer.len() as u32) };
    if bytes_written != 3 * highscores.len() as u32 {
        debug!("Failed to save highscores");
    }
}
