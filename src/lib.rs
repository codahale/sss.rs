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
//! let shares = split(5, 3, secret.as_bytes(), &mut rng);
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

#![feature(core)]
#![feature(rand)]

pub use sss::{combine, split};

mod gf256;
mod poly;
mod sss;
