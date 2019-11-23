//! Implements polynomial operations in GF(2^8).

use rand::{CryptoRng, Rng};

use gf256::{div, mul};

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
    let mut value = 0;
    for (i, &(ax, ay)) in points.iter().enumerate() {
        let mut weight = 1;
        for (j, &(bx, _)) in points.iter().enumerate() {
            if i != j {
                weight = mul(weight, div(bx, ax ^ bx));
            }
        }
        value ^= mul(weight, ay)
    }
    value
}

#[cfg(test)]
mod test {
    extern crate proptest;
    extern crate rand_chacha;

    use rand::SeedableRng;

    use super::*;

    use self::proptest::prelude::*;
    use self::rand_chacha::ChaChaRng;

    #[test]
    fn test_eval() {
        assert_eq!(eval(&vec![1, 0, 2, 3], 2), 17);
    }

    proptest! {
        #[test]
        fn generate_last_byte_is_never_zero(seed: [u8; 32]) {
            let mut rng = ChaChaRng::from_seed(seed);
            let p = generate(20, 100, &mut rng);
            assert_ne!(p[20], 0);
        }
    }

    #[test]
    fn test_generate() {
        let seed = [
            0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0,
            0, 0, 0,
        ];
        let mut rng = ChaChaRng::from_seed(seed);
        assert_eq!(generate(5, 50, &mut rng), vec![50, 114, 155, 45, 8, 123])
    }

    #[test]
    fn test_y_intercept() {
        assert_eq!(y_intercept(vec![(1, 1), (2, 2), (3, 3)]), 0);
        assert_eq!(y_intercept(vec![(1, 80), (2, 90), (3, 20)]), 30);
        assert_eq!(y_intercept(vec![(1, 43), (2, 22), (3, 86)]), 107);
    }
}
