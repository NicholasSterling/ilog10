#![feature(int_log)]

// This is a proof of concept for doing integer log10 based on log2.
// This version returns the floor, but it could be trivially modified
// to round or produce the ceiling as desired.  It could also be used
// with other bases > 2.

// Index into this with floor(log2(x)) to get a guess at floor(log10(x)).
// The value might be one lower than it should be, so then you
// have to check whether x > LIMITS(guess).
// In reality this would have 128 u8 entries in order to support u128.
// And you would generate those entries with a macro, but for this demo
// it's probably clearer to just write the entries out like this.
const LOG10S_FOR_LOG2S: [u8; 16] = [
//        log2
//        ---- ------
    0, //   0      1
    0, //   1      2
    0, //   2      4
    0, //   3      8  *
    1, //   4     16
    1, //   5     32
    1, //   6     64  *
    2, //   7    128
    2, //   8    256
    2, //   9    512  *
    3, //  10   1024
    3, //  11   2048
    3, //  12   4096
    3, //  13   8192  *
    4, //  14  16384
    4, //  15  32768
];

// LIMITS[log] is the highest x for which floor(log10(x)) == log.
// In reality you would need this to have all such numbers that fit in a u128.
// And you might want to have a separate version for types up to u32, so that
// you don't have to manipulate u128s.
const LIMITS: [u16; 5] = [
    9,  // maximum x for which floor(log10(x)) is 0
    99,  // maximum x for which floor(log10(x)) is 1
    999,  // ...
    9_999,
    u16::MAX // can't use 99_999 because it's not u16
];

// Returns the floor of log base 10 of its argument.
// In reality you would make this generic, supporting types up to u128.
// The same tables could be used for all of the u* types.
// This routine uses the floor(log2(x)) function in order to get good performance;
// on modern architectures there is typically a fairly quick instruction for that.
pub fn log10_floor(x: u16) -> u8 {
    let log2x = x.log2() as usize;
    let log10x_guess = unsafe {
        // SAFETY: ilog2_floor of a u16 can only be 0..15,
        // for which there are elements in the array.
        *(&LOG10S_FOR_LOG2S).get_unchecked(log2x)
    };
    let limit = unsafe {
        // SAFETY: Indices come from LOG10S_FOR_LOG2S,
        // and we made sure we have an entry for each.
        *(&LIMITS).get_unchecked(log10x_guess as usize)
    };
    if x > limit {
        log10x_guess + 1
    } else {
        log10x_guess
    }
}

/*
// Safe version.
pub fn log10_floor(x: u16) -> u8 {
    let log2x = x.log2() as usize;
    let log10x_guess = LOG10S_FOR_LOG2S[log2x];
    if x > LIMITS[log10x_guess as usize] {
        log10x_guess + 1
    } else {
        log10x_guess
    }
}
 */

// From jhpratt https://github.com/rust-lang/rust/issues/70887
pub const fn log10_u32(x: u32) -> u32 {
    const TABLE: &[u64] = &[
        0x0000_0000_0000,
        0x0000_0000_0000,
        0x0000_0000_0000,
        0x0000_FFFF_FFF6,
        0x0001_0000_0000,
        0x0001_0000_0000,
        0x0001_FFFF_FF9C,
        0x0002_0000_0000,
        0x0002_0000_0000,
        0x0002_FFFF_FC18,
        0x0003_0000_0000,
        0x0003_0000_0000,
        0x0003_0000_0000,
        0x0003_FFFF_D8F0,
        0x0004_0000_0000,
        0x0004_0000_0000,
        0x0004_FFFE_7960,
        0x0005_0000_0000,
        0x0005_0000_0000,
        0x0005_FFF0_BDC0,
        0x0006_0000_0000,
        0x0006_0000_0000,
        0x0006_0000_0000,
        0x0006_FF67_6980,
        0x0007_0000_0000,
        0x0007_0000_0000,
        0x0007_FA0A_1F00,
        0x0008_0000_0000,
        0x0008_0000_0000,
        0x0008_C465_3600,
        0x0009_0000_0000,
        0x0009_0000_0000,
    ];
    ((x as u64 + TABLE[31 - x.leading_zeros() as usize]) >> 32) as _
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test0() {
        assert_eq!(log10_floor(0), 0);
    }

    #[test]
    fn test1() {
        assert_eq!(log10_floor(       1), 0);
        assert_eq!(log10_floor(       9), 0);
        assert_eq!(log10_floor(      10), 1);
        assert_eq!(log10_floor(      99), 1);
        assert_eq!(log10_floor(     100), 2);
        assert_eq!(log10_floor(     999), 2);
        assert_eq!(log10_floor(   1_000), 3);
        assert_eq!(log10_floor(   9_999), 3);
        assert_eq!(log10_floor(  10_000), 4);
        assert_eq!(log10_floor(u16::MAX), 4);
    }

}
