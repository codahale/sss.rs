use std::collections::VecMap;
use std::rand;

use poly::{eval, generate, y_intercept};

/// Split a secret into N shares, of which K are required to re-combine. Returns
/// a map of share IDs to share values.
pub fn split<T: rand::Rng>(n: u8, k: u8, secret: &[u8], rng: &mut T) -> VecMap<Vec<u8>> {
    // Generate a random K-degree polynomial for each byte of the secret.
    let polys = secret.iter().map(|b| {
        generate(k-1, *b, rng)
    }).collect::<Vec<Vec<u8>>>();

    // Collect the evaluation of each polynomial with the share ID as the input.
    (1..n+1).map(|id| {
        (id as usize, polys.iter().map(|p| eval(p, id)).collect())
    }).collect()
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
    points.iter().map(y_intercept).collect()
}

#[cfg(test)]
mod test {
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
        let mut shares: VecMap<Vec<u8>> = VecMap::new();
        shares.insert(1, vec![64, 163, 216, 189, 193]);
        shares.insert(3, vec![194, 250, 117, 212, 82]);
        shares.insert(5, vec![95, 17, 153, 111, 252]);

        assert_eq!(combine(&shares), vec![1, 2, 3, 4, 5])
    }
}
