use super::transaction::Transaction;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationError {
    UnsupportedVersion,
    InvalidChainId,
    InvalidAmount,
    InvalidFee,
    InvalidNonce,
    InvalidSignature,
    InvalidSender,
    IntegerOverflow,
    InsufficientBalance,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ValidationContext {
    pub sender_balance: u64,
    pub expected_nonce: u64,
}

pub struct TransactionValidator;

impl TransactionValidator {
    pub fn validate(
        transaction: &Transaction,
        public_key: &[u8; crate::crypto::signature::PUBLIC_KEY_SIZE],
        context: ValidationContext,
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

        if transaction.nonce != context.expected_nonce {
            return Err(ValidationError::InvalidNonce);
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

        if context.sender_balance < required_balance {
            return Err(ValidationError::InsufficientBalance);
        }

        Ok(())
    }
}

pub fn validate_transaction(
    transaction: &Transaction,
    public_key: &[u8; crate::crypto::signature::PUBLIC_KEY_SIZE],
    sender_balance: u64,
    expected_nonce: u64,
) -> Result<(), ValidationError> {
    TransactionValidator::validate(
        transaction,
        public_key,
        ValidationContext {
            sender_balance,
            expected_nonce,
        },
    )
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
            nonce: 5,
            fee: 100,
            signature: [0u8; SIGNATURE_SIZE],
        };

        transaction.sign(&key);

        (transaction, public_key)
    }

    fn valid_context() -> ValidationContext {
        ValidationContext {
            sender_balance: 2_000_000,
            expected_nonce: 5,
        }
    }

    #[test]
    fn validator_accepts_valid_transaction() {
        let (transaction, public_key) = valid_transaction();

        assert_eq!(
            TransactionValidator::validate(
                &transaction,
                &public_key,
                valid_context()
            ),
            Ok(())
        );
    }

    #[test]
    fn validation_wrapper_matches_validator() {
        let (transaction, public_key) = valid_transaction();

        let direct = TransactionValidator::validate(
            &transaction,
            &public_key,
            valid_context(),
        );

        let wrapper = validate_transaction(
            &transaction,
            &public_key,
            2_000_000,
            5,
        );

        assert_eq!(direct, wrapper);
    }

    #[test]
    fn validator_rejects_wrong_version() {
        let (mut transaction, public_key) = valid_transaction();

        transaction.version = 2;

        assert_eq!(
            TransactionValidator::validate(
                &transaction,
                &public_key,
                valid_context()
            ),
            Err(ValidationError::UnsupportedVersion)
        );
    }

    #[test]
    fn validator_rejects_wrong_chain() {
        let (mut transaction, public_key) = valid_transaction();

        transaction.chain_id = 99;

        assert_eq!(
            TransactionValidator::validate(
                &transaction,
                &public_key,
                valid_context()
            ),
            Err(ValidationError::InvalidChainId)
        );
    }

    #[test]
    fn validator_rejects_zero_amount() {
        let (mut transaction, public_key) = valid_transaction();

        transaction.amount = 0;

        assert_eq!(
            TransactionValidator::validate(
                &transaction,
                &public_key,
                valid_context()
            ),
            Err(ValidationError::InvalidAmount)
        );
    }

    #[test]
    fn validator_rejects_zero_fee() {
        let (mut transaction, public_key) = valid_transaction();

        transaction.fee = 0;

        assert_eq!(
            TransactionValidator::validate(
                &transaction,
                &public_key,
                valid_context()
            ),
            Err(ValidationError::InvalidFee)
        );
    }

    #[test]
    fn validator_rejects_nonce_that_is_too_low() {
        let (mut transaction, public_key) = valid_transaction();

        transaction.nonce = 4;

        assert_eq!(
            TransactionValidator::validate(
                &transaction,
                &public_key,
                valid_context()
            ),
            Err(ValidationError::InvalidNonce)
        );
    }

    #[test]
    fn validator_rejects_nonce_that_is_too_high() {
        let (mut transaction, public_key) = valid_transaction();

        transaction.nonce = 6;

        assert_eq!(
            TransactionValidator::validate(
                &transaction,
                &public_key,
                valid_context()
            ),
            Err(ValidationError::InvalidNonce)
        );
    }

    #[test]
    fn validator_rejects_invalid_sender() {
        let (mut transaction, public_key) = valid_transaction();

        transaction.sender = [99u8; ADDRESS_SIZE];

        assert_eq!(
            TransactionValidator::validate(
                &transaction,
                &public_key,
                valid_context()
            ),
            Err(ValidationError::InvalidSender)
        );
    }

    #[test]
    fn validator_rejects_invalid_signature() {
        let (mut transaction, public_key) = valid_transaction();

        transaction.signature[0] ^= 1;

        assert_eq!(
            TransactionValidator::validate(
                &transaction,
                &public_key,
                valid_context()
            ),
            Err(ValidationError::InvalidSignature)
        );
    }

    #[test]
    fn validator_rejects_insufficient_balance() {
        let (transaction, public_key) = valid_transaction();

        let context = ValidationContext {
            sender_balance: 1_000_099,
            expected_nonce: 5,
        };

        assert_eq!(
            TransactionValidator::validate(
                &transaction,
                &public_key,
                context
            ),
            Err(ValidationError::InsufficientBalance)
        );
    }

    #[test]
    fn validator_accepts_exact_balance() {
        let (transaction, public_key) = valid_transaction();

        let context = ValidationContext {
            sender_balance: 1_000_100,
            expected_nonce: 5,
        };

        assert_eq!(
            TransactionValidator::validate(
                &transaction,
                &public_key,
                context
            ),
            Ok(())
        );
    }

    #[test]
    fn validator_rejects_balance_overflow() {
        let key = signing_key();
        let public_key = key.verifying_key().to_bytes();

        let mut transaction = Transaction {
            version: TRANSACTION_VERSION,
            chain_id: DEVELOPMENT_CHAIN_ID,
            sender: sha256(&public_key),
            recipient: [2u8; ADDRESS_SIZE],
            amount: u64::MAX,
            nonce: 5,
            fee: 1,
            signature: [0u8; SIGNATURE_SIZE],
        };

        transaction.sign(&key);

        let context = ValidationContext {
            sender_balance: u64::MAX,
            expected_nonce: 5,
        };

        assert_eq!(
            TransactionValidator::validate(
                &transaction,
                &public_key,
                context
            ),
            Err(ValidationError::IntegerOverflow)
        );
    }
}