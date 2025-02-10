#![no_std]
#![no_main]

use panic_halt as _;

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use microbit::{board::Board, display::blocking::Display, hal::Timer};

#[entry]

fn main() -> ! {


    fn merge_letters(
        left: &[[u8; 5]; 5], 
        right: &[[u8; 5]; 5], 
        shift: usize
    ) -> [[u8; 5]; 5] {
        let mut new_matrix = [[0; 5]; 5];

        for row in 0..5 {
            for col in 0..5 {
                if col < (5 - shift) {
                    new_matrix[row][col] = left[row][col + shift]; // Shift old letter out
                } else {
                    new_matrix[row][col] = right[row][col - (5 - shift)]; // Shift new letter in
                }
            }
        }
        
        new_matrix
    }

    const TRANSITION_STEPS: usize = 5;

    fn generate_word_animation(
        current: [[u8; 5]; 5], 
        next: [[u8; 5]; 5]
    ) -> [[[u8; 5]; 5]; TRANSITION_STEPS] {
        let mut frames = [[[0; 5]; 5]; TRANSITION_STEPS];

        for shift in 0..TRANSITION_STEPS {
            frames[shift] = merge_letters(&current, &next, shift);
        }

        frames
    }

    if let Some(board) = Board::take() {
        let mut timer = Timer::new(board.TIMER0);
        let mut display = Display::new(board.display_pins);

        #[allow(non_snake_case)]
        let m_frame1 = [
            [0, 1, 1, 1, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 1, 1, 1, 0],
        ];

        let m_frame2= [
            [0, 1, 0, 1, 0],
            [1, 0, 1, 0, 1],
            [1, 0, 0, 0, 1],
            [0, 1, 0, 1, 0],
            [0, 0, 1, 0, 0],
        ];


        #[allow(non_snake_case)]
        let m_frame3= [
            [0, 1, 0, 0, 0],
            [0, 1, 1, 0, 1],
            [0, 1, 0, 1, 0],
            [0, 1, 0, 0, 0],
            [0, 1, 0, 0, 0],
        ];


        #[allow(non_snake_case)]
        let m_frame4= [
            [1, 0, 1, 0, 0],
            [1, 0, 0, 0, 0],
            [1, 0, 1, 0, 1],
            [1, 0, 1, 0, 1],
            [1, 0, 1, 0, 1],
        ];

        #[allow(non_snake_case)]
        let m_frame5= [
            [0, 0, 0, 0, 0],
            [1, 0, 0, 0, 0],
            [0, 1, 0, 0, 0],
            [1, 1, 0, 0, 0],
            [0, 1, 0, 0, 0],
        ];

        let word = [&m_frame1, &m_frame2, &m_frame3, &m_frame4, &m_frame5];

        loop {
            for w in 0..word.len() - 1 {
                    let frames = generate_word_animation(*word[w], *word[w + 1]);

                    for frame in &frames {
                        display.show(&mut timer, *frame, 100);
                    }
                }
            display.clear();
            timer.delay_ms(250_u32);
        }
    }

    panic!("End");
}
