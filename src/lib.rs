#[cfg(feature = "buddy-alloc")]
mod alloc;

mod assets;

mod graphics;

mod mouse;
use mouse::Mouse;

mod wasm4;
use wasm4::*;

#[rustfmt::skip]
const SMILEY: [u8; 8] = [
    0b11000011,
    0b10000001,
    0b00100100,
    0b00100100,
    0b00000000,
    0b00100100,
    0b10011001,
    0b11000011,
];

#[no_mangle]
fn update() {
    unsafe { *DRAW_COLORS = 2 }
    text("Hello from Rust!", 10, 10);

    let gamepad = unsafe { *GAMEPAD1 };
    if gamepad & BUTTON_1 != 0 {
        unsafe { *DRAW_COLORS = 4 }
    }

    blit(&SMILEY, 76, 76, 8, 8, BLIT_1BPP);
    text("Press X to blink", 16, 90);

    let mouse_position = Mouse.coordinates();
    vline(mouse_position.0 as i32, 0, SCREEN_SIZE);
    hline(0, mouse_position.1 as i32, SCREEN_SIZE);
    if Mouse.left_clicked() {
        trace("Left clicked");
    }
    if Mouse.right_clicked() {
        trace("Right clicked");
    }
    if Mouse.middle_clicked() {
        trace("Middle clicked");
    }
    Mouse.update();
}
