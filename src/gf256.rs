//! Implements basic arithmetic and polynomial operations in GF(2^8).

use rand::{CryptoRng, Rng};

macro_rules! bool_to_mask {
    ($e:expr) => {{
        ($e as u8) * 0xff
    }};
}

/// Multiply two elements of GF(2^8).
fn mul(lhs: u8, rhs: u8) -> u8 {
    // This algorithm is constant-time, allowing us to perform GF256 arithmetic over secret values
    // without leaking information about the values via timing.
    let mut aa = lhs;
    let mut bb = rhs;
    let mut r = 0;
    // Loop over all 8 bits, regardless of whether or not it's required. aa != 0 is the usual loop
    // condition, but here it's folded into the per-round bitmask.
    for _ in 0..8 {
        // Perform r ^= bb iff aa & 1 == 0 && aa != 0.
        let loop_live = bool_to_mask!(aa != 0);
        r ^= bb & bool_to_mask!(aa & 1 != 0) & loop_live;
        let t = bb & 0x80;
        bb <<= 1;
        // Perform bb ^= 0x1b iff t != 0 && aa != 0.
        bb ^= 0x1b & bool_to_mask!(t != 0) & loop_live;
        aa >>= 1;
    }
    r
}

/// Divide one element of GF(2^8) by another.
fn div(lhs: u8, rhs: u8) -> u8 {
    if rhs == 0 {
        panic!("Divide by zero: {} / {}", lhs, rhs)
    }

    // Again, this algorithm is constant-time. First, we find the multiplicative inverse of `a` by
    // iterating over all possible values, and using a bitmask to accumulate only that value which,
    // when multiplied by `a`, is `1`.
    let mut inv = 0;
    for i in 0x00..=0xff {
        inv += i & bool_to_mask!(mul(i, rhs) == 1);
    }

    // Finally, we multiply `e` by the multiplicative inverse of `a`.
    mul(inv, lhs)
}

/// Evaluate a polynomial, returning the Y value for the given X value.
pub fn eval(p: &[u8], x: u8) -> u8 {
    p.iter().rev().fold(0, |res, &v| mul(res, x) ^ v)
}

/// Generates a random polynomial of the Nth degree with a Y-intercept with the
/// given value.
pub fn generate<R>(n: usize, y: u8, rng: &mut R) -> Vec<u8>
where
    R: Rng + CryptoRng,
{
    // Allocate a vec of n+1 bytes.
    let mut p = vec![0; n + 1];

    // Set its Y-intercept to the given value.
    p[0] = y;

    // Generate random coefficients.
    rng.fill_bytes(&mut p[1..n]);

    // Ensure the Nth coefficient is non-zero, otherwise it's an (N-1)th-degree
    // polynomial.
    p[n] = rng.gen_range(1, 255);

    p
}

/// Interpolates a vector of (X, Y) points, returning the Y value at zero.
pub fn y_intercept(points: Vec<(u8, u8)>) -> u8 {
    points.iter().enumerate().fold(0, |value, (i, &(ax, ay))| {
        let weight = points
            .iter()
            .enumerate()
            .filter(|&(j, _)| i != j)
            .fold(1, |weight, (_, &(bx, _))| mul(weight, div(bx, ax ^ bx)));
        value ^ mul(weight, ay)
    })
}

#[cfg(test)]
mod test {
    extern crate proptest;
    extern crate rand_chacha;

    use rand::SeedableRng;

    use super::*;

    use self::proptest::prelude::*;
    use self::rand_chacha::ChaChaRng;

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

    #[test]
    fn test_eval() {
        assert_eq!(eval(&vec![1, 0, 2, 3], 2), 17);
    }

    proptest! {
        #[test]
        fn test_generate(seed: [u8; 32]) {
            let mut rng = ChaChaRng::from_seed(seed);
            let p = generate(20, 100, &mut rng);
            assert_eq!(p[0], 100);
            assert_ne!(p[20], 0);
        }
    }

    #[test]
    fn test_y_intercept() {
        assert_eq!(y_intercept(vec![(1, 1), (2, 2), (3, 3)]), 0);
        assert_eq!(y_intercept(vec![(1, 80), (2, 90), (3, 20)]), 30);
        assert_eq!(y_intercept(vec![(1, 43), (2, 22), (3, 86)]), 107);
    }
}
