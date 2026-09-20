# SwasVM Specification

**Status:** Draft v0.1  
**Layer:** Execution / Smart Contracts  
**Last Updated:** 2026-09-20

---

## 1. Purpose

SwasVM is the future deterministic execution environment of SwasChain.

Its purpose is to allow SwasChain nodes to execute programmable state transitions while guaranteeing that every honest node reaches the same result from the same blockchain state and transaction input.

SwasVM is not part of the initial SwasChain implementation.

This document defines the architectural boundary and protocol requirements that the future VM must satisfy.

---

## 2. Design Goals

SwasVM must provide:

1. Deterministic execution
2. Explicit state access
3. Resource accounting
4. Contract isolation
5. Versioned execution rules
6. Bounded computation
7. Deterministic errors
8. Cryptographically verifiable state transitions
9. Safe interaction with native blockchain functionality
10. A stable interface between consensus and contract execution

The VM must never introduce nondeterministic behavior into consensus-critical execution.

---

## 3. Execution Model

At a high level:

    Transaction
         ↓
    Transaction Validation
         ↓
    Execution Engine
         ↓
    SwasVM
         ↓
    State Reads
         ↓
    Contract Execution
         ↓
    State Writes
         ↓
    Execution Result
         ↓
    State Root

The VM executes within the deterministic state-transition engine.

Consensus determines which valid block is executed.

The VM determines the state changes produced by valid executable transactions.

---

## 4. VM Boundary

The VM must have a clearly defined boundary.

Outside the VM:

- networking
- peer management
- consensus
- block production
- block storage
- RPC transport
- wallet management

Inside the VM:

- contract execution
- contract state access
- execution context
- resource accounting
- contract calls
- deterministic host functions

Conceptually:

    Consensus
        |
        v
    Block Engine
        |
        v
    State Transition Engine
        |
        v
      SwasVM
        |
        +---- Contract
        |
        +---- Contract State
        |
        +---- Host Interface

---

## 5. Determinism Requirement

The most important SwasVM invariant is deterministic execution.

Given:

    same blockchain state
    +
    same transaction
    +
    same block context
    +
    same VM version

every honest node must produce:

    same execution result
    +
    same state changes
    +
    same resulting state root

The VM must not depend on local machine behavior.

---

## 6. Forbidden Nondeterminism

Smart contracts must not directly access nondeterministic system resources.

The VM must prohibit direct access to:

- operating system clocks
- random system sources
- filesystem
- network sockets
- environment variables
- local process state
- external APIs
- host-specific hardware
- thread scheduling
- unbounded system resources

If a contract needs information from outside the VM, that information must be provided through a deterministic protocol-defined mechanism.

---

## 7. Block Execution Context

Contract execution occurs inside a deterministic block context.

The context may expose:

- chain ID
- block height
- block hash
- block timestamp
- transaction hash
- sender address
- transaction nonce
- remaining execution budget
- VM version

Example:

    ExecutionContext {
        chain_id
        block_height
        block_hash
        block_timestamp
        transaction_hash
        sender
        nonce
        resource_limit
        vm_version
    }

Only protocol-approved context values may be exposed to contracts.

---

## 8. State Access

Contracts interact with blockchain state through the VM state interface.

Conceptually:

    VM
     |
     +---- read(key)
     |
     +---- write(key, value)
     |
     +---- delete(key)

The VM must not allow contracts to directly access the underlying database.

All state operations pass through a controlled host interface.

---

## 9. Contract Storage

Each contract has isolated persistent storage.

Conceptually:

    Contract Address
          |
          +---- key_1 → value_1
          +---- key_2 → value_2
          +---- key_3 → value_3

A contract must not arbitrarily modify another contract's storage.

Cross-contract state interaction must occur through explicit protocol-defined calls.

---

## 10. State Namespacing

Contract storage should use a deterministic namespace.

Conceptually:

    state_key =
        contract_address
        ||
        storage_key

This prevents storage collisions between independent contracts.

The exact encoding will be finalized during VM implementation.

---

## 11. Execution Environment

The VM executes contracts inside an isolated logical environment.

Conceptually:

    +-------------------------+
    |       SwasVM            |
    |                         |
    |  +-------------------+  |
    |  | Contract Runtime  |  |
    |  +-------------------+  |
    |                         |
    |  +-------------------+  |
    |  | Host Interface    |  |
    |  +-------------------+  |
    |                         |
    |  +-------------------+  |
    |  | Resource Meter     |  |
    |  +-------------------+  |
    +-------------------------+

The runtime must not obtain unrestricted access to the host operating system.

---

## 12. Contract Code

Future SwasChain versions may support compiled contract bytecode.

The exact bytecode format is intentionally not fixed in v0.1.

