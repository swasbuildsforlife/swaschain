pub const TRANSACTION_VERSION: u8 = 1;
pub const DEVELOPMENT_CHAIN_ID: u32 = 1;

pub const ADDRESS_SIZE: usize = 32;
pub const SIGNATURE_SIZE: usize = 64;
pub const UNSIGNED_TRANSACTION_SIZE: usize = 93;
pub const COMPLETE_TRANSACTION_SIZE: usize = 157;

pub const TX_DOMAIN: &[u8] = b"SWASCHAIN_TX_V1";
pub const TX_HASH_DOMAIN: &[u8] = b"SWASCHAIN_TX_HASH_V1";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transaction {
    pub version: u8,
    pub chain_id: u32,
    pub sender: [u8; ADDRESS_SIZE],
    pub recipient: [u8; ADDRESS_SIZE],
    pub amount: u64,
    pub nonce: u64,
    pub fee: u64,
    pub signature: [u8; SIGNATURE_SIZE],
}

impl Transaction {
    pub fn unsigned_payload(&self) -> [u8; UNSIGNED_TRANSACTION_SIZE] {
        let mut payload = [0u8; UNSIGNED_TRANSACTION_SIZE];

        payload[0] = self.version;
        payload[1..5].copy_from_slice(&self.chain_id.to_be_bytes());
        payload[5..37].copy_from_slice(&self.sender);
        payload[37..69].copy_from_slice(&self.recipient);
        payload[69..77].copy_from_slice(&self.amount.to_be_bytes());
        payload[77..85].copy_from_slice(&self.nonce.to_be_bytes());
        payload[85..93].copy_from_slice(&self.fee.to_be_bytes());

        payload
    }

    pub fn signed_message(&self) -> Vec<u8> {
        let payload = self.unsigned_payload();

        let mut message = Vec::with_capacity(TX_DOMAIN.len() + payload.len());
        message.extend_from_slice(TX_DOMAIN);
        message.extend_from_slice(&payload);

        message
    }

    pub fn serialize(&self) -> [u8; COMPLETE_TRANSACTION_SIZE] {
        let unsigned = self.unsigned_payload();
        let mut serialized = [0u8; COMPLETE_TRANSACTION_SIZE];

        serialized[..UNSIGNED_TRANSACTION_SIZE].copy_from_slice(&unsigned);
        serialized[UNSIGNED_TRANSACTION_SIZE..].copy_from_slice(&self.signature);

        serialized
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_transaction() -> Transaction {
        Transaction {
            version: TRANSACTION_VERSION,
            chain_id: DEVELOPMENT_CHAIN_ID,
            sender: [1u8; ADDRESS_SIZE],
            recipient: [2u8; ADDRESS_SIZE],
            amount: 1_000_000,
            nonce: 1,
            fee: 100,
            signature: [3u8; SIGNATURE_SIZE],
        }
    }

    #[test]
    fn unsigned_payload_has_protocol_size() {
        let tx = sample_transaction();

        assert_eq!(tx.unsigned_payload().len(), UNSIGNED_TRANSACTION_SIZE);
    }

    #[test]
    fn complete_transaction_has_protocol_size() {
        let tx = sample_transaction();

        assert_eq!(tx.serialize().len(), COMPLETE_TRANSACTION_SIZE);
    }

    #[test]
    fn integers_use_big_endian_encoding() {
        let tx = sample_transaction();
        let payload = tx.unsigned_payload();

        assert_eq!(&payload[1..5], &1u32.to_be_bytes());
        assert_eq!(&payload[69..77], &1_000_000u64.to_be_bytes());
        assert_eq!(&payload[77..85], &1u64.to_be_bytes());
        assert_eq!(&payload[85..93], &100u64.to_be_bytes());
    }

    #[test]
    fn signed_message_contains_domain_separator() {
        let tx = sample_transaction();
        let message = tx.signed_message();

        assert!(message.starts_with(TX_DOMAIN));
        assert_eq!(message.len(), TX_DOMAIN.len() + UNSIGNED_TRANSACTION_SIZE);
    }
}
