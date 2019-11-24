//! Implements basic arithmetic operations in GF(2^8).

/// Multiply two elements of GF(2^8).
pub fn mul(lhs: u8, rhs: u8) -> u8 {
    // This algorithm is constant-time, allowing us to perform GF256 arithmetic over secret values
    // without leaking information about the values via timing.
    let mut aa = lhs;
    let mut bb = rhs;
    let mut r = 0;
    // Loop over all 8 bits, regardless of whether or not it's required. aa != 0 is the usual loop
    // condition, but here it's folded into the per-round bitmask.
    for _ in 0..8 {
        // Perform r ^= bb iff aa & 1 == 0 && aa != 0.
        let loop_live = bool_to_mask(aa != 0);
        r ^= bb & bool_to_mask(aa & 1 != 0) & loop_live;
        let t = bb & 0x80;
        bb <<= 1;
        // Perform bb ^= 0x1b iff t != 0 && aa != 0.
        bb ^= 0x1b & bool_to_mask(t != 0) & loop_live;
        aa >>= 1;
    }
    r
}

/// Divide one element of GF(2^8) by another.
pub fn div(lhs: u8, rhs: u8) -> u8 {
    if rhs == 0 {
        panic!("Divide by zero: {} / {}", lhs, rhs)
    }

    // Again, this algorithm is constant-time. First, we find the multiplicative inverse of `a` by
    // iterating over all possible values, and using a bitmask to accumulate only that value which,
    // when multiplied by `a`, is `1`.
    let mut inv = 0;
    for i in 0x00..=0xff {
        inv += i & bool_to_mask(mul(i, rhs) == 1);
    }

    // Finally, we multiply `e` by the multiplicative inverse of `a`.
    mul(inv, lhs)
}

// If b is true, return 0xff, otherwise, return 0x00. Constant-time implementation.
#[inline(always)]
fn bool_to_mask(b: bool) -> u8 {
    (b as u8) * 0xff
}

#[cfg(test)]
mod test {
    extern crate proptest;

    use super::*;

    use self::proptest::prelude::*;

    proptest! {
        #[test]
        fn test_div_is_inverse_of_mul(a in 0u8..=255, b in 1u8..=255) {
            assert_eq!(mul(div(a, b), b), a);
        }

        #[test]
        fn test_mul_is_inverse_of_div(a in 0u8..=255, b in 1u8..=255) {
            assert_eq!(div(mul(a, b), b), a);
        }

        #[test]
        fn test_mul_is_commutative(a in 0u8..=255, b in 0u8..=255) {
            assert_eq!(mul(a, b), mul(b, a));
        }
    }

    #[test]
    fn test_mul() {
        assert_eq!(mul(90, 21), 254);
        assert_eq!(mul(133, 5), 167);
        assert_eq!(mul(0, 21), 0);
    }

    #[test]
    fn test_div() {
        assert_eq!(div(90, 21), 189);
        assert_eq!(div(6, 55), 151);
        assert_eq!(div(22, 192), 138);
        assert_eq!(div(0, 21), 0);
    }
}
