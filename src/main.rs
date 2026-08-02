#![no_main]
#![no_std]

use cortex_m_rt::entry;
use microbit::{Board, hal::uarte};
use panic_rtt_target as _;
use rtt_target::{rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();

    let mut serial  = uarte::Uarte::new(
        board.UARTE0, 
        board.uart.into(), 
        uarte::Parity::EXCLUDED, 
        uarte::Baudrate::BAUD115200
    );

    
    loop {
        let mut buffer = [0u8; 1];
        if serial.read(&mut buffer).is_ok() {
            serial.write(&buffer).unwrap();
        }
    }
}