# SwasChain Architecture

## 1. Overview

SwasChain is a custom Layer-1 blockchain designed as a modular, deterministic, and verifiable distributed system.

The goal is to build the blockchain protocol from first principles, including:

- Cryptographic transactions
- Deterministic state transitions
- Block production
- Transaction validation
- Persistent storage
- Peer-to-peer networking
- Multi-node synchronization
- Consensus
- RPC and CLI interfaces
- Observability and security testing

SwasChain is being developed as an engineering and research project.

The protocol will prioritize:

- Correctness
- Determinism
- Security
- Testability
- Modularity
- Measurable performance

The project will avoid artificial complexity and unsupported performance claims.

---

## 2. High-Level Architecture

SwasChain consists of several cooperating subsystems.

The high-level flow is:

SWASCHAIN NODE

    RPC
     |
    CLI
     |
    P2P
     |
     v
  NODE ENGINE
     |
     +-------------------+-------------------+
     |                   |                   |
     v                   v                   v
  MEMPOOL            CONSENSUS             STATE
     |                   |                   |
     +-------------------+-------------------+
                         |
                         v
                  BLOCK ENGINE
                         |
                  +------+------+
                  |             |
                  v             v
             TRANSACTIONS     BLOCKS
                  |             |
                  +------+------+
                         |
                         v
                      STORAGE


The architecture is intentionally modular so that individual components
can be developed and tested independently.

---

## 3. Core Design Principles

SwasChain follows several core engineering principles.

### 3.1 Determinism

Given the same valid input state and transaction sequence, every honest
node must produce the same resulting state.

Consensus-critical execution must not depend on machine-specific
behavior.

### 3.2 Verifiability

Nodes must independently verify:

- Transactions
- Digital signatures
- Blocks
- State transitions
- Consensus messages
- Cryptographic commitments

A node should not blindly trust data received from another node.

### 3.3 Modularity

Major subsystems should have clear interfaces.

This allows individual components to be:

- Tested independently
- Replaced
- Optimized
- Extended

without unnecessarily rewriting the entire system.

### 3.4 Explicit State Transitions

State changes must happen through well-defined protocol rules.

Hidden application logic must not modify consensus-critical state.

### 3.5 Security by Validation

All externally supplied data must be treated as untrusted until it has
passed the appropriate validation rules.

### 3.6 Measurable Performance

Performance claims must be based on reproducible benchmarks.

The project will avoid theoretical or marketing-based TPS claims.

---

## 4. Node Architecture

Each SwasChain node will contain the following major components:

- RPC Layer
- CLI
- Mempool
- Consensus
- State Engine
- Block Engine
- Storage
- P2P Networking

The node can be conceptually represented as:

    +-----------------------------+
    |          RPC / CLI          |
    +--------------+--------------+
                   |
                   v
    +-----------------------------+
    |        NODE ENGINE          |
    +-----------------------------+
       |        |        |       
       v        v        v
    Mempool  Consensus  State
       |        |        |
       +--------+--------+
                |
                v
         Block Engine
                |
                v
             Storage

P2P networking connects the node to other SwasChain nodes.

---

## 5. RPC Layer

The RPC layer provides external access to node functionality.

Planned responsibilities include:

- Query blockchain status
- Query blocks
- Query transactions
- Query accounts
- Submit transactions
- Query network information

The RPC layer must never bypass protocol validation.

For example, an RPC request submitting a transaction should eventually
follow the same validation rules as a transaction received through the
network.

---

## 6. CLI

The CLI will provide an interface for developers and node operators.

Planned commands include:

    swaschain status

    swaschain block

    swaschain tx

    swaschain account

    swaschain node

The CLI will eventually communicate with the node through the RPC
interface where appropriate.

The CLI should remain a thin interface over protocol functionality
rather than containing consensus-critical logic.

---

## 7. Mempool

The mempool stores valid but unconfirmed transactions waiting for
inclusion in a block.

Responsibilities include:

- Transaction validation
- Signature verification
- Duplicate detection
- Nonce handling
- Transaction prioritization
- Capacity limits
- Removal of confirmed transactions

The mempool must never assume that a transaction is valid simply
because it was received from another node.

A transaction entering the mempool must satisfy the protocol's
validation rules.

Conceptually:

    Transaction
         |
         v
    Validation
         |
     +---+---+
     |       |
   Invalid  Valid
     |       |
   Reject   Mempool
             |
             v
        Block Builder