Possible future architecture:

    Source Code
         ↓
    Contract Compiler
         ↓
    SwasVM Bytecode
         ↓
    Deployment Transaction
         ↓
    Blockchain State

The bytecode format must be deterministic and versioned.

---

## 13. VM Versioning

Every executable contract must be associated with a VM version.

Example:

    vm_version = 1

Execution semantics must be determined by the VM version.

This allows future VM upgrades without silently changing the meaning of existing contracts.

A historical block must remain reproducible using the execution rules applicable at that height.

---

## 14. Contract Deployment

A contract deployment transaction creates executable contract state.

Conceptual flow:

    Deployment Transaction
            ↓
        Validation
            ↓
        Resource Check
            ↓
        VM Initialization
            ↓
        Contract Code Validation
            ↓
        Contract State Creation
            ↓
        State Root Update

Deployment must be deterministic.

Invalid contract code must be rejected before it can become executable state.

---

## 15. Contract Invocation

A contract invocation transaction identifies:

- target contract
- caller
- function or entry point
- arguments
- transaction metadata
- execution resource limit

Conceptual structure:

    ContractCall {
        contract
        entry_point
        arguments
        resource_limit
    }

Arguments must use a canonical deterministic encoding.

---

## 16. Entry Points

Contracts expose explicitly defined entry points.

Example:

    transfer(...)
    mint(...)
    burn(...)
    get_balance(...)

The VM must reject calls to unavailable or invalid entry points.

The exact contract ABI will be defined when the bytecode system is implemented.

---

## 17. Host Interface

The VM may expose a limited set of deterministic host functions.

Potential categories:

### Blockchain information

- current block height
- block hash
- chain ID

### Transaction information

- sender
- transaction hash
- nonce

### State

- read storage
- write storage
- delete storage

### Contract interaction

- call contract
- inspect contract metadata

Every host function must have explicitly defined deterministic semantics.

---

## 18. Host Function Restrictions

A host function must never secretly introduce nondeterministic behavior.

For example, a function such as:

    get_current_time()

would be unsafe if it returned the local machine's wall-clock time.

Instead, a protocol-defined block timestamp may be exposed:

    get_block_timestamp()

The difference is important because the latter is part of the consensus-defined execution context.

---

## 19. Resource Accounting

Contract execution must have bounded resource consumption.

The VM must track execution resources.

The initial design uses a gas-like abstraction.

Conceptually:

    execution_limit = N

Each VM operation consumes a defined amount of execution resource.

Example:

    instruction → resource cost
    storage read → resource cost
    storage write → resource cost
    contract call → resource cost

Execution must terminate when the resource limit is exhausted.

---

## 20. Resource Determinism

Resource costs must be deterministic.

All honest nodes executing the same transaction must calculate the same resource usage.

Resource accounting must not depend on:

- CPU model
- RAM size
- operating system
- compiler optimization
- local machine load
- network latency

This allows execution limits to provide a consensus-safe computation bound.

---

## 21. Out-of-Resource Execution

If a contract exhausts its execution resource limit:

    execution
        ↓
    resource exhausted
        ↓
    execution failure

The protocol must define whether the transaction's state changes are completely reverted.

The initial design should use atomic transaction semantics:

    success → commit state changes

    failure → revert state changes

The exact fee behavior for failed execution will be finalized during transaction execution design.

---

## 22. Atomic State Changes

Contract execution must use transactional state semantics.

Conceptually:

    Begin State Overlay
            ↓
       Execute Contract
            |
       +----+----+
       |         |
    Success    Failure
       |         |
       v         v
    Commit     Revert

Partial state changes must never remain after a failed transaction unless explicitly defined by protocol rules.

---

## 23. Nested Calls

Contracts may eventually call other contracts.

Conceptual flow:

    Contract A
        |
        +---- call → Contract B
                       |
                       +---- call → Contract C

Nested calls must obey:

- resource limits
- call-depth limits
- state isolation rules
- deterministic error propagation
- transaction atomicity

The initial VM implementation may restrict or defer nested calls.

---

## 24. Call Depth

Unbounded recursive contract calls could exhaust resources.

Therefore the VM must enforce a maximum call depth.

Example:

    maximum_call_depth = N

The exact value will be selected through implementation and security testing.

Exceeding the limit results in deterministic execution failure.

---

## 25. Reentrancy

Cross-contract calls can introduce reentrancy risks.

The VM and contract execution model must make call ordering explicit.

Future contract standards may provide patterns or runtime protections for safe state transitions.

The VM must not assume that contracts are automatically safe merely because execution is deterministic.

---

## 26. Execution Result

Every VM execution must produce a deterministic result.

Conceptually:

    ExecutionResult {
        success
        return_data
        state_changes
        resource_used
        events
        error
    }

