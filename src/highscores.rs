// Copyright Claudio Mattera 2021.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Functions for loading and saving high scores

use crate::wasm4::{diskr, diskw};
use crate::debug;

/// Get the high scores
pub fn get_high_scores() -> Vec<(u32, u32)> {
    let mut highscores = Vec::new();

    let mut buffer: [u8; 32] = [0; 32];
    let bytes_read = unsafe {
        diskr(&mut buffer as *mut u8, buffer.len() as u32)
    };
    let n = bytes_read as usize / 8;
    for i in 0..n {
        let difficulty = (buffer[i * 8 + 0] as u32) << 3 | (buffer[i * 8 + 1] as u32) << 2 | (buffer[i * 8 + 2] as u32) << 1 | (buffer[i * 8 + 3] as u32);
        let time = (buffer[i * 8 + 4] as u32) << 3 | (buffer[i * 8 + 5] as u32) << 2 | (buffer[i * 8 + 6] as u32) << 1 | (buffer[i * 8 + 7] as u32);
        highscores.push((difficulty, time));
    }

    highscores
}

/// Get the high scores
pub fn save_high_score(difficulty: u32, time: u32) {
    let mut highscores = get_high_scores();
    highscores.push((difficulty, time));
    highscores.sort();
    while highscores.len() > 4 {
        highscores.pop();
    }

    let mut buffer: Vec<u8> = Vec::new();

    for (difficulty, time) in &highscores {
        buffer.push((difficulty >> 3) as u8);
        buffer.push((difficulty >> 2) as u8);
        buffer.push((difficulty >> 1) as u8);
        buffer.push((difficulty >> 0) as u8);
        buffer.push((time >> 3) as u8);
        buffer.push((time >> 2) as u8);
        buffer.push((time >> 1) as u8);
        buffer.push((time >> 0) as u8);
    }

    let bytes_written = unsafe { diskw(buffer.as_mut_ptr(), buffer.len() as u32) };
    if bytes_written != 8 * highscores.len() as u32 {
        debug!("Something wrong");
    }
}
