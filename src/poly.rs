//! Implements polynomial operations in GF(2^8).

use rand::Rng;

use gf256::{div, mul};

/// Evaluate a polynomial, returning the Y value for the given X value.
pub fn eval(p: &[u8], x: u8) -> u8 {
    p.iter().rev().fold(0u8, |res, &v| mul(res, x) ^ v)
}

/// Generates a random polynomial of the Nth degree with a Y-intercept with the
/// given value.
pub fn generate<T: Rng>(n: u8, y: u8, rng: &mut T) -> Vec<u8> {
    let mut p = vec![0; n as usize];
    // Set its Y-intercept to the given value.
    p[0] = y;
    // Generate random coefficients.
    rng.fill_bytes(&mut p[1..(n as usize - 1)]);
    // Ensure the Nth coefficient is non-zero, otherwise it's an (N-1)th-degree
    // polynomial.
    p[n as usize - 1] = rng.gen_range(1, 255);
    p
}

/// Interpolates a vector of (X, Y) points, returning the Y value at zero.
pub fn y_intercept(points: &[(u8, u8)]) -> u8 {
    let mut value = 0u8;
    for (i, &(ax, ay)) in points.iter().enumerate() {
        let mut weight = 1u8;
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
    extern crate rand_chacha;

    use rand::SeedableRng;

    use super::*;

    use self::rand_chacha::ChaChaRng;

    #[test]
    fn test_eval() {
        assert_eq!(eval(&vec![1, 0, 2, 3], 2), 17);
    }

    #[test]
    fn test_generate() {
        let mut rng = ChaChaRng::from_seed([
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ]);
        assert_eq!(generate(5, 50, &mut rng), vec![50, 118, 184, 224, 144])
    }

    #[test]
    fn test_y_intercept() {
        assert_eq!(y_intercept(&vec![(1, 1), (2, 2), (3, 3)]), 0);
        assert_eq!(y_intercept(&vec![(1, 80), (2, 90), (3, 20)]), 30);
        assert_eq!(y_intercept(&vec![(1, 43), (2, 22), (3, 86)]), 107);
    }
}
