#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::digital::InputPin;
use microbit::{Board, display::blocking::Display, hal::Timer};
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

const HEART_BIG: [[u8; 5]; 5] = [
        [0, 1, 0, 1 ,0],
        [1, 1, 1, 1 ,1],
        [1, 1, 1, 1 ,1],
        [0, 1, 1, 1 ,0],
        [0, 0, 1, 0 ,0],
    ];

const HEART_SMALL: [[u8; 5]; 5] = [
        [0, 0, 0, 0 ,0],
        [0, 1, 0, 1, 0],
        [0, 1, 1, 1, 0],
        [0, 0, 1, 0, 0],
        [0, 0, 0, 0 ,0],
    ];

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    let mut button_a = board.buttons.button_a;
    let mut button_b = board.buttons.button_b; 

    let mut timer_delay = 500;

    loop {
        if button_a.is_low().unwrap() {
            if timer_delay > 100 {
                timer_delay -= 100;
                rprintln!("{}", timer_delay);
            }
        }

        if button_b.is_low().unwrap() {
            if timer_delay < 1000 {
                timer_delay += 100;
                rprintln!("{}", timer_delay);
            }
        }

        display.show(
            &mut timer,
            HEART_BIG,
            timer_delay
        );
        display.show(
            &mut timer,
            HEART_SMALL,
            timer_delay
        );
    }
}