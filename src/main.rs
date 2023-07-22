//! Generate some random numbers.

#![no_std]
#![no_main]

use cortex_m_rt::entry;
#[rustfmt::skip]
use microbit::{Board, hal::Rng as HwRng};
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

#[entry]
fn init() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();
    let mut hw_rng = HwRng::new(board.RNG);
    let mut rng = SwRng::new_seed(1);
    reseed(&mut rng, &mut hw_rng);

    for _ in 0..10 {
        let r: u8 = rng.generate_range(1..7);
        rprintln!("{}", r);
    }
    loop {
        continue;
    }
}
