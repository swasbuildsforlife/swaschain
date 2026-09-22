# SwasChain Consensus Specification

Status: Draft v0.1
Protocol: SwasChain
Layer: Consensus
Last Updated: 2026-09-20


## 1. Purpose

This document defines the consensus model of SwasChain.

Consensus determines:

- who may propose blocks
- how blocks are validated
- how nodes agree on chain history
- how conflicting blocks are handled
- how block ordering is determined
- how forks are resolved
- how nodes recover after temporary network failures
- which data is considered consensus-critical

The consensus implementation must be deterministic.

All honest nodes following the same protocol rules must independently reach the same canonical chain under the protocol's stated network assumptions.


## 2. Initial Consensus Scope

SwasChain v0.1 uses a permissioned development network.

The initial network is intentionally limited so that the core blockchain architecture can be implemented and tested before introducing a fully permissionless validator economy.

Initial assumptions:

- validator identities are explicitly configured
- only authorized validators may produce blocks
- validators communicate over the SwasChain peer-to-peer network
- blocks are independently verified by every validator
- invalid blocks are rejected
- deterministic chain selection is required

This development consensus model is not the final consensus design of SwasChain.

Future versions may introduce a permissionless or stake-based validator model after the core protocol has been thoroughly tested.


## 3. Consensus Goals

The consensus layer must provide:

1. Deterministic block acceptance
2. Deterministic chain selection
3. Block producer authentication
4. Protection against invalid blocks
5. Protection against conflicting histories
6. Explicit validator identity
7. Recovery from temporary network partitions
8. Clear failure behavior
9. Testable consensus rules
10. A foundation for future validator mechanisms


## 4. Consensus Terminology

### Validator

A node authorized to participate in block production and consensus validation.

### Block Producer

The validator selected to propose a block at a particular height.

### Proposal

A block broadcast by the selected block producer.

### Vote

A validator message indicating whether a proposed block satisfies consensus rules.

### Height

The sequential position of a block in the blockchain.

### Finalized Block

A block that has satisfied the finalization rule of the active consensus protocol.

### Fork

A situation where multiple valid blocks or chains exist at the same or different heights.


## 5. Validator Identity

Each validator has a cryptographic identity.

The validator identity is separate from normal account ownership.

A validator key is used to authenticate consensus messages and block proposals.

Conceptually:

    Validator Private Key
            │
            ├── signs block proposals
            │
            └── signs consensus messages

    Validator Public Key
            │
            └── verifies validator signatures

Validator private keys must never be transmitted to peers.

Validator identities must be explicitly known to the development network.

## 6. Block Producer Selection

For the initial permissioned network, block producers are selected from
the configured validator set.

Let:

    validators = [V0, V1, V2, ... Vn]

The genesis block has:

    height = 0

The genesis block is not assigned a producer through the normal
round-robin schedule.

For every non-genesis block, the deterministic producer selection rule
is:

    producer(height) =
        validators[(height - 1) mod validator_count]

This means the first block after genesis is produced by V0.

Example:

    Validators:
        V0
        V1
        V2
        V3

    Height 1 -> V0
    Height 2 -> V1
    Height 3 -> V2
    Height 4 -> V3
    Height 5 -> V0
    Height 6 -> V1

The producer schedule is completely deterministic.

The producer selection must never depend on:

- Local randomness
- Network arrival order
- Local system time
- Mempool contents
- Operating system behavior
- Unspecified implementation details

The validator set used for producer selection must be the active
consensus validator set for the corresponding block height.

For the initial protocol, the validator set is static and defined by
the genesis consensus configuration.

If the validator set changes in a future protocol version, the
activation height and producer-selection behavior must be explicitly
defined as consensus rules.

A node must independently calculate the expected producer for every
block before accepting the proposal.


## 7. Proposal Eligibility

A validator may propose a block only when:

1. It is part of the active validator set.
2. It is the designated producer for the current height.
3. It is operating on the correct parent block.
4. The proposed block satisfies transaction and state rules.
5. The validator can produce a valid producer signature.

