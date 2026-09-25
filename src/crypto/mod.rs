use sha2::{Digest, Sha256};

pub const HASH_SIZE: usize = 32;

pub fn sha256(data: &[u8]) -> [u8; HASH_SIZE] {
    let digest = Sha256::digest(data);

    let mut hash = [0u8; HASH_SIZE];
    hash.copy_from_slice(&digest);

    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sha256_is_deterministic() {
        let first = sha256(b"SwasChain");
        let second = sha256(b"SwasChain");

        assert_eq!(first, second);
    }

    #[test]
    fn different_input_produces_different_hash() {
        let first = sha256(b"SwasChain");
        let second = sha256(b"swaschain");

        assert_ne!(first, second);
    }

    #[test]
    fn hash_has_correct_size() {
        let hash = sha256(b"SwasChain");

        assert_eq!(hash.len(), HASH_SIZE);
    }
}