The exact binary representation will be finalized during implementation.

---

## 27. Execution Errors

VM errors must be deterministic and protocol-defined.

Possible categories:

- invalid bytecode
- invalid entry point
- invalid arguments
- insufficient resources
- missing contract
- unauthorized operation
- invalid state access
- call depth exceeded
- arithmetic overflow
- explicit contract failure

Error behavior must be identical across nodes.

---

## 28. Arithmetic Safety

Consensus-critical arithmetic must use checked operations.

Examples include:

- balance changes
- resource accounting
- storage accounting
- integer operations exposed by the VM

Arithmetic overflow must never silently wrap around when that could alter consensus state.

---

## 29. Integer Representation

The VM must define canonical integer behavior.

The implementation must explicitly specify:

- integer widths
- signed versus unsigned values
- overflow behavior
- division behavior
- comparison rules
- serialization rules

These rules must not depend on the host language's default behavior.

---

## 30. Canonical Serialization

Contract arguments, return values, state keys, and protocol-level execution data require canonical serialization.

The same logical value must always serialize to the same byte sequence.

Canonical encoding prevents different nodes from interpreting the same data differently.

The exact serialization format will be selected during VM implementation.

---

## 31. Contract Events

Contracts may emit deterministic events.

Example:

    Event {
        contract
        event_type
        data
    }

Events are execution outputs.

They may later be indexed by the SwasChain explorer.

Events must be deterministic and must not directly modify consensus state outside the defined execution model.

---

## 32. Contract Address

Contracts require deterministic addresses.

The exact address derivation will be finalized during the contract deployment design.

The derivation must prevent accidental collisions and must be deterministic across all nodes.

Potential inputs may include:

- deployer address
- deployment nonce
- contract creation identifier

The final scheme must be specified before contract deployment is implemented.

---

## 33. Permissions

The VM must expose a deterministic authorization model.

Possible authorization inputs include:

- transaction sender
- contract ownership state
- explicit role state
- contract-defined authorization rules

The VM itself should provide primitive identity information while application-level permissions remain contract-defined where appropriate.

---

## 34. Native Functions

Some blockchain functionality may eventually be exposed as native host functions.

Examples could include:

- token transfers
- validator interaction
- staking
- governance
- system contracts

Native functions require strict protocol definitions because they directly affect consensus state.

They must not become arbitrary escape hatches from VM determinism.

---

## 35. System Contracts

Future SwasChain versions may use system contracts for protocol functionality.

Conceptually:

    User Contract
         |
         v
    System Contract
         |
         v
    Protocol State

System contracts must execute under explicitly defined protocol rules.

They must not receive unrestricted host privileges.

---

## 36. State Root Integration

After execution, the resulting global state must produce a deterministic state root.

Conceptually:

    Previous State
          +
    Valid Transactions
          ↓
       SwasVM
          ↓
    New State
          ↓
     State Root

Every node executing the same valid block must derive the same state root.

A mismatch indicates an execution or validation failure.

---

## 37. Block Execution

The block execution pipeline will eventually follow:

    Receive Block
         ↓
    Validate Header
         ↓
    Validate Transactions
         ↓
    Load Previous State
         ↓
    Execute Transactions
         ↓
    Apply State Changes
         ↓
    Calculate State Root
         ↓
    Compare With Block State Root
         ↓
    Commit Block State

The VM is only one component of this pipeline.

---

## 38. Consensus Interaction

Consensus must not execute arbitrary VM code while deciding whether a block is valid.

The logical separation is:

    Consensus
       ↓
    selects / finalizes block
       ↓
    deterministic execution
       ↓
    state transition

However, block validity ultimately requires that execution produces the protocol-specified state root.

Consensus and execution therefore remain separate modules with a well-defined interface.

---

## 39. Execution Interface

A future Rust interface may conceptually resemble:

    trait ExecutionEngine {
        fn execute_transaction(
            &mut self,
            state: &mut State,
            tx: &Transaction,
            context: &ExecutionContext,
        ) -> ExecutionResult;
    }

The exact Rust interface is intentionally deferred until the state engine and transaction execution architecture are implemented.

---

## 40. VM State Interface

The VM should interact with state through an abstract interface.

Conceptually:

    trait StateAccess {
        fn read(&self, key: &[u8]) -> Option<Vec<u8>>;
        fn write(&mut self, key: Vec<u8>, value: Vec<u8>);
        fn delete(&mut self, key: &[u8]);
    }

This abstraction prevents the VM from depending directly on a particular database implementation.

---

## 41. VM and Storage Separation

The VM must not depend directly on RocksDB, SQLite, files, or another specific storage engine.

The architecture should remain:

    SwasVM
       ↓
    State Interface
       ↓
    State Engine
       ↓
    Storage Backend