A node must reject a proposal from an unauthorized producer.


## 8. Block Production

The designated producer constructs a block using:

    Parent Block
        │
        ▼
    Current State
        │
        ├── Select valid transactions
        │
        ▼
    Execute transactions
        │
        ▼
    Calculate transaction root
        │
        ▼
    Calculate state root
        │
        ▼
    Construct block header
        │
        ▼
    Sign block proposal
        │
        ▼
    Broadcast block

The block producer must not modify transactions after they have been selected and validated.


## 9. Block Proposal Requirements

A valid block proposal must contain:

- valid block version
- valid height
- valid previous block hash
- valid timestamp
- valid transaction root
- valid state root
- valid consensus data
- valid transaction list
- valid producer identity
- valid producer signature

The block must reference exactly one parent block.


## 10. Block Producer Signature

The block producer must authenticate the proposal.

Conceptually:

    block_signature =
        Sign(
            validator_private_key,
            canonical_block_header
        )

The receiving node verifies:

    Verify(
        validator_public_key,
        canonical_block_header,
        block_signature
    )

A block with an invalid producer signature must be rejected.


## 11. Proposal Validation

When a validator receives a proposal, it must independently verify:

    1. Block version
    2. Block height
    3. Parent block hash
    4. Producer identity
    5. Producer eligibility
    6. Producer signature
    7. Timestamp rules
    8. Transaction validity
    9. Transaction ordering
    10. Transaction root
    11. State transition
    12. State root
    13. Consensus metadata

A node must never accept a block solely because another validator accepted it.


## 12. Height Validation

For a node whose current canonical tip has height:

    H

the next block must normally have:

    height = H + 1

A block with a height lower than the expected height is stale unless being processed as historical synchronization data.

A block with a height greater than H + 1 cannot normally be applied directly because its parent state is unavailable.


## 13. Parent Validation

Every non-genesis block must reference the hash of its immediate parent.

The node must verify:

    block.previous_block_hash == local_tip_hash

when extending the current canonical chain.

For synchronization, a node may temporarily receive blocks whose parents are not yet available.

Such blocks must remain uncommitted until the required parent chain has been validated.


## 14. Timestamp Rules

Block timestamps are consensus-relevant.

The timestamp must satisfy deterministic protocol bounds.

Initial rules:

    block_timestamp >= parent_timestamp

and:

    block_timestamp <= allowed_future_time

The exact maximum future drift must be defined in implementation configuration before production consensus deployment.

Local system clocks must not be treated as exact consensus truth.

Timestamp validation must therefore use explicit protocol tolerance.


## 15. Transaction Selection

The block producer may select transactions from its local mempool.

However, every selected transaction must independently pass block execution rules.

The producer must maintain deterministic transaction ordering rules.

Initial ordering preference:

1. sender nonce ordering
2. fee ordering where applicable
3. deterministic transaction hash tie-breaker

The exact ordering policy must be finalized before multiple independent implementations are expected to produce identical blocks.


## 16. Transaction Execution

Transactions are executed sequentially according to block order.

For every transaction:

    current_state
          │
          ▼
    validate transaction
          │
          ▼
    execute transaction
          │
          ▼
    updated_state

The resulting state becomes the input for the next transaction.

Any invalid transaction encountered during block execution causes the block to fail validation.


## 17. State Root

After all transactions execute, the node calculates the resulting state root.

Conceptually:

    State
      │
      ▼
    State Transition
      │
      ▼
    Final State
      │
      ▼
    State Root

The state root commits the block to the resulting blockchain state.

All honest validators must calculate the same state root from the same parent state and transaction sequence.


## 18. Transaction Root

The block contains a transaction root committing to the transactions included in the block.

Initial implementation may use a deterministic Merkle tree.

Conceptually:

    Transaction 1 ─┐
                   ├── Hash ─┐
    Transaction 2 ─┘         │
                             ├── Root
    Transaction 3 ─┐         │
                   ├── Hash ─┘
    Transaction 4 ─┘

