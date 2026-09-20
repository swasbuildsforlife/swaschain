# SwasChain Transaction Specification

Status: Draft v0.1
Protocol: SwasChain
Layer: Transaction / Cryptographic Protocol
Last Updated: 2026-09-20


## 1. Purpose

This document defines the normative transaction specification for SwasChain.

It describes:

- transaction structure
- field semantics
- canonical encoding
- transaction hashing
- digital signatures
- address derivation
- nonce handling
- amount and fee rules
- transaction validation
- replay protection
- mempool validation
- block-level validation
- error categories
- protocol versioning

The implementation must follow this specification.

If the implementation and this document disagree, the protocol specification must be reviewed before implementation behavior is changed.


## 2. Transaction Design Goals

SwasChain transactions must satisfy the following properties:

1. Deterministic serialization
2. Cryptographic authenticity
3. Replay protection
4. Deterministic validation
5. Explicit fee accounting
6. Explicit transaction versioning
7. Safe integer handling
8. Stable transaction identifiers
9. Network-independent verification rules
10. Forward-compatible protocol design

Transactions must never depend on:

- local machine state
- operating-system-specific behavior
- floating-point arithmetic
- locale-specific formatting
- unordered map serialization
- wall-clock precision beyond defined protocol rules


## 3. Transaction Version

Every transaction contains a protocol version.

Initial version:

    version = 1

The version determines how the transaction is:

- serialized
- hashed
- signed
- validated
- interpreted

Future protocol versions may introduce additional fields or validation rules.

Nodes must reject unsupported transaction versions unless explicit compatibility rules exist.


## 4. Transaction Structure

The initial SwasChain transaction contains:

    Transaction
    ├── version
    ├── chain_id
    ├── sender
    ├── recipient
    ├── amount
    ├── nonce
    ├── fee
    └── signature

Conceptually:

    Transaction {
        version
        chain_id
        sender
        recipient
        amount
        nonce
        fee
        signature
    }

The signature is not part of the signed message itself.

Instead, the signature authenticates the canonical encoding of the unsigned transaction.


## 5. Field Definitions

### 5.1 version

Type:

    uint8

Meaning:

Identifies the transaction protocol version.

Initial value:

    1

Validation:

    version == 1

for the initial implementation.


### 5.2 chain_id

Type:

    uint32

Meaning:

Identifies the blockchain network on which the transaction is valid.

Example:

    1 = SwasChain development network

Additional network identifiers may be assigned later.

The chain ID is included in the signed payload to prevent cross-network transaction replay.

A transaction created for one chain must not be valid on another chain with a different chain ID.


### 5.3 sender

Type:

    32-byte address

Meaning:

The account authorizing the transaction.

The sender must correspond to the public key used to verify the transaction signature.

Address derivation is defined in Section 8.


### 5.4 recipient

Type:

    32-byte address

Meaning:

The account receiving the transferred amount.

The initial protocol supports account-to-account transfers.

Smart-contract addresses may be supported by a future protocol version.


### 5.5 amount

Type:

    uint64

Meaning:

The number of base units transferred from the sender to the recipient.

The protocol does not use floating-point values.

For example:

    1000000

represents exactly 1,000,000 base units.

The meaning of a higher-level denomination can be defined by the network configuration.

Validation:

    amount > 0

unless a future protocol version explicitly permits zero-value transactions.


### 5.6 nonce

Type:

    uint64

Meaning:

A monotonically increasing transaction sequence number associated with the sender account.

For an account whose current nonce is:

    N

the next valid transaction must normally contain:

    nonce = N + 1

The nonce provides replay protection and deterministic transaction ordering within an account.


### 5.7 fee

Type:

    uint64

Meaning:

The amount paid by the sender to the network for processing the transaction.

The initial implementation treats the fee as an explicit transaction field.

The state transition must ensure:

    sender_balance >= amount + fee

without integer overflow.

Fee distribution is a consensus-level decision and must be deterministic.


### 5.8 signature

Type:

    64 bytes

Meaning:

An Ed25519 digital signature over the canonical unsigned transaction payload.

The signature authenticates:

    version
    chain_id
    sender
    recipient
    amount
    nonce
    fee

The signature field itself is excluded from the signed payload.


## 6. Cryptographic Algorithms

The initial protocol uses:

    Signature algorithm: Ed25519
    Hash algorithm: SHA-256

These choices provide a conservative and widely implemented cryptographic foundation for the initial protocol.

The implementation must use established cryptographic libraries rather than custom cryptographic implementations.

