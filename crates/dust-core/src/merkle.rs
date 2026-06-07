use crate::Hash;

pub fn merkle_root(mut hashes: Vec<Hash>) -> Hash {
    if hashes.is_empty() {
        return Hash::ZERO;
    }

    while hashes.len() > 1 {
        if hashes.len() % 2 == 1 {
            let last = *hashes.last().expect("non-empty");
            hashes.push(last);
        }

        hashes = hashes
            .chunks_exact(2)
            .map(|pair| {
                let mut bytes = Vec::with_capacity(64);
                bytes.extend_from_slice(pair[0].as_bytes());
                bytes.extend_from_slice(pair[1].as_bytes());
                Hash::digest(bytes)
            })
            .collect();
    }

    hashes[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_merkle_root_is_zero() {
        assert_eq!(merkle_root(Vec::new()), Hash::ZERO);
    }
}
