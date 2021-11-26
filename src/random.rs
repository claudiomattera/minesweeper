// Copyright Claudio Mattera 2021.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Random primitives

use once_cell::unsync::Lazy;

use rand_core::SeedableRng;
use rand_xorshift::XorShiftRng;

use crate::time::BigTicker;

use crate::debug;

/// The game random generator
///
/// The generator is lazily initialized, i.e. it is constructed on the first
/// time it is accessed.
/// The current number of elapsed frames is used as random seed, so it is best
/// to access it the first time after user interaction.
pub static mut RNG: Lazy<XorShiftRng> = Lazy::new(|| {
    let frames_from_beginning: u64 = BigTicker.get();
    let seed = frames_from_beginning;

    debug!("Initializing random generator with seed {}", seed);
    XorShiftRng::seed_from_u64(seed)
});