---

## 8. Consensus

Consensus determines how participating nodes agree on the canonical
block sequence.

The consensus design will explicitly define:

- Validator roles
- Block proposal
- Voting
- Finality
- Validator membership
- Failure assumptions
- Message validation
- Network partitions
- Conflicting proposals
- Validator behavior

Consensus will be implemented only after the underlying transaction,
state transition, and block validation rules are stable.

The exact consensus mechanism will be selected based on clearly defined
requirements and failure assumptions rather than added only for
complexity.

---

## 9. State Engine

The state engine maintains the current blockchain state.

A state transition can be represented conceptually as:

    State(n+1) = Execute(State(n), Transactions)

The execution process must be deterministic.

The initial state model is expected to maintain information such as:

- Account balances
- Account nonces
- Protocol metadata

The architecture will later allow smart contract state to be added.

A valid state transition follows:

    Old State
         +
    Valid Transaction
         =
    New State

The resulting state commitment will eventually be represented in the
block structure.

---

## 10. Block Engine

The block engine is responsible for constructing and validating blocks.

A block is expected to contain protocol-defined metadata such as:

- Block height
- Previous block hash
- Timestamp
- Transaction commitment
- State commitment
- Consensus metadata

The exact block format will be defined in the protocol specification.

The simplified block flow is:

    Transactions
         |
         v
      Mempool
         |
         v
    Block Builder
         |
         v
    Candidate Block
         |
         v
    Block Validation
         |
         v
      Consensus
         |
         v
    State Execution
         |
         v
    State Commitment
         |
         v
    Persistent Storage

---

## 11. Cryptographic Layer

Cryptographic functionality will be isolated from higher-level
blockchain logic.

The cryptographic layer is expected to provide:

- Cryptographic hashing
- Public/private key handling
- Digital signatures
- Signature verification
- Transaction signing
- Transaction authentication
- Merkle-style commitments

SwasChain will use established and reviewed cryptographic algorithms.

The project will not invent custom cryptographic primitives.

Cryptographic operations must have dedicated tests covering both valid
and invalid inputs.

---

## 12. Transaction Flow

The expected transaction lifecycle is:

    User
      |
      v
    Create Transaction
      |
      v
    Sign Transaction
      |
      v
    Submit Transaction
      |
      v
    Validate Transaction
      |
      v
    Mempool
      |
      v
    Block Proposal
      |
      v
    Block Validation
      |
      v
    Consensus
      |
      v
    State Transition
      |
      v
    Persistent Storage
      |
      v
    Finalized Block

A transaction should not modify blockchain state directly.

It must pass through the defined validation and execution pipeline.

---

## 13. State Model

The initial SwasChain state model will use an account-based
representation.

Conceptually:

    Account
    |
    +-- Address
    |
    +-- Balance
    |
    +-- Nonce

The account model may later be extended for smart contracts.

The nonce will provide replay protection and enforce transaction
ordering rules for an account.

State transitions must remain deterministic.

For identical:

    Previous State
    +
    Block
    +
    Protocol Rules

all honest nodes must obtain the same:

    New State

The protocol will eventually define a state commitment that allows
nodes to verify that the resulting state is consistent with the block.

---

## 14. P2P Networking

SwasChain nodes will communicate through a peer-to-peer network.

The networking layer will eventually support:

- Peer discovery
- Peer identity
- Handshake
- Peer authentication
- Transaction propagation
- Block propagation
- Chain synchronization

Conceptually:

    Peer Discovery
          |
          v
       Handshake
          |
          v
    Peer Authentication
          |
          +-------------------+
          |                   |
          v                   v
    Transaction           Block
    Propagation         Propagation
          |                   |
          +---------+---------+
                    |
                    v
             Chain Sync

All received network messages are untrusted input.

Network messages must be validated before being passed to
consensus-critical components.

---

## 15. Testing, Security and Development Philosophy

SwasChain will be developed incrementally.

Each major subsystem should follow this lifecycle:

    Design
      |
      v
    Implementation
      |
      v
    Unit Tests
      |
      v
    Integration Tests
      |
      v
    Failure Testing
      |
      v
    Security Testing
      |
      v
    Documentation
      |
      v
    Benchmarking
      |
      v
    Integration

Testing will happen at multiple levels.

### Unit Tests

Individual functions and modules will be tested independently.

### Integration Tests

Interactions between major blockchain components will be tested.

