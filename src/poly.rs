#![allow(unstable)]

//! Implements polynomial operations in GF(2^8).

use std::rand;

use gf256::*;

/// Evaluate a polynomial, returning the Y value for the given X value.
pub fn eval(p: &Vec<u8>, x: u8) -> u8 {
    p.iter().rev().fold(0u8, |res, &v| mul(res, x) ^ v)
}

/// Generates a random polynomial of the Nth degree with a Y-intercept with the
/// given value.
pub fn generate<T: rand::Rng>(n: u8, y: u8, rng: &mut T) -> Vec<u8> {
    // Generate a random polynomial.
    let mut p = rng.gen_iter().take(n as usize).collect::<Vec<u8>>();

    // Set its Y-intercept to the given value.
    p[0] = y;

    // Ensure the Nth coefficient is non-zero, otherwise it's an (N-1)th-degree
    // polynomial.
    p[n as usize - 1] = rng.gen_range(1, 255);

    p
}

/// Interpolates a vector of (X, Y) points, returning the Y value at zero.
pub fn y_intercept<'a>(points: &'a Vec<(u8, u8)>) -> u8 {
    let mut value = 0u8;
    for (i, &(ax, ay)) in points.iter().enumerate() {
        let mut weight = 1u8;
        for (j, &(bx, _)) in points.iter().enumerate() {
            if i != j {
                let top = bx; // xor 0
                let bottom = ax ^ bx;
                let factor = div(top, bottom);
                weight = mul(weight, factor);
            }
        }
        value ^= mul(weight, ay)
    }
    value
}

#[cfg(test)]
mod test {
    use super::*;
    use std::rand;

    #[test]
    fn test_eval() {
        assert_eq!(eval(&vec![1, 0, 2, 3], 2), 17);
    }

    #[test]
    fn test_generate() {
        let mut rng = rand::ChaChaRng::new_unseeded();
        assert_eq!(generate(5, 50, &mut rng),
                   vec![50, 160, 64, 83, 161])
    }

    #[test]
    fn test_y_intercept() {
        assert_eq!(y_intercept(&vec![(1,1), (2,2), (3,3)]), 0);
    }
}
