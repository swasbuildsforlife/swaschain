# SwasChain Protocol Specification

## 1. Purpose

This document defines the initial protocol rules for SwasChain.

It describes the structure and validation requirements for:

- Transactions
- Accounts
- Blocks
- Block hashes
- State transitions
- Nonces
- Signatures
- Transaction ordering
- Genesis state
- Invalid data handling

This document is the protocol-level contract that future SwasChain
implementations must follow.

The implementation may evolve, but consensus-critical behavior must
remain deterministic and explicitly specified.

---

## 2. Protocol Goals

SwasChain aims to provide a blockchain protocol with:

- Deterministic execution
- Cryptographically verifiable transactions
- Replay protection
- Verifiable block linking
- Explicit state transitions
- Persistent and reproducible blockchain state
- Secure peer-to-peer validation
- Clear separation between protocol and application layers

The protocol should remain understandable enough that an independent
implementation can eventually reproduce its consensus-critical rules.

---

## 3. Initial Network Model

SwasChain will initially operate as a permissioned development network
while the protocol is being built and tested.

The development network will allow controlled experimentation with
multiple independent nodes.

The protocol architecture should avoid depending on a single node.

The eventual network model will support multiple participating nodes
that independently:

- Receive transactions
- Validate transactions
- Maintain state
- Validate blocks
- Exchange network messages
- Participate in consensus

---

## 4. Account Model

SwasChain initially uses an account-based state model.

An account contains:

    Address
    Balance
    Nonce

Conceptually:

    Account {
        address
        balance
        nonce
    }

### 4.1 Address

An address uniquely identifies an account.

The exact binary representation and human-readable encoding will be
defined by the cryptographic implementation.

An address must be deterministically derived from the account's
cryptographic identity according to the protocol rules.

### 4.2 Balance

Balance represents the amount of the native SwasChain asset controlled
by an account.

Balances must never become negative.

### 4.3 Nonce

Each account has a monotonically increasing nonce.

The nonce provides transaction ordering and replay protection.

For a transaction submitted by an account:

    transaction.nonce == account.nonce

must be satisfied before that transaction can be executed.

After successful execution:

    account.nonce = account.nonce + 1

A transaction that has already been executed must not be executable
again with the same nonce.

---

## 5. Transaction Model

A SwasChain transaction represents a request to modify blockchain state.

The initial transaction model is:

    Transaction {
        version
        sender
        recipient
        amount
        nonce
        fee
        signature
    }

Each field has a specific protocol purpose.

---

## 6. Transaction Fields

### 6.1 Version

The version identifies the transaction encoding and protocol rules
used to interpret the transaction.

This allows future protocol upgrades without making old transaction
formats ambiguous.

### 6.2 Sender

The sender identifies the account authorizing the transaction.

The sender must correspond to the public key or cryptographic identity
used to verify the transaction signature.

### 6.3 Recipient

The recipient identifies the account that receives the transferred
value.

The recipient must be represented using the protocol-defined address
format.

### 6.4 Amount

Amount specifies how much value is transferred.

The amount must:

- Be non-negative
- Be representable by the protocol's integer type
- Not exceed the sender's spendable balance after accounting for fees

### 6.5 Nonce

Nonce identifies the transaction sequence number for the sender.

A transaction with an incorrect nonce must not be executed.

### 6.6 Fee

Fee represents the amount paid by the sender for transaction
processing.

The exact fee economics will be finalized as the execution and block
production system is implemented.

For the initial implementation, fee handling must still be
deterministic.

### 6.7 Signature

The signature proves that the sender authorized the transaction.

A node must verify the signature before accepting the transaction.

An invalid signature makes the transaction invalid.

---

## 7. Transaction Signing

A transaction must be signed over a canonical representation of its
signable fields.

The signature must not include itself as part of the signed message.

Conceptually:

    SignableTransaction
            |
            v
        Serialize
            |
            v
           Hash
            |
            v
       Sign with Key
            |
            v
         Signature

The signature is then attached to the transaction.

Every node must reconstruct the same signable representation before
verifying the signature.

---

## 8. Canonical Serialization

Consensus-critical data must have deterministic serialization.

The same logical transaction must produce exactly the same serialized
bytes on every compatible implementation.

Canonical serialization must define:

- Field ordering
- Integer representation
- Byte ordering
- String or byte encoding
- Optional field representation
- Version handling

No consensus-critical hash or signature may depend on ambiguous
serialization.

---

## 9. Transaction Hash

Every transaction will have a deterministic transaction identifier.

Conceptually:

    Transaction Hash
        =
    Hash(Canonical Transaction Bytes)

The transaction hash must uniquely identify the serialized transaction
under the selected cryptographic hash function.

The transaction hash will be used for:

- Transaction lookup
- Duplicate detection
- Block transaction commitments
- Future explorer functionality

The exact hash algorithm will be selected and documented in the
cryptography specification.

---

## 10. Transaction Validation

