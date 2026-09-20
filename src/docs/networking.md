# SwasChain Networking Specification

**Status:** Draft v0.1  
**Layer:** P2P Networking  
**Last Updated:** 2026-09-20

---

## 1. Purpose

This document defines the peer-to-peer networking protocol of SwasChain.

The networking layer is responsible for allowing independent SwasChain nodes to:

- discover and authenticate peers
- establish reliable connections
- exchange protocol messages
- propagate transactions
- propagate blocks
- synchronize missing blockchain data
- recover from temporary network failures
- enforce network-level resource limits

Networking must transport protocol data reliably, but must not determine whether that data is valid.

Consensus, block validation, transaction validation, and state execution remain responsibilities of higher layers.

---

## 2. Networking Goals

The initial networking implementation has the following goals:

1. Reliable node-to-node communication
2. Deterministic protocol message formats
3. Authenticated peer identities
4. Transaction propagation
5. Block propagation
6. Blockchain synchronization
7. Connection failure recovery
8. Protocol version compatibility
9. Resource and message-size limits
10. Protection against basic network-level abuse

The networking layer must remain modular so that the underlying transport can evolve without changing the blockchain protocol itself.

---

## 3. Initial Network Model

SwasChain initially operates as a permissioned development network.

The initial network consists of a known set of validator and full-node identities.

Peer discovery is initially based on explicitly configured bootstrap peers.

Example:

    Node A
      |
      +---- Node B
      |
      +---- Node C
      |
      +---- Node D

Nodes may maintain multiple peer connections.

A node must never assume that a single peer represents the canonical chain.

All received blockchain data must be independently validated.

---

## 4. Node Identity

Every network node has a persistent cryptographic identity.

A node identity contains:

- node public key
- node identifier
- supported protocol version
- supported network identifier
- network endpoint information

The node identifier is derived deterministically from the node public key.

Conceptually:

    node_id = SHA256(node_public_key_bytes)

The node identity is different from an account address.

### Account identity

Used for transaction authorization.

### Node identity

Used for network peer authentication.

These identities must not be interchangeable.

---

## 5. Chain Identifier

Every network message belongs to a specific SwasChain network.

The network is identified using a `chain_id`.

Nodes must reject peers or protocol messages belonging to an incompatible chain.

This prevents accidental communication between unrelated SwasChain networks.

Example:

    chain_id = 1

Development, test, and future production networks must use distinct identifiers.

---

## 6. Transport Layer

The networking architecture separates transport from the SwasChain message protocol.

The initial implementation may use a reliable stream-oriented transport.

The first implementation target is TCP.

Conceptually:

    SwasChain Message
          ↓
    Message Codec
          ↓
    Framing Layer
          ↓
    TCP Connection
          ↓
    Network

TCP provides reliable ordered byte delivery.

It does not provide blockchain-level validation, authentication, consensus, or message semantics.

Those responsibilities belong to SwasChain.

Future versions may support alternative transports without changing the logical message protocol.

---

## 7. Connection Establishment

A peer connection follows this general lifecycle:

    CONNECTING
        ↓
    HANDSHAKE
        ↓
    AUTHENTICATED
        ↓
    ACTIVE
        ↓
    CLOSING
        ↓
    CLOSED

A connection must not exchange normal protocol messages before the handshake succeeds.

---

## 8. Handshake

When two nodes connect, they exchange handshake information.

The handshake must include:

- protocol version
- chain ID
- node ID
- node public key
- supported capabilities
- current block height
- latest known block hash
- network endpoint information

Conceptual handshake:

    HANDSHAKE {
        protocol_version
        chain_id
        node_id
        public_key
        capabilities
        block_height
        latest_block_hash
    }

The receiving node validates:

1. protocol compatibility
2. chain ID
3. node ID derivation
4. public-key consistency
5. message size
6. supported capabilities

A failed handshake results in connection termination.

---

## 9. Peer Authentication

Peer authentication establishes that a node controls the private key corresponding to its advertised public key.

The initial protocol uses cryptographic challenge-response authentication.

