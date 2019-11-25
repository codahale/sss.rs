#![forbid(unsafe_code)]
#![forbid(warnings)]

//! # Shamir's Secret Sharing algorithm over GF( 2^8 ).
//!
//! Shamir's Secret Sharing algorithm allows you to securely share a secret with
//! `N` people, allowing the recovery of that secret if `K` of those people
//! combine their shares.
//!
//! ## Example
//!
//! ```
//! extern crate rand;
//! extern crate sss;
//!
//! // we start with a secret value
//! let secret = "this is a super secret";
//! println!("Secret: {}", secret);
//!
//! // and a cryptographically secure RNG
//! let mut rng = rand::thread_rng();
//!
//! // we generate 5 shares of which 3 are required
//! let shares = sss::split(5, 3, secret.as_bytes(), &mut rng);
//! println!("Shares: {:?}", shares);
//!
//! // we select 3 of those shares
//! let mut selected = std::collections::HashMap::new();
//! for &id in shares.keys().take(3) {
//!     selected.insert(id, shares[&id].clone());
//! }
//!
//! // we combine them to recover the secret
//! let recovered = sss::combine(&selected);
//! println!("Recovered: {}", String::from_utf8(recovered).unwrap());
//! ```
//!
//! ## How It Works
//!
//! It begins by encoding a secret as a number (e.g., 42), and generating `N`
//! random polynomial equations of degree `K`-1 which have an Y-intercept equal
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
//! secret value--the common Y-intercept for all the generated polynomials.
//!
//! If fewer than K shares are combined, the interpolated polynomial will be
//! wrong, and the result of f(0) will not be the secret.
//!
//! This package constructs polynomials over the field GF( 2^8 ) for each byte
//! of the secret, allowing for fast splitting and combining of anything which
//! can be encoded as bytes.
//!
//! This package has not been audited by cryptography or security professionals.

extern crate rand;

mod gf256;
mod poly;

use std::collections::HashMap;
use std::hash::BuildHasher;

use rand::{CryptoRng, Rng};

use poly::{eval, generate, y_intercept};

/// Split a secret into N shares, of which K are required to re-combine. Returns
/// a map of share IDs to share values.
pub fn split<R>(n: u8, k: u8, secret: &[u8], rng: &mut R) -> HashMap<u8, Vec<u8>>
where
    R: Rng + CryptoRng,
{
    // Generate a random K-degree polynomial for each byte of the secret.
    let polys = secret
        .iter()
        .map(|&b| generate((k - 1) as usize, b, rng))
        .collect::<Vec<Vec<u8>>>();

    // Collect the evaluation of each polynomial with the share ID as the input.
    (1..=n)
        .map(|id| (id, polys.iter().map(|p| eval(p, id)).collect()))
        .collect()
}

/// Combine a map of share IDs into the original secret.
///
/// N.B.: There is no way to know if this is successful or not.
pub fn combine<S>(shares: &HashMap<u8, Vec<u8>, S>) -> Vec<u8>
where
    S: BuildHasher,
{
    let len = shares.values().next().unwrap().len();
    if shares.values().any(|v| v.len() != len) {
        panic!("mismatched share lengths")
    }

    (0..len)
        .map(|i| y_intercept(shares.iter().map(|(&id, v)| (id, v[i])).collect()))
        .collect()
}

#[cfg(test)]
mod test {
    extern crate itertools;
    extern crate proptest;
    extern crate rand_chacha;

    use rand::SeedableRng;

    use super::*;

    use self::itertools::Itertools;
    use self::proptest::prelude::*;
    use self::rand_chacha::ChaChaRng;

    proptest! {
        #[test]
        fn test_split(secret: Vec<u8>, seed: [u8; 32]) {
            let mut rng = ChaChaRng::from_seed(seed);
            let splits = split(5, 3, &secret, &mut rng);

            for i in 5..=3 {
                for keys in splits.keys().combinations(i) {
                    let mut subset = HashMap::new();
                    for &key in keys {
                        subset.insert(key, splits.get(&key).unwrap().to_vec());
                    }
                    assert_eq!(combine(&subset), secret);
                }
            }

            for i in 2..=1 {
                for keys in splits.keys().combinations(i) {
                    let mut subset = HashMap::new();
                    for &key in keys {
                        subset.insert(key, splits.get(&key).unwrap().to_vec());
                    }
                    assert_ne!(combine(&subset), secret);
                }
            }
        }
    }

    #[test]
    fn test_combine() {
        let mut shares = HashMap::new();
        shares.insert(1, vec![64, 163, 216, 189, 193]);
        shares.insert(3, vec![194, 250, 117, 212, 82]);
        shares.insert(5, vec![95, 17, 153, 111, 252]);

        assert_eq!(combine(&shares), vec![1, 2, 3, 4, 5])
    }
}
