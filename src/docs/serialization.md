# Transaction Serialization

## Overview

SwasChain transactions use a deterministic fixed-size binary representation.

The complete serialized transaction is **157 bytes**.

## Transaction Layout

| Field     |     Size | Encoding         |
| --------- | -------: | ---------------- |
| Version   |   1 byte | `u8`             |
| Chain ID  |  4 bytes | Big-endian `u32` |
| Sender    | 32 bytes | Raw bytes        |
| Recipient | 32 bytes | Raw bytes        |
| Amount    |  8 bytes | Big-endian `u64` |
| Nonce     |  8 bytes | Big-endian `u64` |
| Fee       |  8 bytes | Big-endian `u64` |
| Signature | 64 bytes | Ed25519          |

The first seven fields form the **93-byte unsigned transaction payload**.

The 64-byte Ed25519 signature is appended to produce the complete **157-byte transaction**.

## Byte Ordering

All integer fields use **big-endian** encoding.

This provides a deterministic wire representation across supported platforms.

## Signing

The unsigned transaction payload is prefixed with the transaction domain separator:

`SWASCHAIN_TX_V1`

The resulting message is signed using Ed25519.

```text
signed_message = TX_DOMAIN || unsigned_payload
```

The signature is stored separately and is not included in the message being signed.

## Transaction Hash

The complete serialized transaction is prefixed with the transaction hash domain separator:

`SWASCHAIN_TX_HASH_V1`

The transaction hash is calculated as:

```text
hash = SHA256(TX_HASH_DOMAIN || serialized_transaction)
```

The resulting transaction hash is **32 bytes**.

## Deserialization

SwasChain provides two decoding paths.

### Raw Decoding

`deserialize()` converts a fixed 157-byte representation into a `Transaction`.

It is intended as a byte-level decoder and does not perform protocol validation.

### Checked Decoding

`try_deserialize()` first verifies that the input contains exactly 157 bytes.

It then validates:

* Transaction version
* Chain ID

Invalid protocol values are rejected before the transaction is returned.

The current serialization errors are:

```text
InvalidLength
UnsupportedVersion
InvalidChainId
```

## Canonical Representation

A valid transaction must have one deterministic serialized representation.

Canonical serialization is important for:

* Transaction hashes
* Digital signatures
* Network propagation
* Mempool consistency
* Block execution

Different byte representations of the same logical transaction must not be accepted as equivalent protocol representations.

## Implementation

The transaction serialization implementation is located at:

`src/transaction/transaction.rs`

Serialization behavior should remain synchronized with the transaction specification and its corresponding test suite.

Any future changes to the binary format should update:

1. The serialization implementation
2. Serialization tests
3. Protocol documentation
4. Any affected network or execution logic