### Multi-Node Tests

Multiple independent nodes will be used to test:

- Block propagation
- Transaction propagation
- Synchronization
- State consistency

### Adversarial Tests

The system will eventually test cases including:

- Invalid signatures
- Invalid nonces
- Duplicate transactions
- Invalid previous block hashes
- Invalid state commitments
- Malformed network messages
- Conflicting blocks
- Corrupted storage
- Invalid consensus messages

### Benchmarks

Performance will eventually be measured for:

- Transaction validation
- Signature verification
- Block validation
- State transitions
- Block construction
- Storage operations
- Network synchronization

No performance number will be presented as a protocol capability
without a reproducible benchmark.

---

# Development Roadmap

SwasChain will be developed through the following major phases.

## Phase 1 — Architecture & Protocol Design

Define:

- System architecture
- Transaction model
- Block model
- State model
- Cryptographic requirements
- Networking model
- Storage model
- Consensus assumptions
- Future VM interfaces

## Phase 2 — Rust Foundation

Build:

- Core Rust module structure
- Shared types
- Serialization
- Error handling
- Traits
- Basic testing infrastructure

## Phase 3 — Cryptography

Implement and test:

- Hashing
- Key handling
- Digital signatures
- Signature verification
- Transaction authentication
- Merkle commitments

## Phase 4 — Wallet & Transactions

Implement:

- Wallet generation
- Addresses
- Transaction creation
- Transaction signing
- Transaction verification
- Nonce handling
- Replay protection

## Phase 5 — State Machine

Implement:

- Account state
- Balances
- Nonces
- Deterministic state transitions
- State validation
- State commitments

## Phase 6 — Block Engine & Mempool

Implement:

- Transaction pool
- Transaction selection
- Block construction
- Block validation
- Transaction commitments
- State commitments
- Block linking

## Phase 7 — Persistent Storage

Implement durable storage for:

- Blocks
- Transactions
- State
- Metadata
- Chain information

The node must be able to restart without losing its blockchain state.

## Phase 8 — P2P Networking

Implement:

- Peer identity
- Handshake
- Peer discovery
- Transaction propagation
- Block propagation
- Network message validation

## Phase 9 — Multi-Node Synchronization

Implement:

- Chain height exchange
- Missing block requests
- Block synchronization
- Validation during synchronization
- State consistency checks

## Phase 10 — Consensus

Implement and test the selected validator consensus protocol.

The design will explicitly document:

- Validator assumptions
- Failure model
- Proposal rules
- Voting rules
- Finality
- Conflicting blocks
- Network partitions

## Phase 11 — RPC & CLI

Expose functionality for:

- Node status
- Blocks
- Transactions
- Accounts
- Network information
- Transaction submission

## Phase 12 — Security & Adversarial Testing

Attack the system using:

- Invalid transactions
- Invalid signatures
- Replay attempts
- Invalid blocks
- Invalid state commitments
- Malformed network messages
- Conflicting consensus messages
- Storage corruption scenarios

## Phase 13 — Explorer & Observability

Build:

- Block explorer
- Transaction explorer
- Account views
- Validator information
- Network information
- Node metrics
- Logs
- Monitoring

## Phase 14 — Benchmarking & Release

Produce reproducible measurements for:

- Transaction validation
- Block validation
- State execution
- Storage performance
- Synchronization performance
- Memory usage

Prepare:

- Documentation
- Security documentation
- Architecture documentation
- Protocol specification
- Benchmark reports
- Release notes

The first release will only be considered ready after the implemented
features are tested and documented.

---

## Current Status

SwasChain is currently in:

**Phase 1 — Architecture & Protocol Design**

The Rust project foundation has been initialized.

The protocol specification, transaction model, block model, state model,
networking model, storage model, and consensus assumptions are still
being defined.

Implementation should proceed only after the relevant protocol rules
are sufficiently specified.
## Design Principles

SwasChain architecture is guided by a few core principles:

- **Modularity:** Protocol components should have clear responsibilities and well-defined boundaries.
- **Determinism:** State transitions and validation rules should produce predictable results across nodes.
- **Verifiability:** Important protocol operations should be independently verifiable by participating nodes.
- **Extensibility:** The architecture should allow individual components to evolve without requiring a complete redesign of the protocol.
- **Explicit Specifications:** Protocol behavior should be documented before or alongside implementation.

These principles provide a foundation for implementing the protocol while keeping the system understandable and maintainable.