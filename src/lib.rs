// Copyright Claudio Mattera 2021.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! A game engine library implementing Minesweeper based on WASM-4 engine

#[cfg(feature = "buddy-alloc")]
mod alloc;

pub mod assets;
mod debug;
pub mod game;
pub mod graphics;
pub mod input;
pub mod interface;
pub mod sound;
pub mod statemachine;
pub mod time;
pub mod wasm4;
