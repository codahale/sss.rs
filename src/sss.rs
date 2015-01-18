use std::collections::VecMap;
use std::rand;

use poly::*;

/// Split a secret into N shares, of which K are required to re-combine. Returns
/// a map of share IDs to share values.
pub fn split<'a, T: rand::Rng>(n: u8, k: u8, secret: &'a Vec<u8>, rng: &mut T) -> VecMap<Vec<u8>> {
    // Generate a random K-degree polynomial for each byte of the secret.
    let polys: Vec<Vec<u8>> = secret.iter().map(|b| {
        generate(k-1, *b, rng)
    }).collect();

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
        let mut shares: VecMap<Vec<u8>> = VecMap::new();
        shares.insert(1, vec![118, 163, 66, 80, 187]);
        shares.insert(2, vec![239, 91, 129, 172, 98]);
        shares.insert(3, vec![152, 250, 192, 248, 220]);

        assert_eq!(combine(&shares), vec![1, 2, 3, 4, 5])
    }
}
