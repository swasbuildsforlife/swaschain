use ed25519_dalek::{
    Signature,
    Signer,
    SigningKey,
    Verifier,
    VerifyingKey,
};

pub const PRIVATE_KEY_SIZE: usize = 32;
pub const PUBLIC_KEY_SIZE: usize = 32;
pub const SIGNATURE_SIZE: usize = 64;

pub fn signing_key_from_bytes(bytes: &[u8; PRIVATE_KEY_SIZE]) -> SigningKey {
    SigningKey::from_bytes(bytes)
}

pub fn public_key(signing_key: &SigningKey) -> [u8; PUBLIC_KEY_SIZE] {
    signing_key.verifying_key().to_bytes()
}

pub fn sign(signing_key: &SigningKey, message: &[u8]) -> [u8; SIGNATURE_SIZE] {
    signing_key.sign(message).to_bytes()
}

pub fn verify(
    public_key: &[u8; PUBLIC_KEY_SIZE],
    message: &[u8],
    signature: &[u8; SIGNATURE_SIZE],
) -> bool {
    let verifying_key = match VerifyingKey::from_bytes(public_key) {
        Ok(key) => key,
        Err(_) => return false,
    };

    let signature = Signature::from_bytes(signature);

    verifying_key.verify(message, &signature).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_signing_key() -> SigningKey {
        let private_key = [42u8; PRIVATE_KEY_SIZE];
        signing_key_from_bytes(&private_key)
    }

    #[test]
    fn public_key_is_deterministic() {
        let signing_key = test_signing_key();

        let first = public_key(&signing_key);
        let second = public_key(&signing_key);

        assert_eq!(first, second);
    }

    #[test]
    fn signature_is_deterministic_for_same_key_and_message() {
        let signing_key = test_signing_key();
        let message = b"SwasChain transaction";

        let first = sign(&signing_key, message);
        let second = sign(&signing_key, message);

        assert_eq!(first, second);
    }

    #[test]
    fn signature_has_correct_size() {
        let signing_key = test_signing_key();

        let signature = sign(&signing_key, b"SwasChain");

        assert_eq!(signature.len(), SIGNATURE_SIZE);
    }

    #[test]
    fn valid_signature_verifies() {
        let signing_key = test_signing_key();
        let public_key = public_key(&signing_key);
        let message = b"SwasChain transaction";
        let signature = sign(&signing_key, message);

        assert!(verify(&public_key, message, &signature));
    }

    #[test]
    fn modified_message_fails_verification() {
        let signing_key = test_signing_key();
        let public_key = public_key(&signing_key);
        let signature = sign(&signing_key, b"SwasChain transaction");

        assert!(!verify(
            &public_key,
            b"Modified transaction",
            &signature
        ));
    }

    #[test]
    fn modified_signature_fails_verification() {
        let signing_key = test_signing_key();
        let public_key = public_key(&signing_key);
        let mut signature = sign(&signing_key, b"SwasChain transaction");

        signature[0] ^= 1;

        assert!(!verify(
            &public_key,
            b"SwasChain transaction",
            &signature
        ));
    }
}