Rust implementation is expected to use established ecosystem implementations.


## 7. Key Pair

Each account is controlled by an Ed25519 key pair.

Conceptually:

    Private Key
         │
         └── signs transactions

    Public Key
         │
         └── verifies signatures

The private key must never be included in a transaction.

The private key must never be transmitted to another node.

The private key must never be stored in plaintext by a network node.

Wallet software is responsible for protecting private keys.


## 8. Address Derivation

The initial SwasChain address is derived from the public key.

Procedure:

    address_bytes = SHA256(public_key_bytes)

The resulting 32-byte digest is the account address.

Conceptually:

    Ed25519 Public Key
            │
            ▼
          SHA-256
            │
            ▼
    32-byte Account Address

The address is represented in binary internally.

Human-readable encoding is a wallet/CLI presentation concern and must not alter the underlying 32-byte address.


## 9. Canonical Serialization

Every transaction must have exactly one canonical binary representation.

This is required because different serialized representations would produce different signatures and transaction hashes.

The initial serialization format uses fixed-width fields.

Field order:

    version
    chain_id
    sender
    recipient
    amount
    nonce
    fee

The signature is serialized separately after the signed payload.


## 10. Integer Encoding

All integer fields use:

    Unsigned integer
    Big-endian byte order

Field widths:

    version   = 1 byte
    chain_id  = 4 bytes
    amount    = 8 bytes
    nonce     = 8 bytes
    fee       = 8 bytes

This produces deterministic encoding independent of CPU architecture.


## 11. Binary Transaction Layout

The unsigned transaction payload has the following layout:

    Offset    Size    Field
    ------    ----    ----------------
    0         1       version
    1         4       chain_id
    5         32      sender
    37        32      recipient
    69        8       amount
    77        8       nonce
    85        8       fee

Total unsigned payload size:

    93 bytes

The complete serialized transaction additionally contains:

    64-byte signature

Therefore:

    Complete transaction size = 157 bytes

for the initial fixed-width format.


## 12. Signed Payload

The exact byte sequence signed by Ed25519 is:

    version
    ||
    chain_id
    ||
    sender
    ||
    recipient
    ||
    amount
    ||
    nonce
    ||
    fee

where || means byte concatenation.

The signature is calculated over these exact bytes.


## 13. Domain Separation

To reduce the risk of cross-protocol signature confusion, the signed message includes a fixed protocol domain separator.

Initial domain:

    SWASCHAIN_TX_V1

The domain separator is encoded as its ASCII byte sequence.

The actual signed message is:

    domain_separator
    ||
    canonical_unsigned_transaction

Therefore:

    signature =
        Ed25519.sign(
            "SWASCHAIN_TX_V1" ||
            canonical_unsigned_transaction
        )

The domain separator must be identical for all SwasChain version 1 transaction signatures.


## 14. Transaction Hash

Every transaction has a deterministic transaction identifier.

The transaction hash is:

    tx_hash =
        SHA256(
            "SWASCHAIN_TX_HASH_V1" ||
            canonical_complete_transaction
        )

where:

    canonical_complete_transaction =
        canonical_unsigned_transaction ||
        signature

The result is:

    32 bytes

The transaction hash identifies the serialized transaction under the security assumptions of SHA-256.


## 15. Transaction Validation

Every node must independently validate transactions.

Validation must be deterministic.

The initial validation sequence is:

    1. Check transaction version
    2. Check chain ID
    3. Check address lengths
    4. Check amount
    5. Check nonce
    6. Check fee
    7. Reconstruct canonical unsigned payload
    8. Verify signature
    9. Verify sender identity
    10. Check sender account exists
    11. Check sender balance
    12. Check nonce against account state

A transaction is accepted only if all required checks succeed.


## 16. Signature Validation

The node reconstructs:

    domain_separator ||
    canonical_unsigned_transaction

using the received transaction fields.

The node then verifies:

    Ed25519.verify(
        public_key,
        signed_payload,
        signature
    )

The public key must correspond to the sender address.

Therefore:

    SHA256(public_key) == sender

must hold.

If either signature verification or address derivation verification fails, the transaction is invalid.


## 17. Balance Validation

Before execution:

    required_balance = amount + fee

The implementation must check for integer overflow before calculating the required balance.

A transaction is invalid if:

    sender_balance < amount + fee

The transaction must never cause the sender balance to become negative.

State transitions must use checked arithmetic.


## 18. Nonce Validation

