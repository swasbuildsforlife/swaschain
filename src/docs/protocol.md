# SwasChain Protocol Specification

## 1. Purpose

This document defines the initial consensus-critical protocol rules for
SwasChain.

It describes the protocol-level requirements for:

- Accounts
- Transactions
- Transaction validation
- Transaction ordering
- State transitions
- Blocks
- Block validation
- Block linking
- State commitments
- Transaction commitments
- Genesis configuration
- Consensus boundaries
- Deterministic execution
- Protocol versioning

This document acts as a high-level protocol contract for SwasChain.

Exact transaction wire-format and cryptographic encoding requirements
are defined by the transaction specification:

    src/docs/transaction-spec.md

Consensus, networking, and virtual-machine behavior are further defined
by their respective protocol documents.

The Rust implementation must conform to these specifications.

Consensus-critical behavior must never be introduced implicitly through
implementation details.

---

## 2. Protocol Goals

SwasChain aims to provide a blockchain protocol with:

- Deterministic execution
- Cryptographically verifiable transactions
- Replay protection
- Verifiable block linking
- Explicit state transitions
- Persistent and reproducible blockchain state
- Independently verifiable blocks
- Secure peer-to-peer validation
- Clear consensus boundaries
- Clear separation between protocol and application layers
- Explicit protocol versioning

The protocol should remain sufficiently precise that an independent
implementation can eventually reproduce all consensus-critical rules.

---

## 3. Initial Network Model

SwasChain initially operates as a permissioned development network.

The initial development network is intended for:

- Protocol development
- Multi-node testing
- Consensus testing
- Networking testing
- Failure testing
- Adversarial testing
- Performance benchmarking

The initial network uses a controlled validator set.

The protocol architecture must not depend on a single node.

Participating nodes must independently be capable of:

- Receiving transactions
- Validating transactions
- Maintaining blockchain state
- Constructing blocks where authorized
- Validating blocks
- Executing state transitions
- Exchanging network messages
- Participating in consensus
- Recovering blockchain state

The permissioned development model is an initial network configuration,
not a limitation of the long-term protocol architecture.

---

## 4. Account Model

SwasChain uses an account-based state model.

An account contains:

    Account {
        address
        balance
        nonce
    }

The account state is deterministic and must be reproducible by every
honest node.

### 4.1 Address

An address uniquely identifies an account.

For the initial transaction protocol, an address is derived from the
account's public key.

The address derivation rule is:

    address = SHA256(public_key_bytes)

The resulting address is exactly:

    32 bytes

The exact public-key encoding and transaction encoding rules are
defined by:

    src/docs/transaction-spec.md

Addresses are treated as fixed-width binary protocol values.

A human-readable address encoding may be introduced at the application
or wallet layer without changing the underlying protocol address.

### 4.2 Balance

Balance represents the amount of the native SwasChain asset controlled
by an account.

Balances use the protocol-defined unsigned integer representation.

Balances must never become negative.

All balance arithmetic must use checked arithmetic.

An arithmetic overflow or underflow must cause the associated operation
to fail rather than wrapping around.

### 4.3 Nonce

Each account has a monotonically increasing nonce.

The nonce provides:

- Transaction ordering
- Replay protection
- Sequential account execution

For the initial protocol:

    transaction.nonce == account.nonce

must be satisfied before the transaction can execute.

After successful execution:

    account.nonce = account.nonce + 1

A transaction whose nonce has already been consumed must not execute
again.

---

## 5. Transaction Model

A SwasChain transaction represents a request to modify blockchain state.

The initial transaction structure is:

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

The transaction fields have the following protocol-level meaning:

- version: transaction protocol version
- chain_id: network/domain identifier
- sender: source account address
- recipient: destination account address
- amount: value transferred
- nonce: sender transaction sequence number
- fee: transaction processing fee
- signature: authorization proof

The exact binary layout is defined by:

    src/docs/transaction-spec.md

---

## 6. Transaction Fields

### 6.1 Version

The transaction version identifies the transaction encoding and
validation rules used to interpret the transaction.

The initial transaction version is:

    version = 1

Unsupported transaction versions must be rejected.

Future versions must define explicit compatibility and activation rules.

### 6.2 Chain ID

Each transaction contains a protocol-defined chain identifier.

The chain ID separates transaction domains between independent SwasChain
networks.

A transaction signed for one chain must not be valid on another chain
with a different chain ID.

The chain ID therefore forms part of the transaction's signed domain.

The initial representation is:

    uint32

encoded in:

    big-endian

