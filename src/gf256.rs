//! Implements basic arithmetic operations in GF(2^8).

/// Multiply two elements of GF(2^8).
pub fn mul(e: u8, a: u8) -> u8 {
    // This algorithm is constant-time, allowing us to perform GF256 arithmetic over secret values
    // without leaking information about the values via timing.
    let mut aa = e;
    let mut bb = a;
    let mut r = 0;
    // Loop over all 8 bits, regardless of whether or not it's required. aa != 0 is the usual loop
    // condition, but here it's folded into the per-round bitmask.
    for _ in 0..8 {
        // Perform r ^= bb iff aa & 1 == 0 && aa != 0.
        r ^= bb & ((aa & 1 != 0) as u8 * 0xff) & ((aa != 0) as u8 * 0xff);
        let t = bb & 0x80;
        bb <<= 1;
        // Perform bb ^= 0x1b iff t != 0 && aa != 0.
        bb ^= 0x1b & ((t != 0) as u8 * 0xff) & ((aa != 0) as u8 * 0xff);
        aa >>= 1;
    }
    r
}

/// Divide one element of GF(2^8) by another.
pub fn div(e: u8, a: u8) -> u8 {
    if a == 0 {
        panic!("Divide by zero: {} / {}", e, a)
    }

    // Again, this algorithm is constant-time. First, we find the multiplicative inverse of `a` by
    // iterating over all possible values, and using a bitmask to accumulate only that value which,
    // when multiplied by `a`, is `1`.
    let mut inv = 0;
    for i in 0x00..=0xff {
        inv += i & ((mul(i, a) == 1) as u8 * 0xff);
    }

    // Finally, we multiply `e` by the multiplicative inverse of `a`.
    mul(inv, e)
}

#[cfg(test)]
mod test {
    extern crate proptest;

    use super::*;

    use self::proptest::prelude::*;

    proptest! {
        #[test]
        fn div_is_inverse_of_mul(a in 0u8..=255, b in 1u8..=255) {
            assert_eq!(mul(div(a, b), b), a);
        }

        #[test]
        fn mul_is_inverse_of_div(a in 0u8..=255, b in 1u8..=255) {
            assert_eq!(div(mul(a, b), b), a);
        }

        #[test]
        fn mul_is_commutative(a in 0u8..=255, b in 0u8..=255) {
            assert_eq!(mul(a, b), mul(b, a));
        }
    }

    #[test]
    fn test_mul() {
        assert_eq!(mul(90, 21), 254);
        assert_eq!(mul(133, 5), 167);
    }

    #[test]
    fn test_mul_zero() {
        assert_eq!(mul(0, 21), 0);
    }

    #[test]
    fn test_div() {
        assert_eq!(div(90, 21), 189);
        assert_eq!(div(6, 55), 151);
        assert_eq!(div(22, 192), 138);
    }

    #[test]
    fn test_div_zero() {
        assert_eq!(div(0, 21), 0);
    }
}
