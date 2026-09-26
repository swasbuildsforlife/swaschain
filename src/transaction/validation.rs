use super::transaction::Transaction;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationError {
    UnsupportedVersion,
    InvalidChainId,
    InvalidAmount,
    InvalidFee,
    InvalidSignature,
    InvalidSender,
    IntegerOverflow,
    InsufficientBalance,
}

pub fn validate_transaction(
    transaction: &Transaction,
    public_key: &[u8; crate::crypto::signature::PUBLIC_KEY_SIZE],
    sender_balance: u64,
) -> Result<(), ValidationError> {
    if transaction.version != super::transaction::TRANSACTION_VERSION {
        return Err(ValidationError::UnsupportedVersion);
    }

    if transaction.chain_id != super::transaction::DEVELOPMENT_CHAIN_ID {
        return Err(ValidationError::InvalidChainId);
    }

    if transaction.amount == 0 {
        return Err(ValidationError::InvalidAmount);
    }

    if transaction.fee == 0 {
        return Err(ValidationError::InvalidFee);
    }

    if !transaction.verify_sender_identity(public_key) {
        return Err(ValidationError::InvalidSender);
    }

    if !transaction.verify_signature(public_key) {
        return Err(ValidationError::InvalidSignature);
    }

    let required_balance = transaction
        .amount
        .checked_add(transaction.fee)
        .ok_or(ValidationError::IntegerOverflow)?;

    if sender_balance < required_balance {
        return Err(ValidationError::InsufficientBalance);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::sha256;
    use crate::transaction::transaction::{
        Transaction,
        ADDRESS_SIZE,
        DEVELOPMENT_CHAIN_ID,
        SIGNATURE_SIZE,
        TRANSACTION_VERSION,
    };

    fn signing_key() -> ed25519_dalek::SigningKey {
        ed25519_dalek::SigningKey::from_bytes(&[42u8; 32])
    }

    fn valid_transaction() -> (Transaction, [u8; 32]) {
        let key = signing_key();
        let public_key = key.verifying_key().to_bytes();

        let mut transaction = Transaction {
            version: TRANSACTION_VERSION,
            chain_id: DEVELOPMENT_CHAIN_ID,
            sender: sha256(&public_key),
            recipient: [2u8; ADDRESS_SIZE],
            amount: 1_000_000,
            nonce: 1,
            fee: 100,
            signature: [0u8; SIGNATURE_SIZE],
        };

        transaction.sign(&key);

        (transaction, public_key)
    }

    #[test]
    fn valid_transaction_passes_validation() {
        let (transaction, public_key) = valid_transaction();

        assert_eq!(
            validate_transaction(&transaction, &public_key, 2_000_000),
            Ok(())
        );
    }

    #[test]
    fn unsupported_version_is_rejected() {
        let (mut transaction, public_key) = valid_transaction();

        transaction.version = 2;

        assert_eq!(
            validate_transaction(&transaction, &public_key, 2_000_000),
            Err(ValidationError::UnsupportedVersion)
        );
    }

    #[test]
    fn invalid_chain_id_is_rejected() {
        let (mut transaction, public_key) = valid_transaction();

        transaction.chain_id = 99;

        assert_eq!(
            validate_transaction(&transaction, &public_key, 2_000_000),
            Err(ValidationError::InvalidChainId)
        );
    }

    #[test]
    fn zero_amount_is_rejected() {
        let (mut transaction, public_key) = valid_transaction();

        transaction.amount = 0;

        assert_eq!(
            validate_transaction(&transaction, &public_key, 2_000_000),
            Err(ValidationError::InvalidAmount)
        );
    }

    #[test]
    fn zero_fee_is_rejected() {
        let (mut transaction, public_key) = valid_transaction();

        transaction.fee = 0;

        assert_eq!(
            validate_transaction(&transaction, &public_key, 2_000_000),
            Err(ValidationError::InvalidFee)
        );
    }

    #[test]
    fn invalid_signature_is_rejected() {
        let (mut transaction, public_key) = valid_transaction();

        transaction.signature[0] ^= 1;

        assert_eq!(
            validate_transaction(&transaction, &public_key, 2_000_000),
            Err(ValidationError::InvalidSignature)
        );
    }

    #[test]
    fn invalid_sender_is_rejected() {
        let (mut transaction, public_key) = valid_transaction();

        transaction.sender = [99u8; ADDRESS_SIZE];

        assert_eq!(
            validate_transaction(&transaction, &public_key, 2_000_000),
            Err(ValidationError::InvalidSender)
        );
    }

    #[test]
    fn insufficient_balance_is_rejected() {
        let (transaction, public_key) = valid_transaction();

        assert_eq!(
            validate_transaction(&transaction, &public_key, 1_000_099),
            Err(ValidationError::InsufficientBalance)
        );
    }

    #[test]
    fn exact_balance_is_accepted() {
        let (transaction, public_key) = valid_transaction();

        assert_eq!(
            validate_transaction(&transaction, &public_key, 1_000_100),
            Ok(())
        );
    }

    #[test]
    fn balance_above_required_amount_is_accepted() {
        let (transaction, public_key) = valid_transaction();

        assert_eq!(
            validate_transaction(&transaction, &public_key, 5_000_000),
            Ok(())
        );
    }

    #[test]
    fn amount_and_fee_overflow_is_rejected() {
        let key = signing_key();
        let public_key = key.verifying_key().to_bytes();

        let mut transaction = Transaction {
            version: TRANSACTION_VERSION,
            chain_id: DEVELOPMENT_CHAIN_ID,
            sender: sha256(&public_key),
            recipient: [2u8; ADDRESS_SIZE],
            amount: u64::MAX,
            nonce: 1,
            fee: 1,
            signature: [0u8; SIGNATURE_SIZE],
        };

        transaction.sign(&key);

        assert_eq!(
            validate_transaction(&transaction, &public_key, u64::MAX),
            Err(ValidationError::IntegerOverflow)
        );
    }
}