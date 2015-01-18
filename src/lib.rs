#![allow(unstable)]

//! # Shamir's Secret Sharing algorithm over GF( 2^8 ).
//!
//! Shamir's Secret Sharing algorithm allows you to securely share a secret with
//! `N` people, allowing the recovery of that secret if `K` of those people
//! combine their shares.
//!
//! ## Example
//!
//! ```
//! use std::collections::VecMap;
//! use std::rand;
//! use sss::{combine,split};
//!
//! // always use OsRng
//! let mut rng = rand::OsRng::new().ok().expect("Failed to obtain OS RNG");
//!
//! // we start with a secret value
//! let secret = "this is a super secret";
//! println!("Secret: {}", secret);
//!
//! // we generate 5 shares of which 3 are required
//! let shares = split(5, 3, &secret.bytes().collect(), &mut rng);
//! println!("Shares: {:?}", shares);
//!
//! // we select 3 of those shares
//! let mut selected: VecMap<Vec<u8>> = VecMap::new();
//! for id in shares.keys().take(3) {
//!     selected.insert(id, shares[id].clone());
//! }
//!
//! // we combine them to recover the secret
//! let recovered = combine(&selected);
//! println!("Recovered: {}", String::from_utf8(recovered).unwrap());
//! ```
//!
//! ## How It Works
//!
//! It begins by encoding a secret as a number (e.g., 42), and generating `N`
//! random polynomial equations of degree `K`-1 which have an X-intercept equal
//! to the secret. Given `K=3`, the following equations might be generated:
//!
//! ```ignore
//! f1(x) =  78x^2 +  19x + 42
//! f2(x) = 128x^2 + 171x + 42
//! f3(x) = 121x^2 +   3x + 42
//! f4(x) =  91x^2 +  95x + 42
//! etc.
//! ```
//!
//! These polynomials are then evaluated for values of X > 0:
//!
//! ```ignore
//! f1(1) =  139
//! f2(2) =  896
//! f3(3) = 1140
//! f4(4) = 1783
//! etc.
//! ```
//!
//! These (x, y) pairs are the shares given to the parties. In order to combine
//! shares to recover the secret, these (x, y) pairs are used as the input
//! points for Lagrange interpolation, which produces a polynomial which matches
//! the given points. This polynomial can be evaluated for f(0), producing the
//! secret value--the common x-intercept for all the generated polynomials.
//!
//! If fewer than K shares are combined, the interpolated polynomial will be
//! wrong, and the result of f(0) will not be the secret.
//!
//! This package constructs polynomials over the field GF( 2^8 ) for each byte
//! of the secret, allowing for fast splitting and combining of anything which
//! can be encoded as bytes.
//!
//! This package has not been audited by cryptography or security professionals.

use poly::{eval, generate, interpolate};
use std::collections::VecMap;
use std::rand;

mod gf256;
mod poly;

/// Split a secret into N shares, of which K are required to re-combine. Returns
/// a map of share IDs to share values.
pub fn split<'a, T: rand::Rng>(n: u8, k: u8, secret: &'a Vec<u8>, rng: &mut T) -> VecMap<Vec<u8>> {
    let polys: Vec<Vec<u8>> = secret.iter().map( |b| generate(k-1, *b, rng) ).collect();
    let mut shares: VecMap<Vec<u8>> = VecMap::with_capacity(n as usize);
    for id in (1..n+1) {
        let share: Vec<u8> = polys.iter().map( |p| eval(p, id) ).collect();
        shares.insert(id as usize, share);
    }
    return shares;
}

/// Combine a map of share IDs into the original secret.
///
/// N.B.: There is no way to know if this is successful or not.
pub fn combine<'a>(shares: &'a VecMap<Vec<u8>>) -> Vec<u8> {
    let mut points: Vec<Vec<(u8, u8)>> = Vec::new();
    for (id, share) in shares.iter() {
        for (i, v) in share.iter().enumerate() {
            if points.len() <= i {
                points.push(Vec::new())
            }
            points[i].push((id as u8, *v));
        }
    }
    return points.iter().map(|v| interpolate(v, 0)).collect();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecMap;
    use std::rand;

    #[test]
    fn test_split() {
        let mut rng = rand::ChaChaRng::new_unseeded();
        let actual = split(5, 3, &vec![1, 2, 3, 4, 5], &mut rng);
        let mut expected: VecMap<Vec<u8>> = VecMap::new();
        expected.insert(1, vec![118, 163, 66, 80, 187]);
        expected.insert(2, vec![239, 91, 129, 172, 98]);
        expected.insert(3, vec![152, 250, 192, 248, 220]);
        expected.insert(4, vec![198, 176, 28, 79, 203]);
        expected.insert(5, vec![177, 17, 93, 27, 117]);

        assert_eq!(actual, expected)
    }

    #[test]
    fn test_combine() {
        let mut expected: VecMap<Vec<u8>> = VecMap::new();
        expected.insert(1, vec![118, 163, 66, 80, 187]);
        expected.insert(2, vec![239, 91, 129, 172, 98]);
        expected.insert(3, vec![152, 250, 192, 248, 220]);

        assert_eq!(vec![1, 2, 3, 4, 5], combine(&expected))
    }
}
