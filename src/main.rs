//! Generate some random numbers.

#![no_std]
#![no_main]

use cortex_m_rt::entry;
use microbit::{
    Board,
    display::blocking::Display,
    hal::{Rng as HwRng, timer::Timer},
};
use nanorand::{Rng, SeedableRng};
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

#[cfg(feature = "pcg64")]
mod sw_rng {
    pub use nanorand::pcg64::Pcg64 as SwRng;
    pub const SEED_SIZE: usize = 16;
}
#[cfg(feature = "wyrand")]
mod sw_rng {
    pub use nanorand::wyrand::WyRand as SwRng;
    pub const SEED_SIZE: usize = 8;
}
use sw_rng::*;


/// Reseed the `rng` using the built-in crypto-grade
/// RNG. Lol. What a world we live in.
fn reseed(sw_rng: &mut SwRng, hw_rng: &mut HwRng) {
    let mut seed = [0u8; SEED_SIZE];
    hw_rng.random(&mut seed);
    sw_rng.reseed(seed);
}

static D6_0: [[u8; 5]; 5] = [
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
];

static D6_1: [[u8; 5]; 5] = [
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 1, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
];

static D6_2: [[u8; 5]; 5] = [
    [0, 0, 0, 0, 0],
    [0, 0, 0, 1, 0],
    [0, 0, 0, 0, 0],
    [0, 1, 0, 0, 0],
    [0, 0, 0, 0, 0],
];

static D6_3: [[u8; 5]; 5] = [
    [0, 0, 0, 0, 0],
    [0, 0, 0, 1, 0],
    [0, 0, 1, 0, 0],
    [0, 1, 0, 0, 0],
    [0, 0, 0, 0, 0],
];

static D6_4: [[u8; 5]; 5] = [
    [0, 0, 0, 0, 0],
    [0, 1, 0, 1, 0],
    [0, 0, 0, 0, 0],
    [0, 1, 0, 1, 0],
    [0, 0, 0, 0, 0],
];

static D6_5: [[u8; 5]; 5] = [
    [0, 0, 0, 0, 0],
    [0, 1, 0, 1, 0],
    [0, 0, 1, 0, 0],
    [0, 1, 0, 1, 0],
    [0, 0, 0, 0, 0],
];

static D6_6: [[u8; 5]; 5] = [
    [0, 0, 0, 0, 0],
    [0, 1, 0, 1, 0],
    [0, 1, 0, 1, 0],
    [0, 1, 0, 1, 0],
    [0, 0, 0, 0, 0],
];

static D6: [[[u8; 5]; 5]; 7] = [ D6_0, D6_1, D6_2, D6_3, D6_4, D6_5, D6_6 ];

#[entry]
fn init() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();
    let mut hw_rng = HwRng::new(board.RNG);
    let mut frame_timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    let mut rng = SwRng::new_seed(1);
    reseed(&mut rng, &mut hw_rng);

    loop {
        let r: usize = rng.generate_range(1..7);
        rprintln!("{}", r);
        display.show(&mut frame_timer, D6[r], 700);
        display.show(&mut frame_timer, D6[0], 300);
    }
}
