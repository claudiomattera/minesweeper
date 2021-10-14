// Copyright Claudio Mattera 2021.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Drawing colours control

use crate::wasm4::DRAW_COLORS;

#[derive(Clone, Copy)]
pub struct DrawColors;

impl DrawColors {
    pub fn set_one(&mut self, index: usize, value: u8) -> &mut Self {
        let drawcolorsimpl = unsafe { &mut *(DRAW_COLORS as *mut DrawColorsImpl) };
        if value < 5 {
            let value = value as u16;
            let mask_shift = (index) * 4;
            let mask: u16 = 0xf000 >> mask_shift;
            let rest = drawcolorsimpl.0 & !mask;
            let new_shift = (3 - index) * 4;
            let new = value << new_shift;
            drawcolorsimpl.0 = rest | new;
        }
        self
    }

    pub fn set(&mut self, value: u16) {
        let drawcolorsimpl = unsafe { &mut *(DRAW_COLORS as *mut DrawColorsImpl) };
        drawcolorsimpl.0 = value;
    }

    pub fn get(&self) -> u16 {
        let drawcolorsimpl = unsafe { &mut *(DRAW_COLORS as *mut DrawColorsImpl) };
        drawcolorsimpl.0
    }
}

#[derive(Clone, Copy)]
struct DrawColorsImpl(pub u16);

impl DrawColorsImpl {
    pub fn set_one(&mut self, index: usize, value: u8) -> &mut Self {
        if value < 5 {
            let value = value as u16;
            let mask_shift = (index) * 4;
            let mask: u16 = 0xf000 >> mask_shift;
            let rest = self.0 & !mask;
            let new_shift = (3 - index) * 4;
            let new = value << new_shift;
            self.0 = rest | new;
        }
        self
    }

    pub fn set(&mut self, value: u16) {
        self.0 = value;
    }

    pub fn get(&self) -> u16 {
        self.0
    }
}

// pub const A: &DrawColorsImpl = unsafe { &*(DRAW_COLORS as *const DrawColorsImpl) };

#[cfg(test)]
mod tests {
    use super::DrawColorsImpl;

    #[test]
    fn get() {
        let draw_colors = DrawColorsImpl(0x3102);
        let actual = draw_colors.get();
        let expected = 0x3102;
        assert_eq!(actual, expected);
    }

    #[test]
    fn set() {
        let mut draw_colors = DrawColorsImpl(0);
        draw_colors.set(0x3102);
        let actual = draw_colors.get();
        let expected = 0x3102;
        assert_eq!(actual, expected);
    }

    #[test]
    fn set_one_first() {
        let mut draw_colors = DrawColorsImpl(0);
        draw_colors.set_one(0, 3);
        let actual = draw_colors.get();
        let expected = 0x3000;
        assert_eq!(actual, expected);
    }

    #[test]
    fn set_one_second() {
        let mut draw_colors = DrawColorsImpl(0);
        draw_colors.set_one(1, 3);
        let actual = draw_colors.get();
        let expected = 0x0300;
        assert_eq!(actual, expected);
    }

    #[test]
    fn set_one_third() {
        let mut draw_colors = DrawColorsImpl(0);
        draw_colors.set_one(2, 3);
        let actual = draw_colors.get();
        let expected = 0x0030;
        assert_eq!(actual, expected);
    }

    #[test]
    fn set_one_fourth() {
        let mut draw_colors = DrawColorsImpl(0);
        draw_colors.set_one(3, 3);
        let actual = draw_colors.get();
        let expected = 0x0003;
        assert_eq!(actual, expected);
    }

    #[test]
    fn set_one_first_existing() {
        let mut draw_colors = DrawColorsImpl(0x1111);
        draw_colors.set_one(0, 3);
        let actual = draw_colors.get();
        let expected = 0x3111;
        assert_eq!(actual, expected);
    }

    #[test]
    fn set_one_second_existing() {
        let mut draw_colors = DrawColorsImpl(0x1111);
        draw_colors.set_one(1, 3);
        let actual = draw_colors.get();
        let expected = 0x1311;
        assert_eq!(actual, expected);
    }

    #[test]
    fn set_one_third_existing() {
        let mut draw_colors = DrawColorsImpl(0x1111);
        draw_colors.set_one(2, 3);
        let actual = draw_colors.get();
        let expected = 0x1131;
        assert_eq!(actual, expected);
    }

    #[test]
    fn set_one_fourth_existing() {
        let mut draw_colors = DrawColorsImpl(0x1111);
        draw_colors.set_one(3, 3);
        let actual = draw_colors.get();
        let expected = 0x1113;
        assert_eq!(actual, expected);
    }

    #[test]
    fn set_one_all() {
        let mut draw_colors = DrawColorsImpl(0);
        draw_colors.set_one(0, 3);
        draw_colors.set_one(1, 1);
        draw_colors.set_one(2, 0);
        draw_colors.set_one(3, 2);
        let actual = draw_colors.get();
        let expected = 0x3102;
        assert_eq!(actual, expected);
    }

    #[test]
    fn set_one_chained() {
        let mut draw_colors = DrawColorsImpl(0);
        draw_colors
            .set_one(0, 3)
            .set_one(1, 1)
            .set_one(2, 0)
            .set_one(3, 2);
        let actual = draw_colors.get();
        let expected = 0x3102;
        assert_eq!(actual, expected);
    }
}
