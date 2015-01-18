//! Implements polynomial operations in GF(2^8).

use std::rand;

use gf256::*;

/// Evaluate a polynomial, returning the Y value for the given X value.
pub fn eval(p: &Vec<u8>, x: u8) -> u8 {
    p.iter().rev().fold(0u8, |res, &v| mul(res, x) ^ v)
}

/// Generates a random polynomial of the Nth degree with an X-intercept with the
/// given value.
pub fn generate<T: rand::Rng>(n: u8, x: u8, rng: &mut T) -> Vec<u8> {
    // Generate a random polynomial.
    let mut p: Vec<u8> = rng.gen_iter().take(n as usize-2).collect();

    // Set its X-intercept to the given value.
    p.insert(0, x);

    // Ensure the Nth coefficient is non-zero, otherwise it's an (N-1)th-degree
    // polynomial.
    p.push(rng.gen_range(1, 255));

    return p;
}

/// Interpolates a vector of (X, Y) points, returning the Y value for the given
/// X value.
pub fn interpolate<'a>(points: &'a Vec<(u8, u8)>, x: u8) -> u8 {
    let mut value = 0u8;
    for (i, &(ax, ay)) in points.iter().enumerate() {
        let mut weight = 1u8;
        for (j, &(bx, _)) in points.iter().enumerate() {
            if i != j {
                let top = x ^ bx;
                let bottom = ax ^ bx;
                let factor = div(top, bottom);
                weight = mul(weight, factor);
            }
        }
        value ^= mul(weight, ay)
    }
    return value
}

#[cfg(test)]
mod tests {
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
                   vec![50u8, 118u8, 160u8, 64u8, 84u8])
    }


    #[test]
    fn test_interpolate() {
        assert_eq!(interpolate(&vec![(1,1), (2,2), (3,3)], 0), 0);
    }
}
