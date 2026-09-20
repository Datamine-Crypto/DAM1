use patterns::{because, source};

pub struct SplitMix;
source!(
    SplitMix,
    "the splitmix generator of Steele, Lea and Flood, whose additive step and two finalizer multipliers give every seed a full period and \
     well spread outputs"
);

const MIX_A: u64 = 0xbf58_476d_1ce4_e5b9;
because!(MIX_A, SplitMix, "the first multiplier of the finalizer, which spreads the high bits of the state into the low ones");

const MIX_B: u64 = 0x94d0_49bb_1331_11eb;
because!(MIX_B, SplitMix, "the second multiplier of the finalizer, which finishes the spread the first began");

const SHIFT_HIGH: u32 = 30;
because!(SHIFT_HIGH, SplitMix, "the first fold of the finalizer, bringing the top of the word down onto the bottom before the first \
     multiply");

const SHIFT_MID: u32 = 27;
because!(SHIFT_MID, SplitMix, "the second fold of the finalizer, between the two multiplies");

const SHIFT_LOW: u32 = 31;
because!(SHIFT_LOW, SplitMix, "the last fold of the finalizer, after the second multiply, which leaves no bit depending on its own \
     position alone");

pub fn mix(z: u64) -> u64 {
    let a = (z ^ (z >> SHIFT_HIGH)).wrapping_mul(MIX_A);
    let b = (a ^ (a >> SHIFT_MID)).wrapping_mul(MIX_B);
    b ^ (b >> SHIFT_LOW)
}
because!(
    mix,
    "the splitmix finalizer on its own, so a sum of hashes is spread into bits that depend on every bit of the sum"
);
