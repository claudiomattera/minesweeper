// Copyright Claudio Mattera 2021.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Functions for loading and saving high scores

use crate::debug;
use crate::wasm4::{diskr, diskw};

use super::Difficulty;

#[derive(Clone)]
pub struct HighScores {
    easy: Option<u16>,
    medium: Option<u16>,
    hard: Option<u16>,
}

impl HighScores {
    /// Load the high scores
    ///
    /// The result is a list of optional times in seconds, one for each difficulty.
    pub fn load() -> Self {
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

        Self {
            easy: highscores[0],
            medium: highscores[1],
            hard: highscores[2],
        }
    }

    /// Get the highscore for a difficulty
    pub fn get(&self, difficulty: Difficulty) -> Option<u16> {
        match difficulty {
            Difficulty::Easy => self.easy,
            Difficulty::Medium => self.medium,
            Difficulty::Hard => self.hard,
        }
    }

    /// Set a new highscore
    ///
    /// The score is set only if better than the current one, i.e. if the time
    /// is smaller.
    pub fn set(&mut self, difficulty: Difficulty, time: u16) {
        match difficulty {
            Difficulty::Easy => {
                let new_time = self
                    .easy
                    .map_or(time, |old_time| std::cmp::min(old_time, time));
                self.easy = Some(new_time);
            }
            Difficulty::Medium => {
                let new_time = self
                    .medium
                    .map_or(time, |old_time| std::cmp::min(old_time, time));
                self.medium = Some(new_time);
            }
            Difficulty::Hard => {
                let new_time = self
                    .hard
                    .map_or(time, |old_time| std::cmp::min(old_time, time));
                self.hard = Some(new_time);
            }
        }
    }

    /// Save highscores
    pub fn save(&self) {
        let mut buffer: [u8; 3 * 3] = [0; 3 * 3];

        if let Some(time) = self.easy {
            buffer[0] = 0xff;
            buffer[1] = time as u8;
            buffer[2] = (time >> 8) as u8;
        }
        if let Some(time) = self.medium {
            buffer[3] = 0xff;
            buffer[4] = time as u8;
            buffer[5] = (time >> 8) as u8;
        }
        if let Some(time) = self.hard {
            buffer[6] = 0xff;
            buffer[7] = time as u8;
            buffer[8] = (time >> 8) as u8;
        }

        let bytes_written = unsafe { diskw(buffer.as_mut_ptr(), buffer.len() as u32) };
        if bytes_written != buffer.len() as u32 {
            debug!("Failed to save highscores");
        }
    }
}