Each account maintains a current nonce.

For the initial sequential account model:

    expected_nonce = account_nonce + 1

A transaction is valid only when:

    transaction.nonce == expected_nonce

Transactions with:

    nonce < expected_nonce

are considered stale or replayed.

Transactions with:

    nonce > expected_nonce

are considered future transactions and may be handled by the mempool according to implementation policy.

The initial implementation should keep nonce handling deterministic and simple.


## 19. Replay Protection

SwasChain provides replay protection through multiple protocol components:

    chain_id
    +
    sender
    +
    nonce
    +
    transaction signature

A transaction signed for another chain must fail chain ID validation.

A previously executed transaction from the same sender must fail nonce validation when replayed.

A transaction with modified fields must fail signature verification.


## 20. Transaction Ordering

Transactions from the same sender are ordered by nonce.

Example:

    Alice nonce 1
    Alice nonce 2
    Alice nonce 3

must execute sequentially.

The network may order transactions from different senders according to block-production rules.

Transaction ordering must never modify transaction contents.

A block producer must not alter a transaction after signature verification.


## 21. State Transition

A valid transfer produces the following state changes:

    sender_balance
        -= amount + fee

    recipient_balance
        += amount

    sender_nonce
        += 1

Conceptually:

    State'

is derived deterministically from:

    State
    +
    Transaction

The same initial state and same transaction must always produce the same resulting state.


## 22. Fee Accounting

The fee is deducted from the sender:

    sender_balance -= fee

The exact recipient of the fee is a consensus rule.

For the initial implementation, the fee may be assigned to the block producer or a protocol-defined reward account.

This rule must be deterministic and must not depend on local node configuration.

The final fee distribution rule must be locked before implementing production block execution.


## 23. Mempool Validation

The mempool is responsible for rejecting obviously invalid transactions before they reach block production.

Mempool validation includes:

    transaction version
    chain ID
    serialization
    signature
    sender identity
    nonce
    basic balance availability
    fee

The mempool must not permanently modify canonical blockchain state.

A transaction accepted into the mempool is not yet final.


## 24. Block-Level Transaction Validation

When a transaction appears inside a block, validators must execute it against the block's evolving state.

A transaction that was valid when received by the mempool may later become invalid because:

- another transaction consumed the sender's balance
- another transaction used the same nonce
- the transaction became stale
- the block ordering changed its execution context

Therefore block execution must independently validate every transaction.

Mempool acceptance is never a substitute for consensus validation.


## 25. Invalid Transaction Categories

The implementation should expose structured validation errors.

Initial categories:

    UnsupportedVersion
    InvalidChainId
    InvalidAddress
    InvalidAmount
    InvalidFee
    InvalidNonce
    InsufficientBalance
    InvalidSignature
    InvalidSender
    SerializationError
    IntegerOverflow

Errors should be machine-readable internally.

Human-readable messages may be generated by the CLI or RPC layer.


## 26. Transaction Immutability

After signing, transaction fields must be treated as immutable.

Changing any of:

    version
    chain_id
    sender
    recipient
    amount
    nonce
    fee

invalidates the signature.

Changing the signature changes the transaction hash.

Nodes must never silently modify received transactions.


## 27. Transaction Identity

A transaction is identified by:

    tx_hash

The hash must be calculated from the canonical complete transaction.

Transaction IDs must therefore be deterministic across all honest nodes.

If two honest nodes receive byte-identical transactions, they must calculate the same transaction hash.


## 28. Serialization Invariants

The following invariants must always hold:

    serialize(tx) == serialize(tx)

for the same transaction.

Additionally:

    deserialize(serialize(tx)) == tx

must hold for all valid version 1 transactions.

Canonical serialization must not depend on:

- JSON key ordering
- whitespace
- locale
- operating system
- CPU architecture
- Rust struct memory layout
- hash-map iteration order


## 29. JSON Representation

JSON may be used by:

- CLI tools
- RPC
- debugging tools
- explorer APIs

JSON is not the consensus serialization format.

Consensus-critical hashing and signing must use the canonical binary representation defined in this document.

This separation prevents presentation formats from affecting consensus.


## 30. Transaction Lifecycle

The complete lifecycle is:

    Wallet
      │
      ▼
    Create transaction
      │
      ▼
    Canonical serialization
      │
      ▼
    Sign transaction
      │
      ▼
    Calculate transaction hash
      │
      ▼
    Submit to node
      │
      ▼
    Mempool validation
      │
      ▼
    Mempool
      │
      ▼
    Block proposal
      │
      ▼
    Block validation
      │
      ▼
    State transition
      │
      ▼
    Committed block


