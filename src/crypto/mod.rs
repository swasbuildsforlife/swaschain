use sha2::{Digest, Sha256};

pub const HASH_SIZE: usize = 32;
pub const PUBLIC_KEY_SIZE: usize = 32;
pub const ADDRESS_SIZE: usize = 32;

pub fn sha256(data: &[u8]) -> [u8; HASH_SIZE] {
    let digest = Sha256::digest(data);

    let mut hash = [0u8; HASH_SIZE];
    hash.copy_from_slice(&digest);

    hash
}

pub fn derive_address(public_key: &[u8; PUBLIC_KEY_SIZE]) -> [u8; ADDRESS_SIZE] {
    sha256(public_key)
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

    #[test]
    fn address_derivation_is_deterministic() {
        let public_key = [7u8; PUBLIC_KEY_SIZE];

        let first = derive_address(&public_key);
        let second = derive_address(&public_key);

        assert_eq!(first, second);
    }

    #[test]
    fn address_has_correct_size() {
        let public_key = [7u8; PUBLIC_KEY_SIZE];

        let address = derive_address(&public_key);

        assert_eq!(address.len(), ADDRESS_SIZE);
    }

    #[test]
    fn different_public_keys_produce_different_addresses() {
        let first_key = [1u8; PUBLIC_KEY_SIZE];
        let second_key = [2u8; PUBLIC_KEY_SIZE];

        let first_address = derive_address(&first_key);
        let second_address = derive_address(&second_key);

        assert_ne!(first_address, second_address);
    }

    #[test]
    fn address_is_sha256_of_public_key() {
        let public_key = [9u8; PUBLIC_KEY_SIZE];

        let address = derive_address(&public_key);
        let expected = sha256(&public_key);

        assert_eq!(address, expected);
    }
}