The transaction root must be deterministic.

The exact Merkle tree construction rules must be finalized before implementation.


## 19. Block Hash

The block hash is derived from its canonical header representation.

Conceptually:

    block_hash =
        SHA256(
            domain_separator ||
            canonical_block_header
        )

The block hash must be deterministic across all honest nodes.


## 20. Block Finalization

The initial development network uses explicit validator agreement.

A block may be considered finalized when the protocol's configured validator threshold has been satisfied.

For the initial implementation, the consensus layer should use a two-thirds supermajority threshold:

    votes_for >= ceil(2 * validator_count / 3)

The exact vote counting and quorum rules must be implemented carefully so that the threshold is deterministic for every validator count.


## 21. Why a Supermajority Is Required

A simple majority is insufficient for a Byzantine fault-tolerant consensus design.

A two-thirds threshold provides a stronger quorum intersection property.

For the initial development network, the protocol assumes that fewer than one-third of validators are Byzantine.

This assumption is part of the security model and must be documented clearly.

The initial implementation is therefore a research and engineering system, not a claim of production-grade Byzantine fault tolerance.


## 22. Consensus Votes

Validators may produce consensus votes for a proposed block.

A vote conceptually contains:

    validator_id
    height
    round
    block_hash
    vote_type
    signature

The vote must be authenticated by the validator.

A validator must not be able to create an apparently valid vote for another validator.


## 23. Vote Types

The initial consensus design may distinguish:

    PREVOTE
    PRECOMMIT

A future implementation may use these phases to separate:

    proposal observation
    block validity agreement
    finalization

The exact state machine for these phases must be implemented and tested before consensus is considered complete.


## 24. Double Voting

A validator must not vote for conflicting blocks in the same consensus context.

A consensus context is identified by:

    height
    round
    vote_type

For example, a validator must not produce:

    Vote(height=10, round=0, PREVOTE, block=A)

and:

    Vote(height=10, round=0, PREVOTE, block=B)

Both votes being signed by the same validator constitutes conflicting behavior.

Nodes must detect conflicting votes.


## 25. Round

Consensus may require multiple rounds at the same height.

A round exists to recover when:

- the designated producer is offline
- a proposal is invalid
- messages are delayed
- validators cannot reach the required quorum

Conceptually:

    Height 10
        Round 0
        Round 1
        Round 2
        ...

A higher round must never silently replace a finalized block.


## 26. Round Change

A validator may move to a higher round when the current round cannot make progress.

A round-change mechanism must eventually allow validators to continue when a producer or network path fails.

Round-change messages must be authenticated.

A future implementation must define:

- timeout calculation
- round-change message format
- proposer selection
- locked block behavior
- quorum requirements


## 27. Locking

Once a validator has strong evidence that a block may be finalized, it must not freely switch to a conflicting block.

A locking mechanism prevents inconsistent validator behavior during competing proposals.

The exact lock/unlock rules must be defined together with the final consensus state machine.

This is essential before claiming Byzantine fault tolerance.


## 28. Forks

A fork occurs when two valid blocks compete at the same height.

Possible causes include:

- delayed network messages
- simultaneous proposals
- validator failure
- malicious behavior
- network partition

Validators must never resolve forks using arbitrary local preference.

The consensus protocol must define the canonical branch.


## 29. Finalized Blocks and Forks

Once a block satisfies the finalization threshold, honest validators must treat it as finalized.

A finalized block must not be replaced by a conflicting block under normal protocol operation.

If conflicting finalized blocks are ever observed, this indicates a serious protocol or implementation failure and must trigger explicit safety handling.


## 30. Chain Selection

Before finalization, nodes may maintain candidate branches.

Chain selection must be deterministic.

For the initial consensus design, finality is preferred over a simple longest-chain rule.

A branch containing a finalized block has priority over an unfinalized competing branch.

A future specification will define exact fork-choice behavior for all synchronization cases.


## 31. Validator Set

The initial validator set is configured at network initialization.