Conceptual flow:

    Node A → challenge → Node B

    Node B → signature(challenge) → Node A

    Node A → verifies signature

The same process is performed in the opposite direction when mutual authentication is required.

A peer must not be trusted merely because it claims a particular node ID.

The node ID must be cryptographically linked to the public key.

---

## 10. Message Framing

TCP provides a byte stream rather than message boundaries.

Therefore SwasChain requires an explicit message framing layer.

Each network message must contain a deterministic frame.

Conceptual format:

    +----------------+
    | Message Length |
    +----------------+
    | Message Type   |
    +----------------+
    | Protocol Ver.  |
    +----------------+
    | Payload        |
    +----------------+

The exact binary encoding will be finalized during implementation.

The frame must allow a receiver to determine:

- total message size
- message type
- protocol version
- payload boundaries

---

## 11. Maximum Message Size

Every incoming network message must have a maximum permitted size.

A node must reject oversized messages before allocating unbounded memory.

Limits must exist independently for:

- handshake messages
- transaction messages
- block messages
- synchronization requests
- synchronization responses
- consensus messages

The exact byte limits will be finalized during implementation and benchmark testing.

Resource limits are part of the security boundary.

---

## 12. Network Message Types

The initial protocol defines the following logical message categories.

### Connection messages

- HANDSHAKE
- HANDSHAKE_ACK
- PING
- PONG
- DISCONNECT

### Transaction messages

- TX_ANNOUNCE
- TX_REQUEST
- TX_RESPONSE

### Block messages

- BLOCK_ANNOUNCE
- BLOCK_REQUEST
- BLOCK_RESPONSE

### Synchronization messages

- STATUS_REQUEST
- STATUS_RESPONSE
- BLOCK_RANGE_REQUEST
- BLOCK_RANGE_RESPONSE

### Consensus messages

- PREVOTE
- PRECOMMIT
- ROUND_CHANGE

The wire representation may change during implementation, but message semantics must remain explicit.

---

## 13. Transaction Propagation

A node may receive a transaction from:

- local RPC
- local CLI
- another peer

After receiving a peer transaction, the node must perform validation before accepting it into the mempool.

Conceptual flow:

    Peer
      ↓
    TX_ANNOUNCE
      ↓
    Request transaction
      ↓
    Receive transaction
      ↓
    Decode
      ↓
    Validate
      ↓
    Mempool
      ↓
    Propagate

A node must not blindly forward an invalid transaction.

---

## 14. Transaction Announcement

To reduce unnecessary bandwidth, nodes may first announce transaction hashes rather than transmitting complete transactions.

Example:

    TX_ANNOUNCE {
        transaction_hashes[]
    }

A receiving node determines which transactions it does not already possess.

It may then request those transactions.

This reduces duplicate transaction transmission.

---

## 15. Transaction Deduplication

Nodes must maintain a bounded record of recently observed transaction hashes.

If a transaction hash has already been processed, the node should avoid repeatedly processing or propagating it.

Deduplication state must be bounded to prevent unbounded memory growth.

A transaction may be removed from the recent-seen cache after an appropriate expiration period.

---

## 16. Block Propagation

When a node accepts a valid new block, it may announce the block to peers.

Conceptual flow:

    Block Producer
          ↓
       New Block
          ↓
    Local Validation
          ↓
       Storage
          ↓
    BLOCK_ANNOUNCE
          ↓
        Peers

Peers may request the complete block after receiving its announcement.

---

## 17. Block Announcement

A block announcement should contain enough information for a peer to determine whether the block may be relevant.

Example:

    BLOCK_ANNOUNCE {
        height
        block_hash
        previous_block_hash
    }

The announcement does not replace full block validation.

A node must independently validate the complete block before accepting it into its local chain.

---

## 18. Block Requests

A node may request a block by:

- block hash
- block height

Example:

    BLOCK_REQUEST {
        block_hash
    }

or:

    BLOCK_REQUEST {
        height
    }

The responding node must return the requested block if available.

