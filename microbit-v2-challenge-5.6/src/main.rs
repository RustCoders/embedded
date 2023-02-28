#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::rtt_init_print;
use panic_rtt_target as _;

use microbit::{
    board::Board,
    display::blocking::Display,
    hal::{prelude::*, Timer},
};

// Return an array representing the lit LEDs.
fn get_array(current: usize) -> [[u8; 5]; 5] {
    // Which one is lit depends on where we are as 
    // we go around the outer edge.
    let indices = [
      (0,0),
      (0,1),
      (0,2),
      (0,3),
      (0,4),
      (1,4),
      (2,4),
      (3,4),
      (4,4),
      (4,3),
      (4,2),
      (4,1),
      (4,0),
      (3,0),
      (2,0),
      (1,0)
    ];
    let (x, y) = indices[current];

    // All off for now but...
    let mut leds = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];

    // ...set the lit one
    leds[x][y] = 1;
    leds
}

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
  
    // There are sixteen LEDs total, so we increment to 14, inclusive 
    // to get zero-based 15 index
    const MAX_TO_INC: usize = 14;
    let mut index: usize = 0;
    
    loop {

        let lights = get_array(index);
        // Show light_it_all for 1000ms
        display.show(&mut timer, lights, 100);
        // clear the display again
        display.clear();
        timer.delay_ms(20_u32);
        
        // Increment
        match index {
            0..=MAX_TO_INC => index = index +1,
            _ => index = 0,
        }
    }
}