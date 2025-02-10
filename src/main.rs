#![no_std]
#![no_main]

use panic_halt as _;

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use microbit::{board::Board, display::blocking::Display, hal::Timer};

#[entry]
fn main() -> ! {
    if let Some(board) = Board::take() {
        let mut timer = Timer::new(board.TIMER0);
        let mut display = Display::new(board.display_pins);

        #[allow(non_snake_case)]
        let letter_I = [
            [0, 1, 1, 1, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 1, 1, 1, 0],
        ];

        #[allow(non_snake_case)]
        let letter_I_1 = [
            [1, 1, 1, 0, 0],
            [0, 1, 0, 0, 1],
            [0, 1, 0, 0, 1],
            [0, 1, 0, 0, 0],
            [1, 1, 1, 0, 0],
        ];

        #[allow(non_snake_case)]
        let letter_I_2 = [
            [1, 1, 0, 0, 1],
            [1, 0, 0, 1, 0],
            [1, 0, 0, 1, 0],
            [1, 0, 0, 0, 1],
            [1, 1, 0, 0, 0],
        ];

        #[allow(non_snake_case)]
        let letter_I_3 = [
            [1, 0, 0, 1, 0],
            [0, 0, 1, 0, 1],
            [0, 0, 1, 0, 0],
            [0, 0, 0, 1, 0],
            [1, 0, 0, 0, 1],
        ];

        #[allow(non_snake_case)]
        let letter_I_4 = [
            [0, 0, 1, 0, 1],
            [0, 1, 0, 1, 0],
            [0, 1, 0, 0, 0],
            [0, 0, 1, 0, 1],
            [0, 0, 0, 1, 0],
        ];

        let heart = [
            [0, 1, 0, 1, 0],
            [1, 0, 1, 0, 1],
            [1, 0, 0, 0, 1],
            [0, 1, 0, 1, 0],
            [0, 0, 1, 0, 0],
        ];


        let heart_1 = [
            [1, 0, 1, 0, 0],
            [0, 1, 0, 1, 0],
            [0, 0, 0, 1, 0],
            [1, 0, 1, 0, 0],
            [0, 1, 0, 0, 0],
        ];

        let heart_2 = [
            [0, 1, 0, 0, 1],
            [1, 0, 1, 0, 1],
            [0, 0, 1, 0, 1],
            [0, 1, 0, 0, 1],
            [1, 0, 0, 0, 1],
        ];

        let heart_3 = [
            [1, 0, 0, 1, 0],
            [0, 1, 0, 1, 1],
            [0, 1, 0, 1, 0],
            [1, 0, 0, 1, 0],
            [0, 0, 0, 1, 0],
        ];

        let heart_4 = [
            [0, 0, 1, 0, 0],
            [1, 0, 1, 1, 0],
            [1, 0, 1, 0, 1],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
        ];

        let heart_5 = [
            [0, 1, 0, 0, 0],
            [0, 1, 1, 0, 1],
            [0, 1, 0, 1, 0],
            [0, 1, 0, 0, 0],
            [0, 1, 0, 0, 0],
        ];

        #[allow(non_snake_case)]
        let letter_M = [
            [1, 0, 0, 0, 1],
            [1, 1, 0, 1, 1],
            [1, 0, 1, 0, 1],
            [1, 0, 0, 0, 1],
            [1, 0, 0, 0, 1],
        ];

        #[allow(non_snake_case)]
        let letter_i = [
            [0, 0, 0, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
        ];

        #[allow(non_snake_case)]
        let letter_r = [
            [0, 0, 0, 0, 0],
            [0, 1, 0, 1, 0],
            [0, 1, 1, 0, 0],
            [0, 1, 0, 0, 0],
            [0, 1, 0, 0, 0],
        ];

        #[allow(non_snake_case)]
        let letter_a = [
            [0, 0, 0, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 1, 0, 1, 0],
            [0, 1, 1, 1, 0],
            [0, 1, 0, 1, 0],
        ];
        loop {
            display.show(&mut timer, letter_I, 200);
            display.show(&mut timer, letter_I_1, 200);
            display.show(&mut timer, letter_I_2, 200);
            display.show(&mut timer, letter_I_3, 200);
            display.show(&mut timer, letter_I_4, 200);
            display.show(&mut timer, heart, 200);
            display.show(&mut timer, heart_1, 200);
            display.show(&mut timer, heart_2, 200);
            display.show(&mut timer, heart_3, 200);
            display.show(&mut timer, heart_4, 200);
            display.show(&mut timer, heart_5, 200);
            display.show(&mut timer, letter_M, 200);
            display.show(&mut timer, letter_i, 1000);
            display.show(&mut timer, letter_r, 1000);
            display.show(&mut timer, letter_a, 1000);
            display.clear();
            timer.delay_ms(250_u32);
        }
    }

    panic!("End");
}