Conceptually:

    validator_set = {
        validator_1,
        validator_2,
        validator_3,
        ...
    }

Validator membership is consensus-critical.

Changing the validator set must therefore occur through a deterministic protocol mechanism.

The initial development version may use a static validator set from genesis.


## 32. Validator Set Changes

Dynamic validator membership is not part of the initial implementation.

Future versions may support:

- validator registration
- validator removal
- stake-based voting power
- validator rotation
- validator rewards
- validator penalties

Such changes must be processed as consensus state transitions.


## 33. Genesis Consensus Configuration

The genesis block must define the initial consensus configuration.

This may include:

    chain_id
    initial validators
    initial block height
    protocol version
    consensus parameters

All nodes joining the same network must use compatible genesis configuration.

Different genesis configurations represent different networks.


## 34. Network Partition

During a network partition, validators may become divided into groups.

The consensus protocol must prioritize safety over creating conflicting finalized histories.

A minority partition must not be able to independently finalize a conflicting chain if the quorum threshold cannot be satisfied.

When connectivity is restored, nodes must synchronize using finalized history and the defined fork-choice rules.


## 35. Offline Validator

If the designated block producer is unavailable:

1. Validators wait for the configured timeout.
2. The current round may expire.
3. Validators enter a higher round.
4. A new eligible producer is selected.
5. Consensus continues.

The timeout must be deterministic according to the protocol rules.


## 36. Malicious Validator

A malicious validator may attempt to:

- propose invalid blocks
- sign conflicting proposals
- double vote
- send malformed messages
- withhold proposals
- delay messages
- send conflicting messages to different peers

The consensus layer must validate all messages independently.

A signed message must not be trusted merely because it originates from a configured validator.


## 37. Byzantine Fault Model

The initial consensus design assumes:

    Byzantine validators < 1/3 of the active validator set

Under this assumption, the intended safety property is:

    honest validators should not finalize conflicting blocks

The intended liveness property is subject to:

- sufficient network connectivity
- functioning validators
- bounded message delays
- availability of enough honest validators


## 38. Safety

Consensus safety means:

    Two honest validators must not finalize conflicting blocks at the same height.

Safety depends on:

- authenticated validator identities
- quorum intersection
- vote rules
- locking rules
- deterministic validation
- correct implementation

Safety must be demonstrated through adversarial tests rather than assumed from documentation alone.


## 39. Liveness

Consensus liveness means:

    The network can eventually finalize new blocks when required validators are online and communication conditions permit progress.

Liveness must not be achieved by weakening safety rules.

Timeouts and round changes exist to recover from unavailable producers and temporary network problems.


## 40. Consensus Message Authentication

Every consensus message must contain enough information to determine:

- sender validator
- message type
- height
- round
- referenced block
- signature

Messages must be signed using the validator's consensus key.

Nodes must reject:

- invalid signatures
- unknown validators
- malformed messages
- messages from incorrect heights
- messages from invalid rounds


## 41. Consensus Message Replay Protection

Consensus messages must not be reusable across unrelated consensus contexts.

Messages must bind their signatures to:

    chain_id
    height
    round
    message_type
    block_hash
    validator_id

A message valid at one height must not automatically be valid at another height.


## 42. Determinism

Consensus-critical behavior must be deterministic.

It must not depend on:

- random local choices
- hash-map iteration order
- operating system behavior
- local filesystem state
- environment variables
- network arrival order when protocol ordering rules already exist
- floating-point arithmetic

Where multiple valid messages exist, the protocol must define deterministic handling.


## 43. Consensus State Machine

The consensus implementation should model explicit states.

Initial conceptual state machine:

    NEW_HEIGHT
        │
        ▼
    PROPOSE
        │
        ▼
    PREVOTE
        │
        ▼
    PRECOMMIT
        │
        ▼
    FINALIZED

Failure or timeout may cause:

    TIMEOUT
        │
        ▼
    ROUND_CHANGE
        │
        ▼
    PROPOSE