This allows the storage implementation to change without rewriting contract execution.

---

## 42. VM Security Boundary

The VM must assume that contract code can be malicious.

Security requirements include:

- bounded execution
- bounded memory
- bounded call depth
- controlled state access
- deterministic host functions
- validated bytecode
- canonical serialization
- safe arithmetic
- explicit authorization
- atomic state transitions

Contract code must never gain arbitrary host-system access.

---

## 43. Bytecode Validation

Before execution, bytecode must pass validation.

Validation may include:

- correct magic/version
- valid instruction encoding
- valid control-flow structure
- valid operand types
- valid resource metadata
- no forbidden instructions
- valid entry points

Invalid bytecode must never reach normal execution.

---

## 44. VM Memory Model

The VM must define bounded memory usage.

Possible resources include:

- stack
- linear memory
- execution frames
- temporary values
- contract storage operations

The exact memory model depends on the final bytecode architecture.

The important invariant is that memory usage must be bounded and protocol-controlled.

---

## 45. Deterministic Environment

A contract execution environment must behave identically on:

- Windows
- Linux
- macOS
- different CPU architectures
- different database backends

Host implementation details must not alter consensus results.

Cross-platform execution testing is therefore mandatory.

---

## 46. Testing Requirements

The VM must eventually include:

### Determinism tests

- same input produces same output
- same state produces same state root
- repeated execution produces identical results
- cross-platform execution consistency

### Resource tests

- resource accounting
- resource exhaustion
- memory limits
- call-depth limits

### State tests

- read
- write
- delete
- rollback
- commit
- storage isolation

### Security tests

- malformed bytecode
- invalid instructions
- unauthorized state access
- arithmetic overflow
- recursive calls
- reentrancy scenarios
- malicious contracts

### Failure tests

- invalid arguments
- missing contract
- missing entry point
- execution failure
- state rollback

---

## 47. Differential Testing

Future VM implementations should support differential testing.

The same contract and input can be executed through:

    Implementation A
          |
          +------+
                 |
                 v
              Results
                 ^
                 |
          +------+
          |
    Implementation B

Results must match exactly where the protocol requires deterministic equivalence.

This can help detect implementation-specific consensus bugs.

---

## 48. Fuzz Testing

The VM should eventually be fuzz-tested with:

- random bytecode
- random arguments
- malformed serialized values
- random state
- extreme resource limits
- deeply nested calls
- boundary integer values

The objective is to discover crashes, panics, incorrect state transitions, and nondeterministic behavior.

---

## 49. Formal Invariants

Important VM invariants include:

1. Same input state and transaction produce the same execution result.
2. VM execution cannot access arbitrary host resources.
3. Resource consumption is deterministic.
4. Failed execution cannot leave unauthorized partial state.
5. Contract storage is isolated.
6. Arithmetic behavior is explicitly defined.
7. Contract code is validated before execution.
8. VM version determines execution semantics.
9. State roots are deterministic.
10. Invalid execution cannot produce a valid consensus state.
11. Remote contract code cannot bypass protocol validation.
12. Historical blocks remain reproducible under their applicable VM rules.

---

## 50. Initial Scope

SwasChain v0.1 will NOT implement:

- smart contracts
- bytecode
- contract deployment
- contract calls
- gas accounting
- contract storage
- system contracts

The initial blockchain will focus on the core account and transaction state machine.

The VM remains an architectural extension point.

---

## 51. Future VM Roadmap

Potential future milestones:

### VM v0.1

- execution interface
- deterministic state access
- basic bytecode format
- resource metering

### VM v0.2

- contract deployment
- contract invocation
- persistent contract storage
- events

### VM v0.3

- cross-contract calls
- improved resource accounting
- system contracts

### VM v0.4

- developer SDK
- contract tooling
- debugging
- local simulation

### VM v1.0

- hardened execution engine
- comprehensive security testing
- deterministic cross-platform execution
- production contract compatibility

These milestones are architectural targets, not fixed deadlines.

---

## 52. Implementation Principle

SwasVM must not be implemented simply to make SwasChain appear more complex.

Every VM feature must solve a real protocol requirement.

Complexity is justified only when it improves:

- determinism
- safety
- programmability
- performance
- developer usability
- protocol extensibility

The goal is a defensible execution engine, not a large codebase.

---

## 53. Current Status

SwasVM Specification: Draft v0.1

Designed:

- VM boundary
- deterministic execution model
- execution context
- state interface
- contract storage model
- resource accounting
- execution errors
- contract calls
- VM versioning
- bytecode validation requirements
- security boundary
- state-root integration
- testing strategy
- future evolution

Not yet implemented.

Implementation will begin only after the core SwasChain state machine, transaction execution model, and storage architecture are stable.