A node must validate the returned block before storing or executing it.

---

## 19. Blockchain Synchronization

A newly started or temporarily disconnected node must be able to synchronize missing blockchain data.

Basic synchronization flow:

    Connect to peers
          ↓
    Handshake
          ↓
    Exchange status
          ↓
    Compare heights / hashes
          ↓
    Request missing blocks
          ↓
    Validate blocks
          ↓
    Execute state transitions
          ↓
    Persist blocks
          ↓
    Continue until synchronized

Synchronization must not bypass normal block validation.

---

## 20. Status Exchange

Nodes exchange chain status after establishing a connection.

Example:

    STATUS_RESPONSE {
        chain_id
        latest_height
        latest_block_hash
        finalized_height
        finalized_block_hash
    }

A node can use this information to determine whether synchronization may be required.

A higher reported height does not automatically mean that a peer has a valid canonical chain.

---

## 21. Block Range Synchronization

For multiple missing blocks, a node may request a contiguous block range.

Example:

    BLOCK_RANGE_REQUEST {
        start_height
        end_height
    }

The responding node must enforce:

- maximum range size
- maximum response size
- valid height boundaries

The receiving node validates every block individually.

A synchronization response must never be trusted solely because it came from an established peer.

---

## 22. Synchronization Ordering

Blocks must be processed in chain order.

For a block sequence:

    B100
      ↓
    B101
      ↓
    B102
      ↓
    B103

The receiving node must verify that:

    hash(B100) == previous_hash(B101)

and similarly for every subsequent block.

Missing parent blocks must be obtained before a child block can become executable.

---

## 23. Fork Handling

Networking may expose a node to competing blocks at the same height.

Example:

    B100
      |
      +---- B101A
      |
      +---- B101B

The networking layer must deliver both candidates to the validation and consensus layers.

Networking must not independently decide which chain is canonical.

Chain selection and finalization are consensus responsibilities.

---

## 24. Consensus Message Propagation

Consensus messages are propagated between authenticated validator nodes.

Initial consensus message types:

- PREVOTE
- PRECOMMIT
- ROUND_CHANGE

Each consensus message must contain sufficient information to identify:

- chain ID
- validator identity
- block height
- consensus round
- message type
- referenced block hash
- validator signature

Consensus messages must be independently authenticated and validated.

---

## 25. Consensus Message Replay Protection

Consensus messages must not be reusable across:

- different chains
- different heights
- different rounds
- different consensus phases

A message should therefore be bound to its:

    chain_id
    height
    round
    message_type
    block_hash

This prevents previously valid messages from being replayed in a different consensus context.

---

## 26. Peer Discovery

The initial implementation uses static bootstrap peers.

Configuration may contain:

    bootstrap_peers = [
        peer_1,
        peer_2,
        peer_3
    ]

After connecting to bootstrap peers, a node may learn additional peers through authenticated peer information.

Future versions may introduce:

- peer exchange
- discovery protocols
- persistent peer scoring
- dynamic peer selection
- NAT traversal

These are outside the initial networking scope.

---

## 27. Peer Connection Limits

A node must enforce limits on the number of active connections.

Separate limits may exist for:

- inbound connections
- outbound connections
- validator connections
- synchronization connections

This prevents a malicious peer from exhausting node resources through excessive connection attempts.

Exact limits will be determined through benchmarking and deployment requirements.

---

## 28. Rate Limiting

Network messages must be subject to rate limits.

Rate limiting may be applied per:

- peer
- message type
- connection
- IP address
- node identity

High-volume messages such as announcements and synchronization requests require stricter controls.

Rate limiting must not prevent normal consensus operation.

---

## 29. Backpressure

A slow peer must not be allowed to consume unlimited memory.

Each connection should have bounded:

- receive buffers
- send queues
- pending requests
- synchronization batches

If a peer continuously fails to consume data, the node may temporarily stop sending lower-priority messages or terminate the connection.

Consensus traffic must receive appropriate priority.

---

## 30. Request Tracking

Requests requiring responses should use request identifiers.