A transaction must pass all required validation rules before it can
enter the confirmed blockchain state.

Initial validation includes:

1. Correct transaction version
2. Valid sender format
3. Valid recipient format
4. Valid signature
5. Correct nonce
6. Valid amount
7. Sufficient sender balance
8. Valid fee
9. No integer overflow
10. No invalid encoding
11. No duplicate execution

Conceptually:

    Transaction
         |
         v
    Basic Validation
         |
         v
    Signature Validation
         |
         v
    State Validation
         |
         v
    Valid Transaction

Any failed validation rule causes rejection.

---

## 11. Transaction Replay Protection

SwasChain must prevent previously executed transactions from being
executed again.

The primary replay protection mechanism is the account nonce.

Example:

    Account nonce = 7

A valid next transaction must use:

    nonce = 7

After successful execution:

    Account nonce = 8

A second transaction using:

    nonce = 7

must not be accepted for execution.

---

## 12. Transaction Ordering

Transactions from the same account are ordered by nonce.

For example:

    Transaction A -> nonce 5
    Transaction B -> nonce 6
    Transaction C -> nonce 7

The protocol must not execute nonce 6 before nonce 5 for the same
account.

Transactions from different accounts may be ordered by the block
producer according to the block construction rules, provided that the
resulting state transition remains deterministic.

---

## 13. State Transition

The blockchain state changes only through valid transactions.

Conceptually:

    State Before
        +
    Valid Transaction
        =
    State After

For a simple transfer:

    Sender Balance
        =
    Sender Balance - Amount - Fee

    Recipient Balance
        =
    Recipient Balance + Amount

    Sender Nonce
        =
    Sender Nonce + 1

The transition must be atomic.

If a transaction fails validation or execution, its state changes must
not be partially applied.

---

## 14. Block Model

A SwasChain block contains a header and a transaction set.

Conceptually:

    Block {
        header
        transactions
    }

The block header is expected to contain:

    BlockHeader {
        version
        height
        previous_block_hash
        timestamp
        transaction_root
        state_root
        consensus_data
    }

The exact encoding will be finalized during implementation.

---

## 15. Block Height

Block height represents the position of a block in the canonical chain.

The genesis block has:

    height = 0

The next block has:

    height = 1

Then:

    height = 2
    height = 3
    ...

For a valid non-genesis block:

    current.height = previous.height + 1

A block with an invalid height must be rejected.

---

## 16. Previous Block Hash

Every non-genesis block references the hash of the previous block.

Conceptually:

    Block N
       |
       +-- previous_block_hash
                  |
                  v
             Hash(Block N-1)

This creates a cryptographic chain between blocks.

A block whose previous block hash does not match the expected canonical
predecessor must not be accepted into the canonical chain.

---

## 17. Block Timestamp

Each block contains a timestamp.

The timestamp must be represented using a deterministic protocol-defined
format.

Consensus rules will define acceptable timestamp relationships,
including protection against unreasonable timestamps.

Consensus-critical execution must never depend on a node's local clock
after the block has been accepted.

---

## 18. Transaction Root

The block header contains a transaction commitment.

Conceptually:

    Transactions
         |
         v
    Transaction Tree
         |
         v
    Transaction Root

The transaction root allows nodes to verify that the transactions
represented by the block correspond to the block header.

The exact commitment structure will be finalized during the
cryptographic and block-engine phases.

---

## 19. State Root

The block header contains a commitment representing the resulting
blockchain state.

Conceptually:

    Previous State
          +
    Block Transactions
          |
          v
    State Transition
          |
          v
       New State
          |
          v
      State Root

A node independently executing the block must derive the same state
root.

If the computed state root does not match the block's declared state
root, the block must be rejected.

---

## 20. Block Hash

A block hash is derived from the canonical block representation.

Conceptually:

    Block Hash
        =
    Hash(Canonical Block Header)

The exact hashing process will be finalized by the cryptographic
specification.

The block hash must be deterministic.

Identical valid block headers must produce identical hashes.

---

## 21. Genesis Block

The genesis block is the first block in the SwasChain history.

The genesis block has:

    height = 0

It does not reference a previous SwasChain block.

The genesis configuration must be deterministic and reproducible.

The genesis definition will eventually specify:

- Network identifier
- Initial state
- Initial accounts
- Initial balances, if any
- Protocol version
- Genesis timestamp
- Genesis block metadata

Every node operating on the same network must use the same genesis
configuration.

---

## 22. Block Validation

A node must validate a received block before accepting it.

Initial validation includes:

1. Valid block encoding
2. Valid block version
3. Correct block height
4. Correct previous block hash
5. Valid timestamp
6. Valid transaction commitment
7. Valid state commitment
8. Valid transactions
9. Correct state transition
10. Valid consensus metadata

A block failing any required validation rule must be rejected.

---

## 23. Block Execution

A valid block is executed against the previous canonical state.