## 31. Wallet Responsibilities

The wallet is responsible for:

- generating key pairs
- securely storing private keys
- deriving addresses
- constructing transactions
- signing transactions
- displaying transaction identifiers
- preventing accidental malformed transactions

The wallet must never expose private keys to RPC peers or blockchain nodes.


## 32. Node Responsibilities

A node is responsible for:

- validating transaction structure
- validating signatures
- validating sender identity
- validating nonce
- validating balance
- validating fees
- maintaining the mempool
- executing transactions
- including valid transactions in blocks
- rejecting invalid transactions

A node must never trust a transaction merely because it was received from another peer.


## 33. Deterministic Execution Requirement

All consensus-critical transaction execution must be deterministic.

Transaction execution must not depend on:

    random numbers
    local filesystem state
    environment variables
    wall-clock time
    network response timing
    thread scheduling
    floating-point arithmetic

Any value required for consensus must be explicitly represented in protocol data.


## 34. Integer Safety

All balance and amount arithmetic must use checked operations.

For example:

    checked_add
    checked_sub

must be used where overflow or underflow is possible.

An arithmetic overflow must cause transaction or block validation failure.

Silent integer wrapping is forbidden in consensus-critical state transitions.


## 35. Future Compatibility

Future transaction versions may introduce:

- additional transaction types
- contract calls
- resource limits
- gas
- signatures from multiple parties
- account permissions
- fee markets
- batched operations

Such changes must use explicit protocol versioning.

Version 1 behavior must remain deterministic and well-defined.


## 36. Initial Transaction Scope

Version 1 supports:

    Account-to-account value transfer

It does not yet implement:

    Smart contracts
    Gas
    Contract execution
    Multi-signature accounts
    Token standards
    Cross-chain transfers
    Privacy transactions
    Zero-knowledge proofs

These are future protocol extensions.


## 37. Security Requirements

The implementation must:

1. Never implement custom cryptographic primitives.
2. Never log private keys.
3. Never include private keys in Git.
4. Never sign non-canonical data.
5. Never use floating-point values for balances.
6. Never trust peer-provided transaction validity.
7. Never bypass signature verification.
8. Never silently ignore nonce conflicts.
9. Never allow integer overflow in state transitions.
10. Never use presentation-layer JSON as consensus serialization.


## 38. Testing Requirements

The transaction implementation must eventually include:

### Unit Tests

- serialization
- deserialization
- signature generation
- signature verification
- address derivation
- transaction hashing
- nonce validation
- balance validation
- fee validation
- overflow detection

### Property Tests

Important properties include:

    deserialize(serialize(tx)) == tx

and:

    same transaction -> same hash

and:

    modified signed field -> invalid signature

### Negative Tests

The implementation must test:

- corrupted signatures
- altered amounts
- altered recipients
- altered nonces
- altered chain IDs
- invalid addresses
- insufficient balance
- integer overflow
- replayed transactions
- malformed serialized bytes


## 39. Protocol Invariants

The following must always remain true:

1. Every valid transaction has a valid signature.
2. Every valid sender address corresponds to the signing public key.
3. Every executed transaction has a valid nonce.
4. A transaction cannot spend more than the sender's available balance.
5. Transaction execution is deterministic.
6. Transaction hashes are deterministic.
7. Canonical serialization is deterministic.
8. A transaction cannot be modified after signing.
9. Chain ID prevents cross-network replay.
10. Consensus nodes independently validate transactions.


## 40. Implementation Boundary

This specification defines protocol behavior.

The Rust implementation should separate:

    Transaction Data
          │
          ├── Serialization
          ├── Hashing
          ├── Signing
          ├── Verification
          └── Validation

from:

    Mempool
    Block Engine
    State Engine
    Consensus
    P2P
    RPC
    CLI

This separation allows the transaction layer to be independently tested before integrating it into the rest of SwasChain.


## 41. Current Status

Transaction protocol specification:

    Defined

Implementation:

    Not started

Cryptographic implementation:

    Not started

Wallet implementation:

    Not started

Mempool integration:

    Not started

State execution integration:

    Not started


## 42. Next Engineering Step

After this specification is reviewed and committed, the next Phase 1 work is to finalize:

    Consensus specification
    Networking specification
    VM interface specification

After the protocol documents are stable, implementation begins with the Rust foundation and strongly typed transaction primitives.