Example:

    request_id = 128

    BLOCK_REQUEST {
        request_id
        height
    }

    BLOCK_RESPONSE {
        request_id
        block
    }

The node must reject unexpected, duplicate, or expired responses.

Pending request state must be bounded.

---

## 31. Peer Misbehavior

The networking layer should detect and record protocol-level misbehavior.

Examples include:

- invalid handshake
- invalid authentication
- malformed frames
- oversized messages
- invalid request IDs
- excessive requests
- repeated duplicate traffic
- invalid protocol version
- incompatible chain ID

Repeated severe violations may result in temporary or permanent peer banning.

Consensus-level Byzantine behavior is handled by the consensus layer.

---

## 32. Peer Reputation

The initial version does not require a complex reputation system.

However, nodes should maintain basic peer health information.

Possible metrics include:

- successful connections
- failed handshakes
- response latency
- invalid messages
- synchronization failures
- connection stability

Future versions may use these metrics for peer selection.

Peer reputation must never replace cryptographic validation.

---

## 33. Connection Keepalive

Long-lived peer connections should use periodic liveness checks.

Conceptual flow:

    Node A → PING → Node B
    Node B → PONG → Node A

A peer that repeatedly fails to respond within the configured timeout may be considered unavailable.

Timeout values must be configurable and tested under realistic network conditions.

---

## 34. Connection Recovery

Network connections can fail because of:

- peer shutdown
- network interruption
- timeout
- process failure
- temporary congestion

A node should automatically attempt reconnection according to a bounded retry strategy.

The retry mechanism should use backoff to avoid repeatedly overwhelming an unavailable peer.

---

## 35. Protocol Versioning

Every handshake and network message is associated with a protocol version.

Example:

    protocol_version = 1

Nodes must determine compatibility before entering the ACTIVE state.

Future versions may support multiple protocol versions simultaneously during upgrades.

Protocol versioning must be independent from blockchain state versioning.

---

## 36. Capability Negotiation

Peers may advertise supported capabilities.

Example:

    capabilities = [
        transactions,
        blocks,
        sync,
        consensus
    ]

Future capabilities may include:

- snapshots
- light-client support
- smart-contract execution
- advanced synchronization

A node must never assume that a peer supports a capability that was not advertised.

---

## 37. Message Priority

Not all messages have equal urgency.

Initial conceptual priority:

    1. Consensus messages
    2. Block propagation
    3. Synchronization responses
    4. Transaction propagation
    5. Peer discovery / auxiliary messages

Priority must be implemented carefully so that lower-priority traffic cannot starve indefinitely.

Exact scheduling policy will be finalized during implementation.

---

## 38. Determinism Boundary

Networking itself does not need to be deterministic in:

- packet arrival order
- connection timing
- peer response timing
- network latency

However, blockchain state transitions must remain deterministic.

Therefore:

    Network
       ↓
    nondeterministic delivery
       ↓
    deterministic validation
       ↓
    deterministic execution
       ↓
    deterministic state

No consensus-critical state transition may depend on local network timing.

---

## 39. Security Boundary

The networking layer must assume that any remote peer can be malicious.

Therefore:

- never trust peer-provided state
- never trust peer-provided blocks
- never trust peer-provided transactions
- never trust peer-provided consensus messages
- never allocate unbounded memory from remote input
- never execute unvalidated protocol data

Every received object crosses a validation boundary before entering the corresponding subsystem.

---

## 40. Data Ownership

The networking layer transports data.

Ownership belongs to higher-level components.

Examples:

    Transaction
        ↓
    Transaction validation
        ↓
    Mempool

    Block
        ↓
    Block validation
        ↓
    Block engine

    Consensus message
        ↓
    Consensus validation
        ↓
    Consensus state machine

Networking must not duplicate business logic unnecessarily.

---

## 41. Initial Network Architecture