The exact transitions must be encoded explicitly in Rust rather than relying on implicit control flow.


## 44. Consensus Invariants

The following invariants must always hold:

1. Only an authorized validator may propose a block.
2. A validator must authenticate its consensus messages.
3. Invalid blocks must never be finalized.
4. Transactions must be independently validated during block execution.
5. A validator must not double vote in the same consensus context.
6. Finalized blocks must not be replaced by conflicting blocks.
7. Consensus decisions must be deterministic.
8. Chain ID must be included in consensus message domain separation.
9. Validator set membership must be consensus-defined.
10. Consensus state must survive temporary network failures.


## 45. Testing Requirements

Consensus must be tested under both normal and adversarial conditions.

### Unit tests

Test:

- producer selection
- validator identity verification
- block proposal validation
- vote validation
- quorum calculation
- round changes
- timeout handling
- fork detection
- finalized block handling

### Adversarial tests

Test:

- invalid producer
- invalid block signature
- invalid vote signature
- double voting
- conflicting proposals
- missing producer
- delayed messages
- duplicated messages
- reordered messages
- malformed messages
- minority validator partition
- conflicting finalized blocks

### Property tests

Important properties include:

    invalid block -> never finalized

    unauthorized producer -> rejected

    conflicting finalized blocks -> forbidden

    duplicate consensus message -> does not alter final state

    same consensus input -> same deterministic result


## 46. Observability

Consensus events should eventually expose structured logs such as:

    consensus_height
    consensus_round
    proposer
    proposal_received
    prevote_received
    precommit_received
    quorum_reached
    block_finalized
    round_changed
    validation_failure

Logs must never expose private keys or other secret material.


## 47. Initial Implementation Boundary

Version 0.1 consensus will initially focus on:

- static validator set
- deterministic producer rotation
- authenticated block proposals
- authenticated votes
- explicit rounds
- deterministic quorum calculation
- block validation
- basic finalization
- adversarial testing

It will not initially implement:

- staking
- validator rewards
- slashing
- dynamic validator membership
- delegated staking
- permissionless validator discovery


## 48. Future Consensus Evolution

Future versions may introduce:

- proof-of-stake
- dynamic validator sets
- validator economics
- staking
- slashing
- delegation
- optimized BFT networking
- weighted voting
- validator reputation mechanisms

Any such change must be specified as a new protocol version or a formally compatible consensus upgrade.


## 49. Security Philosophy

Consensus is not considered secure merely because normal blocks work.

SwasChain consensus must be evaluated by attempting to break it.

Development must include:

    build
    test
    attack
    observe failure
    fix
    retest

A consensus implementation that has only passed happy-path tests is not considered complete.


## 50. Current Status

Consensus specification:

    Defined as Draft v0.1

Validator implementation:

    Not started

Vote implementation:

    Not started

Round state machine:

    Not started

Finality implementation:

    Not started

Adversarial consensus testing:

    Not started


## 51. Next Engineering Step

After this document is committed, the remaining major Phase 1 protocol documents are:

    Networking Specification
    VM Interface Specification

After the Phase 1 specifications are stable, implementation can begin with the Rust foundation and strongly typed protocol primitives.
## Consensus Validation Flow

Consensus validation defines how a node evaluates a proposed block before accepting it as part of the canonical chain.

A simplified validation flow is:

1. **Receive Proposal**
   - The node receives a proposed block from a peer or block producer.

2. **Validate Structure**
   - Verify that the block contains the required fields and follows the defined block format.

3. **Validate Parent**
   - Confirm that the referenced parent block is known and consistent with the local chain.

4. **Validate Transactions**
   - Verify that transactions included in the block satisfy the transaction and execution rules.

5. **Verify Consensus Requirements**
   - Check that the proposal satisfies the protocol's consensus-specific requirements.

6. **Accept or Reject**
   - A valid proposal may be accepted for further processing.
   - An invalid proposal is rejected and must not modify the canonical state.

The validation process allows participating nodes to independently verify proposed blocks rather than trusting the block producer.