The exact transaction encoding is defined in the transaction
specification.

### 6.3 Sender

The sender identifies the account authorizing the transaction.

The sender is a 32-byte protocol address.

The sender must correspond to the public key whose signature validates
the transaction.

The relationship is:

    public key
        |
        v
    SHA256(public key bytes)
        |
        v
      address
        |
        v
      sender

A transaction is invalid if its signature does not authenticate the
claimed sender.

### 6.4 Recipient

The recipient identifies the account receiving transferred value.

The recipient is represented as a 32-byte protocol address.

The initial native-transfer protocol does not require the recipient to
have previously existed as an account.

Account creation semantics will be defined by the state implementation.

### 6.5 Amount

Amount specifies the native value transferred from sender to recipient.

The initial representation is:

    uint64

encoded in:

    big-endian

Amount must not exceed the sender's available balance after accounting
for the transaction fee.

All arithmetic must use checked operations.

### 6.6 Nonce

Nonce identifies the transaction sequence number for the sender.

The initial representation is:

    uint64

encoded in:

    big-endian

For a transaction to execute successfully:

    transaction.nonce == current_sender_nonce

A transaction with a future or already-consumed nonce must not execute.

### 6.7 Fee

Fee represents the amount paid by the sender for transaction
processing.

The initial representation is:

    uint64

encoded in:

    big-endian

The sender must have sufficient balance to cover:

    amount + fee

Fee accounting must be deterministic.

The exact fee recipient/distribution rule must be finalized before block
execution is implemented.

No implementation may silently introduce fee economics that are not
defined by the protocol.

### 6.8 Signature

The initial transaction signature algorithm is:

    Ed25519

The signature size is:

    64 bytes

The signature proves authorization by the sender's corresponding
private key.

An invalid signature makes the transaction invalid.

Private keys must never be transmitted as part of a transaction.

---

## 7. Transaction Signing

The initial transaction signing protocol uses Ed25519.

A transaction is signed over a deterministic domain-separated message.

The canonical unsigned transaction contains:

    version
    chain_id
    sender
    recipient
    amount
    nonce
    fee

The signed message is:

    "SWASCHAIN_TX_V1"
        ||
    canonical_unsigned_transaction

Where:

    || = byte concatenation

The domain separator prevents the transaction signature from being
implicitly shared with unrelated signing contexts.

The signing process is conceptually:

    Transaction Fields
          |
          v
    Canonical Serialization
          |
          v
    Add Domain Separator
          |
          v
    Signed Message
          |
          v
    Ed25519 Sign
          |
          v
       Signature

The signature itself is not included in the signed message.

Every compatible implementation must reconstruct exactly the same
signed byte sequence before verifying the signature.

The exact byte-level encoding is defined in:

    src/docs/transaction-spec.md

---

## 8. Canonical Transaction Serialization

Consensus-critical transaction serialization must be deterministic.

The initial transaction serialization defines:

- Fixed field ordering
- Fixed-width integer representation
- Big-endian integer encoding
- Fixed-width address encoding
- Fixed-width signature encoding
- Explicit protocol version
- Explicit chain ID
- No ambiguous optional fields

The canonical unsigned transaction contains:

    version
    chain_id
    sender
    recipient
    amount
    nonce
    fee

The unsigned transaction size for version 1 is:

    93 bytes

The complete transaction including the signature is:

    157 bytes

The transaction specification is authoritative for exact byte layout.

No consensus-critical implementation may use language-specific object
serialization, unordered maps, implicit padding, locale-dependent
encoding, or other ambiguous serialization.

---

## 9. Transaction Hash

Every transaction has a deterministic transaction identifier.

The initial hash algorithm is:

    SHA-256

The transaction hash uses the domain separator:

    "SWASCHAIN_TX_HASH_V1"

The transaction hash is derived from the canonical complete transaction
representation according to the transaction specification.

Conceptually:

    Domain Separator
          ||
    Canonical Transaction Bytes
          |
          v
       SHA-256
          |
          v
    Transaction Hash

The transaction hash is used for:

- Transaction identification
- Duplicate detection
- Mempool tracking
- Block transaction commitments
- Transaction lookup
- Future explorer functionality

Identical canonical transactions must produce identical transaction
hashes.

---

## 10. Transaction Validation

A transaction must pass all required validation rules before it can
modify canonical blockchain state.

Initial validation includes:

