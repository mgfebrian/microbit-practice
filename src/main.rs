#![no_main]
#![no_std]

use cortex_m_rt::entry;
use microbit::{Board, display::blocking::Display, hal::Timer};
use panic_rtt_target as _;
use rtt_target::rtt_init_print;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let heart_big = [
        [0, 1, 0, 1 ,0],
        [1, 1, 1, 1 ,1],
        [1, 1, 1, 1 ,1],
        [0, 1, 1, 1 ,0],
        [0, 0, 1, 0 ,0],
    ];

    let heart_small = [
        [0, 0, 0, 0 ,0],
        [0, 1, 0, 1, 0],
        [0, 1, 1, 1, 0],
        [0, 0, 1, 0, 0],
        [0, 0, 0, 0 ,0],
    ];

    loop {
        display.show(
            &mut timer,
            heart_big,
            500
        );
        display.show(
            &mut timer,
            heart_small,
            500
        );
    }
}