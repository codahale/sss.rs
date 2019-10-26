//! Implements polynomial operations in GF(2^8).

use std::error::Error;

use gf256::{div, mul};

/// Evaluate a polynomial, returning the Y value for the given X value.
pub fn eval(p: &[u8], x: u8) -> u8 {
    p.iter().rev().fold(0u8, |res, &v| mul(res, x) ^ v)
}

/// Generates a random polynomial of the Nth degree with a Y-intercept with the
/// given value.
pub fn generate<E, T>(n: usize, y: u8, rng: T) -> Vec<u8>
where
    E: Error,
    T: Fn(&mut [u8]) -> Result<(), E>,
{
    let mut p = vec![0; n + 1];

    // Set its Y-intercept to the given value.
    p[0] = y;

    // Generate random coefficients.
    rng(&mut p[1..]).unwrap();

    // Ensure the Nth coefficient is non-zero, otherwise it's an (N-1)th-degree
    // polynomial.
    while p[n] == 0 {
        rng(&mut p[n..=n]).unwrap();
    }
    p
}

/// Interpolates a vector of (X, Y) points, returning the Y value at zero.
pub fn y_intercept(points: Vec<(u8, u8)>) -> u8 {
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
    use super::*;

    #[test]
    fn test_eval() {
        assert_eq!(eval(&vec![1, 0, 2, 3], 2), 17);
    }

    #[test]
    fn test_generate() {
        assert_eq!(
            generate(5, 50, fake_getrandom),
            vec![50, 1, 0, 0, 0, 1]
        )
    }

    fn fake_getrandom(dest: &mut [u8]) -> Result<(), std::io::Error> {
        dest[0] = 1;
        for x in 1..dest.len() {
            dest[x] = 0;
        }
        return Ok(());
    }

    #[test]
    fn test_y_intercept() {
        assert_eq!(y_intercept(vec![(1, 1), (2, 2), (3, 3)]), 0);
        assert_eq!(y_intercept(vec![(1, 80), (2, 90), (3, 20)]), 30);
        assert_eq!(y_intercept(vec![(1, 43), (2, 22), (3, 86)]), 107);
    }
}
