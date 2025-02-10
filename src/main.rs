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

        let m_frame3= [
            [0, 1, 0, 0, 1],
            [0, 1, 0, 0, 1],
            [0, 1, 0, 0, 1],
            [0, 1, 0, 0, 1],
            [0, 0, 1, 1, 0],
        ];


        #[allow(non_snake_case)]
        let m_frame4= [
            [0, 0, 1, 0, 0],
            [0, 0, 1, 1, 0],
            [0, 0, 1, 0, 1],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
        ];


        #[allow(non_snake_case)]
        let m_frame5= [
            [0, 1, 0, 0, 0],
            [1, 1, 0, 1, 0],
            [0, 1, 0, 0, 0],
            [0, 1, 0, 1, 0],
            [0, 1, 0, 1, 0],
        ];

        #[allow(non_snake_case)]
        let m_frame6= [
            [0, 0, 0, 0, 0],
            [1, 0, 1, 0, 0],
            [1, 1, 0, 0, 1],
            [1, 0, 0, 0, 1],
            [1, 0, 0, 0, 1],
        ];

        let m_frame7= [
            [0, 0, 0, 0, 0],
            [1, 0, 0, 0, 0],
            [0, 1, 0, 0, 0],
            [1, 1, 0, 0, 0],
            [0, 1, 0, 0, 0],
        ];

        // Happy Valentines <3!
        let h_frame1= [
            [1, 0, 1, 0, 0],
            [1, 0, 1, 0, 1],
            [1, 1, 1, 0, 1],
            [1, 0, 1, 0, 1],
            [1, 0, 1, 0, 1],
        ];

        let h_frame2= [
            [1, 0, 0, 1, 1],
            [0, 1, 0, 1, 0],
            [1, 1, 0, 1, 1],
            [0, 1, 0, 1, 0],
            [0, 1, 0, 1, 0],
        ];


        let h_frame3= [
            [0, 0, 1, 1, 0],
            [1, 0, 1, 0, 1],
            [0, 0, 1, 1, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
        ];
        let h_frame4= [
            [0, 1, 0, 1, 0],
            [0, 1, 0, 1, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
        ];
        let h_frame5= [
            [0, 1, 0, 1, 0],
            [0, 1, 0, 1, 0],
            [0, 1, 0, 1, 0],
            [0, 1, 0, 1, 0],
            [0, 0, 1, 0, 0],
        ];
        let h_frame6= [
            [0, 1, 0, 0, 1],
            [1, 0, 1, 0, 1],
            [1, 1, 1, 0, 1],
            [1, 0, 1, 0, 1],
            [1, 0, 1, 0, 1],
        ];
        let h_frame7= [
            [0, 0, 0, 1, 1],
            [0, 0, 0, 1, 0],
            [0, 0, 0, 1, 1],
            [0, 0, 0, 1, 0],
            [1, 1, 0, 1, 1],
        ];
        let h_frame8= [
            [1, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
            [1, 0, 1, 1, 0],
            [0, 0, 1, 0, 1],
            [1, 0, 1, 0, 0],
        ];
        let h_frame9= [
            [1, 0, 1, 1, 1],
            [1, 0, 0, 1, 0],
            [1, 0, 0, 1, 0],
            [1, 0, 0, 1, 0],
            [1, 0, 0, 1, 0],
        ];


        let h_frame10= [
            [0, 1, 0, 1, 0],
            [0, 1, 0, 1, 0],
            [0, 1, 0, 1, 1],
            [0, 1, 0, 1, 0],
            [0, 1, 0, 1, 0],
        ];

        let h_frame11= [
            [0, 1, 0, 1, 1],
            [0, 1, 0, 1, 0],
            [0, 1, 0, 1, 1],
            [1, 1, 0, 1, 0],
            [0, 1, 0, 1, 1],
        ];

        let h_frame12= [
            [1, 0, 1, 0, 0],
            [0, 0, 1, 0, 1],
            [1, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [1, 0, 0, 0, 1],
        ];

        let h_frame13= [
            [1, 1, 0, 0, 1],
            [0, 0, 0, 0, 1],
            [1, 0, 0, 0, 1],
            [0, 1, 0, 0, 1],
            [1, 0, 0, 0, 1],
        ];

        let h_frame14= [
            [1, 0, 0, 0, 1],
            [0, 1, 0, 1, 0],
            [0, 1, 0, 1, 1],
            [0, 1, 0, 1, 0],
            [1, 0, 0, 1, 0],
        ];

        let h_frame15= [
            [0, 0, 1, 0, 1],
            [1, 0, 1, 0, 1],
            [1, 0, 0, 1, 0],
            [1, 0, 0, 1, 0],
            [1, 0, 0, 1, 0],
        ];

        let h_frame16= [
            [0, 1, 0, 0, 1],
            [0, 1, 0, 1, 0],
            [0, 1, 0, 1, 0],
            [0, 0, 0, 0, 1],
            [0, 1, 0, 0, 0],
        ];

        let h_frame17= [
            [0, 1, 0, 0, 0],
            [1, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 1, 0, 0, 0],
            [1, 0, 0, 0, 0],
        ];

        let h_frame18= [
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
        ];

        // let word = [&m_frame1, &m_frame2, &m_frame3, &m_frame4, &m_frame5, &m_frame6];
        let word = [&h_frame1, &h_frame2, &h_frame3, &h_frame4, &h_frame5, &h_frame6, &h_frame7,&h_frame8,&h_frame9,&h_frame10,&h_frame11,&h_frame12,&h_frame13,&h_frame14,&h_frame15,&h_frame16, &h_frame17, &h_frame18, &m_frame1, &m_frame2, &m_frame3, &m_frame4, &m_frame5, &m_frame6, &m_frame7];

        loop {
            for w in 0..word.len() - 1 {
                    let frames = generate_word_animation(*word[w], *word[w + 1]);

                    for frame in &frames {
                        display.show(&mut timer, *frame, 500);
                    }
                }
            display.clear();
            timer.delay_ms(250_u32);
        }
    }

    panic!("End");
}
