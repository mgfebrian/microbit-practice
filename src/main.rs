#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::{delay::DelayNs, digital::InputPin};
use microbit::{Board, hal::{Temp, Timer}};
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut button_a = board.buttons.button_a;
    let mut thermostat = Temp::new(board.TEMP); 

    rprintln!("Thermostat App Ready! Press Button A");
    loop {
        if button_a.is_low().unwrap() {
            rprintln!("Temperature: {}", thermostat.measure().to_num::<f32>());
        }
        timer.delay_ms(500);
    }
}