Conceptually:

    Previous State
          |
          v
    Validate Block
          |
          v
    Execute Transactions
          |
          v
       New State
          |
          v
    Compute State Root
          |
          v
    Compare With Block
          |
          v
    Accept / Reject

A node must not modify canonical state until the block has passed the
required validation and execution checks.

---

## 24. Chain Selection

The initial development network will define a single canonical chain.

The final chain-selection mechanism will depend on the consensus
protocol selected in the consensus phase.

A node must never accept a competing chain solely because it was
received from another peer.

Candidate chains must satisfy the protocol's validation and consensus
rules.

---

## 25. Invalid Transactions

Invalid transactions must not modify blockchain state.

Examples include:

- Invalid signature
- Incorrect nonce
- Insufficient balance
- Invalid amount
- Invalid fee
- Invalid encoding
- Unsupported version

Depending on where the transaction is encountered, the node may:

- Reject it at RPC submission
- Reject it from the mempool
- Reject it during block validation

An invalid transaction must never be executed as a valid state
transition.

---

## 26. Invalid Blocks

An invalid block must not become part of the canonical chain.

Examples include:

- Wrong previous block hash
- Wrong block height
- Invalid transaction
- Invalid transaction root
- Invalid state root
- Invalid block encoding
- Invalid consensus metadata
- Invalid timestamp according to protocol rules

Nodes should record sufficient information for debugging and
observability when rejecting invalid blocks.

---

## 27. Node Validation Boundary

SwasChain follows a strict validation boundary.

All externally supplied data is considered untrusted.

This includes:

- RPC requests
- Transactions
- Blocks
- Peer messages
- Consensus messages
- Serialized data

The general rule is:

    Untrusted Input
          |
          v
       Validate
          |
          v
    Trusted Internal Representation
          |
          v
    Consensus-Critical Processing

No external data should directly mutate consensus-critical state.

---

## 28. Deterministic Execution Requirements

Consensus-critical execution must not depend on:

- Local filesystem ordering
- Local machine architecture
- Uncontrolled randomness
- Local environment variables
- Unstable floating-point behavior
- Local timezone
- Node-specific external services

Protocol execution should use deterministic integer and byte-based
operations wherever possible.

---

## 29. Protocol Versioning

SwasChain protocol structures will include version information where
necessary.

Versioning exists to allow future protocol upgrades while maintaining
clear interpretation of historical data.

A node must reject unsupported protocol versions when required by the
network rules.

Future upgrades must define compatibility and activation rules before
being considered consensus-safe.

---

## 30. Smart Contract Compatibility

The initial protocol focuses on native account transactions.

However, the architecture reserves a future execution layer for smart
contracts.

Future execution may follow:

    Transaction
         |
         v
    Execution Engine
         |
         v
       SwasVM
         |
         v
      Bytecode
         |
         v
    Contract State

Smart contract execution must preserve the same core requirements:

- Determinism
- Verifiability
- Resource limits
- State integrity
- Consensus compatibility

---

## 31. Protocol Invariants

The following invariants are fundamental.

### Invariant 1

Balances must never become negative.

### Invariant 2

A successful transaction increments the sender nonce exactly once.

### Invariant 3

A transaction with an already-consumed nonce must not execute again.

### Invariant 4

Every non-genesis block references the correct previous block.

### Invariant 5

Block height increases sequentially.

### Invariant 6

All honest nodes executing the same valid block from the same previous
state must derive the same resulting state.

### Invariant 7

A block with an incorrect state root must be rejected.

### Invariant 8

An invalid transaction must never modify canonical state.

### Invariant 9

Consensus-critical serialization must be deterministic.

### Invariant 10

Untrusted network input must pass validation before entering
consensus-critical processing.

---

## 32. Initial Protocol Scope

The first implementation will focus on:

- Native account transfers
- Digital signatures
- Nonces
- Transaction validation
- Deterministic state transitions
- Block construction
- Block validation
- Persistent blockchain state

The following will be implemented later:

- Advanced smart contracts
- SwasVM bytecode execution
- Gas/resource metering
- Parallel execution
- Advanced validator economics
- Developer SDKs

This staged approach keeps the initial protocol small enough to verify
while preserving a clear path toward a more capable blockchain system.

---

## 33. Implementation Rule

The Rust implementation must follow this specification.

If implementation behavior conflicts with a consensus-critical protocol
rule, the implementation must be corrected or the specification must
be explicitly updated.

Protocol changes must be documented rather than silently introduced.

---

## 34. Current Status

SwasChain is currently in:

**Phase 1 — Architecture & Protocol Design**

The high-level architecture has been documented.

This document defines the initial protocol model.

The next protocol documents will further specify:

- Transaction serialization
- Cryptographic algorithms
- Block serialization
- State representation
- Storage format
- Networking protocol
- Consensus protocol

No consensus-critical implementation should be considered final until
the corresponding protocol behavior has been explicitly defined and
tested.