1. Supported transaction version
2. Correct chain ID
3. Valid sender address
4. Valid recipient address
5. Valid canonical encoding
6. Valid Ed25519 signature
7. Correct sender nonce
8. Valid amount
9. Valid fee
10. Sufficient sender balance
11. No arithmetic overflow or underflow
12. No replay through a consumed nonce
13. No duplicate execution

Conceptually:

    Raw Transaction
          |
          v
    Decode / Encoding Validation
          |
          v
    Protocol Validation
          |
          v
    Signature Validation
          |
          v
    State Validation
          |
          v
    Valid Transaction

Any failed consensus-critical validation rule causes rejection.

---

## 11. Transaction Replay Protection

SwasChain must prevent previously authorized transactions from being
executed again.

Replay protection operates at multiple protocol boundaries.

### 11.1 Chain Domain Protection

The chain ID is included in the signed transaction message.

Therefore, a transaction signed for one chain is not automatically valid
on another chain with a different chain ID.

### 11.2 Nonce Protection

The sender nonce prevents the same transaction sequence number from
being consumed repeatedly.

Example:

    Account nonce = 7

The next valid transaction must contain:

    nonce = 7

After successful execution:

    Account nonce = 8

A second transaction attempting to execute with:

    nonce = 7

must be rejected.

### 11.3 Transaction Identity

The transaction hash provides deterministic transaction identity for
duplicate detection.

Nodes must not treat transaction hash tracking as a replacement for
nonce validation.

Nonce validation remains part of consensus-critical state validation.

---

## 12. Transaction Ordering

Transactions from the same account are strictly ordered by nonce.

For example:

    Transaction A -> nonce 5
    Transaction B -> nonce 6
    Transaction C -> nonce 7

The protocol must not execute nonce 6 before nonce 5 for the same
account.

Transactions from different accounts may be ordered by the block
producer according to the deterministic block-construction rules.

The final transaction ordering policy must be deterministic and must
not depend on:

- Local hash-map ordering
- Network arrival timing alone
- Local machine behavior
- Unspecified implementation details

The consensus and block-engine specifications will define the exact
ordering algorithm before block production is considered final.

---

## 13. State Transition

Blockchain state changes only through valid transactions.

Conceptually:

    State Before
        +
    Valid Transaction
        =
    State After

For an initial native transfer:

    Sender Balance
        =
    Sender Balance - Amount - Fee

    Recipient Balance
        =
    Recipient Balance + Amount

    Sender Nonce
        =
    Sender Nonce + 1

The state transition must be atomic.

If validation or execution fails, no partial state mutation may become
part of canonical state.

All arithmetic must use checked operations.

The execution result must be deterministic across all compatible nodes.

---

## 14. Block Model

A SwasChain block contains a header and an ordered transaction set.

Conceptually:

    Block {
        header
        transactions
    }

The initial block header contains:

    BlockHeader {
        version
        height
        previous_block_hash
        timestamp
        transaction_root
        state_root
        consensus_data
    }

The exact binary block serialization will be defined before block
hashing and persistent block storage are finalized.

Block structure must remain deterministic across implementations.

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

For every valid non-genesis block:

    current.height = previous.height + 1

A block with an invalid height must be rejected.

---

## 16. Previous Block Hash

Every non-genesis block references the hash of its predecessor.

Conceptually:

    Block N
       |
       +-- previous_block_hash
                  |
                  v
             Hash(Block N-1)

This creates a cryptographically linked chain.

For a non-genesis block:

    current.previous_block_hash
        ==
    expected_previous_block_hash

must be satisfied.

A block referencing an incorrect predecessor must be rejected.

---

## 17. Block Timestamp

Each block contains a protocol-defined timestamp.

The initial timestamp representation and exact encoding must be
deterministic.

Consensus rules define acceptable timestamp relationships.

At minimum:

    block_timestamp >= parent_timestamp

A block timestamp must also satisfy the maximum permitted future drift
defined by the consensus specification.

The exact future-drift constant must be finalized before consensus
implementation is considered complete.

Consensus-critical execution must never use a node's local wall clock
as an input after a block has been accepted.

---

## 18. Transaction Root

The block header contains a cryptographic commitment to the ordered
transaction set.

Conceptually:

    Ordered Transactions
          |
          v
    Transaction Commitment
          |
          v
    Transaction Root

The transaction root allows nodes to verify that the transactions
represented by a block correspond to the header commitment.

The initial implementation may use a deterministic Merkle-tree-based
commitment.

The exact tree construction, leaf encoding, internal-node hashing,
empty-tree behavior, and ordering rules must be finalized before block
commitments are consensus-critical.

---

## 19. State Root

The block header contains a cryptographic commitment representing the
resulting blockchain state.