The initial implementation follows:

    +----------------------+
    |      RPC / CLI       |
    +----------+-----------+
               |
               v
    +----------------------+
    |     Node Runtime     |
    +----------+-----------+
               |
       +-------+-------+
       |               |
       v               v
    Mempool         Consensus
       |               |
       +-------+-------+
               |
               v
    +----------------------+
    |   Networking Layer   |
    +----------+-----------+
               |
       +-------+-------+
       |       |       |
       v       v       v
      Peer    Peer    Peer
       |       |       |
       +-------+-------+
               |
             TCP

The networking layer provides transport and peer management to the rest of the node.

---

## 42. Testing Requirements

Networking tests must include:

### Connection tests

- successful connection
- failed connection
- handshake timeout
- incompatible protocol
- incompatible chain ID
- invalid node identity

### Message tests

- valid frame
- malformed frame
- truncated frame
- oversized frame
- unknown message type
- invalid payload

### Propagation tests

- transaction propagation
- block propagation
- duplicate suppression
- missing transaction request
- missing block request

### Synchronization tests

- node behind by one block
- node behind by many blocks
- missing parent
- invalid block during synchronization
- competing chains
- interrupted synchronization

### Failure tests

- peer disconnect
- reconnect
- timeout
- slow peer
- unavailable peer
- repeated malformed messages

### Security tests

- replayed messages
- forged identity
- request flooding
- oversized payload
- invalid request ID
- excessive connection attempts

---

## 43. Adversarial Network Testing

The networking layer must eventually be tested against intentionally hostile peers.

Examples:

    Malicious Peer
          |
          +-- malformed frames
          +-- oversized messages
          +-- fake identity
          +-- replayed messages
          +-- request flooding
          +-- slow responses
          +-- invalid blocks
          +-- invalid consensus messages

The objective is not merely to prove that honest nodes communicate.

The objective is to verify that malicious network input cannot violate protocol invariants.

---

## 44. Observability

The networking layer should expose useful metrics and logs.

Initial metrics may include:

- active peer count
- inbound connections
- outbound connections
- messages received
- messages sent
- bytes received
- bytes sent
- transaction propagation latency
- block propagation latency
- synchronization duration
- failed handshakes
- rejected messages
- peer disconnects

Sensitive information must not be logged unnecessarily.

---

## 45. Implementation Boundary

The first networking implementation will focus on:

1. TCP transport
2. peer connection manager
3. node identity
4. handshake
5. message framing
6. basic message codec
7. ping/pong
8. transaction propagation
9. block propagation
10. basic synchronization

Advanced peer discovery, NAT traversal, dynamic reputation, snapshots, and alternative transports are deferred.

---

## 46. Future Networking Evolution

Future SwasChain versions may introduce:

- encrypted transport
- QUIC
- dynamic peer discovery
- peer exchange
- snapshots
- state synchronization
- light-client networking
- advanced peer scoring
- bandwidth-aware synchronization
- parallel block downloads
- secure NAT traversal
- specialized validator networking

These features must preserve the core protocol invariants.

---

## 47. Networking Invariants

The following invariants must always hold:

1. A peer cannot become active without a valid handshake.
2. A peer identity must be cryptographically authenticated.
3. A peer must use the correct chain ID.
4. Oversized messages must be rejected.
5. Network input must never bypass validation.
6. Invalid transactions must not enter the mempool.
7. Invalid blocks must not enter the canonical chain.
8. Consensus messages must be authenticated.
9. Consensus messages must be bound to height and round.
10. Synchronization must preserve block ordering.
11. Networking must not determine consensus.
12. Networking must not make state execution nondeterministic.
13. Remote input must not cause unbounded resource allocation.
14. Peer failures must not corrupt blockchain state.

---

## 48. Current Status

Networking Specification: Draft v0.1

Designed:

- peer identity
- chain identity
- transport model
- connection lifecycle
- handshake
- authentication
- message framing
- transaction propagation
- block propagation
- synchronization
- consensus message propagation
- peer discovery
- rate limiting
- backpressure
- failure recovery
- adversarial testing

Not yet implemented.

Implementation begins only after the Phase 1 protocol documents have been reviewed for consistency.