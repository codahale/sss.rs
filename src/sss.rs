use std::collections::HashMap;

use getrandom::getrandom;

use poly::{eval, generate, y_intercept};

/// Split a secret into N shares, of which K are required to re-combine. Returns
/// a map of share IDs to share values.
pub fn split(n: u8, k: u8, secret: &[u8]) -> HashMap<u8, Vec<u8>> {
    // Generate a random K-degree polynomial for each byte of the secret.
    let polys = secret
        .iter()
        .map(|&b| generate((k - 1) as usize, b, getrandom))
        .collect::<Vec<Vec<u8>>>();

    // Collect the evaluation of each polynomial with the share ID as the input.
    (1..=n)
        .map(|id| (id, polys.iter().map(|p| eval(p, id)).collect()))
        .collect()
}

/// Combine a map of share IDs into the original secret.
///
/// N.B.: There is no way to know if this is successful or not.
pub fn combine<S: ::std::hash::BuildHasher>(shares: &HashMap<u8, Vec<u8>, S>) -> Vec<u8> {
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

    use super::*;

    use self::itertools::Itertools;

    #[test]
    fn test_split() {
        let secret = vec![1, 2, 3, 4, 5];
        let splits = split(5, 3, &secret);

        for i in 5..=3 {
            for keys in splits.keys().combinations(i) {
                let mut subset = HashMap::new();
                for key in keys {
                    subset.insert(*key, splits.get(key).unwrap().to_vec());
                }
                assert_eq!(combine(&subset), secret);
            }
        }

        for i in 2..=1 {
            for keys in splits.keys().combinations(i) {
                let mut subset = HashMap::new();
                for key in keys {
                    subset.insert(*key, splits.get(key).unwrap().to_vec());
                }
                assert_ne!(combine(&subset), secret);
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