Conceptually:

    Previous State
          +
    Block Transactions
          |
          v
    Deterministic State Transition
          |
          v
       New State
          |
          v
      State Root

Every honest node independently executing the same valid block from the
same previous state must derive the same state root.

If the computed state root does not match the block's declared state
root, the block must be rejected.

The exact state commitment structure will be finalized with the state
and persistent-storage implementation.

---

## 20. Block Hash

A block hash is derived from the canonical block header representation.

Conceptually:

    Canonical Block Header
            |
            v
          SHA-256
            |
            v
        Block Hash

The exact domain-separated hashing construction will be defined by the
block and cryptographic specifications before implementation is locked.

The block hash must be deterministic.

Identical canonical block headers must produce identical block hashes.

---

## 21. Genesis Block

The genesis block is the first block in SwasChain history.

The genesis block has:

    height = 0

It does not reference a previous SwasChain block.

The genesis configuration must be deterministic and reproducible.

The genesis definition must specify:

- Network identifier
- Chain ID
- Initial state
- Initial accounts
- Initial balances, if any
- Protocol version
- Genesis timestamp
- Validator configuration
- Consensus configuration
- Genesis metadata

Every node operating on the same network must use the same genesis
configuration.

A node configured with an incompatible genesis configuration must not
participate in that network.

---

## 22. Block Validation

A node must validate a received block before accepting it.

Initial validation includes:

1. Valid block encoding
2. Supported block version
3. Correct block height
4. Correct previous block hash
5. Valid timestamp
6. Valid transaction commitment
7. Valid transaction ordering
8. Valid transactions
9. Correct state transition
10. Correct state root
11. Valid consensus metadata
12. Valid producer/validator authorization where required

A block failing any required consensus rule must be rejected.

---

## 23. Block Execution

A valid block is executed against the previous canonical state.

Conceptually:

    Previous State
          |
          v
    Decode Block
          |
          v
    Validate Header
          |
          v
    Validate Transactions
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
    Compare Commitments
          |
          v
    Consensus Acceptance
          |
          v
    Commit Canonical State

A node must not modify canonical state based solely on receiving a
block.

Canonical state changes only after all required validation,
execution, and consensus conditions have been satisfied.

---

## 24. Chain Selection

The initial development network uses the consensus protocol defined in:

    src/docs/consensus.md

A node must not accept a competing chain solely because it was received
from another peer.

Candidate blocks and chains must satisfy:

- Protocol validation
- State-transition validation
- Consensus rules
- Chain identity requirements
- Genesis compatibility

The final canonical-chain selection and finality rules are determined
by the consensus protocol.

---

## 25. Invalid Transactions

Invalid transactions must never modify canonical blockchain state.

Examples include:

- Invalid signature
- Incorrect chain ID
- Incorrect nonce
- Insufficient balance
- Invalid amount
- Invalid fee
- Invalid encoding
- Unsupported version
- Arithmetic overflow
- Replay of an already-consumed nonce

Depending on where the transaction is encountered, a node may:

- Reject it at RPC submission
- Reject it from the mempool
- Reject it during block validation

A transaction rejected at one stage must not bypass the same
consensus-critical validation at a later stage.

---

## 26. Invalid Blocks

An invalid block must never become part of the canonical chain.

Examples include:

- Wrong previous block hash
- Wrong block height
- Invalid transaction
- Invalid transaction root
- Invalid state root
- Invalid block encoding
- Invalid consensus metadata
- Invalid producer authorization
- Invalid timestamp
- Invalid transaction ordering

Nodes should record sufficient diagnostic information to support:

- Debugging
- Incident investigation
- Consensus testing
- Network observability

Diagnostic information must not alter consensus state.

---

## 27. Node Validation Boundary

SwasChain follows a strict validation boundary.

All externally supplied data is untrusted.

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

Parsing, validation, execution, and persistence must have clear
boundaries.

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
- Unspecified thread scheduling
- Unordered collection iteration

Protocol execution should use deterministic integer and byte-based
operations wherever possible.

All consensus-critical data structures must have deterministic
serialization and iteration behavior.

If concurrency is introduced in execution, the final state and all
consensus-visible results must remain deterministic.

---

## 29. Protocol Versioning

SwasChain protocol structures use explicit version information where
required.

Versioning exists to allow future protocol upgrades while maintaining
clear interpretation of historical data.

A node must reject unsupported protocol versions when required by
network rules.

Future protocol upgrades must define:

