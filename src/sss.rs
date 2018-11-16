use std::collections::HashMap;

use rand::Rng;

use poly::{eval, generate, y_intercept};

/// Split a secret into N shares, of which K are required to re-combine. Returns
/// a map of share IDs to share values.
pub fn split<T: Rng>(n: u8, k: u8, secret: &[u8], rng: &mut T) -> HashMap<u8, Vec<u8>> {
    // Generate a random K-degree polynomial for each byte of the secret.
    let polys = secret
        .iter()
        .map(|b| generate(k - 1, *b, rng))
        .collect::<Vec<Vec<u8>>>();

    // Collect the evaluation of each polynomial with the share ID as the input.
    (1..n + 1)
        .map(|id| (id, polys.iter().map(|p| eval(p, id)).collect()))
        .collect()
}

/// Combine a map of share IDs into the original secret.
///
/// N.B.: There is no way to know if this is successful or not.
pub fn combine<S: ::std::hash::BuildHasher>(shares: &HashMap<u8, Vec<u8>, S>) -> Vec<u8> {
    let mut points: Vec<Vec<(u8, u8)>> = Vec::new();
    for (id, share) in shares.iter() {
        for (i, v) in share.iter().enumerate() {
            if points.len() <= i {
                points.push(Vec::new())
            }
            points[i].push((*id, *v));
        }
    }
    points.into_iter().map(|v| y_intercept(&v)).collect()
}

#[cfg(test)]
mod test {
    extern crate rand_chacha;

    use rand::SeedableRng;

    use super::*;

    use self::rand_chacha::ChaChaRng;

    #[test]
    fn test_split() {
        let mut rng = ChaChaRng::from_seed([
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ]);
        let actual = split(5, 3, &vec![1, 2, 3, 4, 5], &mut rng);

        let mut expected: HashMap<u8, Vec<u8>> = HashMap::new();
        expected.insert(1, vec![172, 146, 231, 45, 178]);
        expected.insert(2, vec![64, 57, 208, 86, 112]);
        expected.insert(3, vec![237, 169, 52, 127, 199]);
        expected.insert(4, vec![131, 116, 190, 160, 239]);
        expected.insert(5, vec![46, 228, 90, 137, 88]);

        assert_eq!(actual, expected)
    }

    #[test]
    fn test_combine() {
        let mut shares: HashMap<u8, Vec<u8>> = HashMap::new();
        shares.insert(1, vec![64, 163, 216, 189, 193]);
        shares.insert(3, vec![194, 250, 117, 212, 82]);
        shares.insert(5, vec![95, 17, 153, 111, 252]);

        assert_eq!(combine(&shares), vec![1, 2, 3, 4, 5])
    }
}
