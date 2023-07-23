# mb2-rng: example of using HW RNG with SW PRNG
Bart Massey 2023

This code demos using the `nanorand` crate's `Pcg64` or
`WyRand` software pseudo-random generator in conjunction
with the MicroBit v2 hardware random number generator.

Run with

    cargo embed --release

or

    cargo embed --release --no-default-features --features=wyrand

Each run should show a different sequence of ten die rolls,
in spite of starting from the same state.

See the branch `display` in this repo for a version that
also displays die rolls on the LED display.