- New protocol version
- Compatibility rules
- Activation mechanism
- Migration behavior
- Historical interpretation
- Rollback or failure behavior where applicable

No consensus-critical protocol change should be silently introduced.

---

## 30. Smart Contract Compatibility

The initial protocol focuses on native account transactions.

The architecture reserves a future execution layer for smart contracts.

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

The future VM architecture is defined separately in:

    src/docs/vm.md

Smart contract execution must preserve the same core protocol
requirements:

- Determinism
- Verifiability
- Resource limits
- State integrity
- Consensus compatibility
- Canonical serialization
- Cross-node reproducibility

Smart contracts are not part of the initial native-transfer
implementation.

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

The transaction chain ID must match the network chain ID.

### Invariant 5

Every valid transaction signature must authenticate its sender.

### Invariant 6

Every non-genesis block references the correct previous block.

### Invariant 7

Block height increases sequentially.

### Invariant 8

All honest nodes executing the same valid block from the same previous
state must derive the same resulting state.

### Invariant 9

A block with an incorrect state root must be rejected.

### Invariant 10

An invalid transaction must never modify canonical state.

### Invariant 11

Consensus-critical serialization must be deterministic.

### Invariant 12

Untrusted network input must pass validation before entering
consensus-critical processing.

### Invariant 13

Consensus-critical arithmetic must not overflow or underflow.

### Invariant 14

A transaction signed for a different chain ID must not be accepted.

---

## 32. Initial Protocol Scope

The first implementation focuses on:

- Native account transfers
- Ed25519 transaction signatures
- SHA-256 hashing
- Chain IDs
- Addresses
- Nonces
- Transaction validation
- Deterministic state transitions
- Block construction
- Block validation
- Transaction commitments
- State commitments
- Persistent blockchain state
- Permissioned multi-node operation
- Consensus integration

The following are intentionally deferred:

- Advanced smart contracts
- SwasVM bytecode execution
- Gas/resource metering
- Parallel execution
- Advanced validator economics
- Permissionless validator admission
- Developer SDKs
- Advanced networking transports

This staged approach keeps the initial protocol small enough to verify
while preserving a clear path toward a more capable blockchain system.

---

## 33. Specification Authority

SwasChain uses layered protocol specifications.

The documents have distinct responsibilities:

### Architecture

Defines system-level components and their relationships.

### Protocol

Defines high-level consensus-critical behavior and invariants.

### Transaction Specification

Defines exact transaction fields, cryptographic algorithms,
serialization, signing, hashing, and wire-format requirements.

### Consensus Specification

Defines validator behavior, block proposal, voting, finality,
rounds, and fork handling.

### Networking Specification

Defines peer communication, message types, synchronization, and
network-level validation.

### VM Specification

Defines the future deterministic smart-contract execution boundary.

If two specifications conflict, the conflict must be resolved explicitly
before implementation continues.

No implementation should silently choose one conflicting rule.

---

## 34. Implementation Rule

The Rust implementation must follow the protocol specifications.

If implementation behavior conflicts with a consensus-critical rule:

1. The implementation must be corrected, or
2. The specification must be explicitly updated.

Protocol changes must be:

- Documented
- Reviewed
- Tested
- Reflected in affected specifications
- Represented by a meaningful Git commit

Consensus-critical behavior must never be changed silently.

---

## 35. Current Status

SwasChain is currently in:

**Phase 1 — Architecture & Protocol Design**

The following design documents have been established:

- Architecture specification
- Protocol specification
- Transaction specification
- Consensus specification
- Networking specification
- SwasVM architecture specification

The protocol is intentionally being reviewed for cross-document
consistency before consensus-critical implementation begins.

The next stage is to audit all specifications together and resolve any
remaining contradictions or undefined consensus-critical parameters.

No consensus-critical implementation should be considered final until
the corresponding protocol behavior has been explicitly defined,
implemented, and tested.
## Protocol Invariants

SwasChain protocol rules should preserve a set of fundamental invariants across all valid state transitions.

### State Consistency

Every accepted state transition must produce a deterministic and internally consistent result.

### Transaction Validity

A transaction must satisfy the protocol's validation requirements before it can affect blockchain state.

### Block Validity

A block must satisfy the defined structural, ordering, and validation rules before it can be accepted by a node.

### Consensus Consistency

Nodes following the protocol should be able to independently verify whether proposed blocks satisfy the consensus rules.

### Deterministic Execution

Given the same valid input state and transaction set, protocol execution should produce the same resulting state.

These invariants provide a foundation for reliable validation and help keep protocol behavior consistent across participating nodes.