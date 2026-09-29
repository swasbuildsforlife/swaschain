# SwasChain Development Guide

## Overview

SwasChain is being developed as a modular blockchain protocol with an emphasis on deterministic behavior, explicit protocol rules, cryptographic integrity, and testable components.

Development is organized incrementally so that protocol foundations are established before higher-level execution and networking features.

## Development Principles

The project follows these principles:

* Keep protocol behavior explicit and deterministic.
* Prefer small, testable modules.
* Document protocol decisions before implementing complex behavior.
* Keep serialization formats stable and well defined.
* Validate untrusted transaction data before execution.
* Add tests alongside protocol functionality.
* Keep cryptographic operations isolated from application logic.

## Current Development Areas

### Protocol

The protocol layer defines:

* Protocol version
* Chain ID
* Transaction format
* Domain separators
* Serialization boundaries
* Hashing rules

### Cryptography

The cryptographic layer currently provides functionality related to:

* SHA-256 hashing
* Ed25519 signatures
* Signature verification
* Address derivation

### Transactions

The transaction layer handles:

* Transaction structure
* Binary serialization
* Binary deserialization
* Transaction hashing
* Signature handling
* Sender identity verification
* Basic transaction validation

### Validation

Transaction validation checks protocol and state-related rules before a transaction can proceed.

Examples include:

* Supported protocol version
* Correct chain ID
* Non-zero amount
* Valid fee
* Correct nonce
* Valid sender identity
* Valid signature
* Sufficient balance
* Arithmetic overflow protection
* Self-transfer rejection

## Testing

Before committing protocol changes, the project should be tested with:

```text
cargo fmt
cargo test
```

Tests should cover both valid transactions and invalid or adversarial inputs.

## Documentation

Protocol behavior should be documented alongside its implementation.

Important documentation areas include:

* Architecture
* Protocol specification
* Transaction specification
* Serialization
* Consensus
* Networking
* Virtual machine design
* Development roadmap

## Contribution Workflow

A typical development cycle is:

1. Define the protocol behavior.
2. Document the design when appropriate.
3. Implement the smallest useful change.
4. Add or update tests.
5. Run formatting and tests.
6. Review the Git diff.
7. Create a focused commit.
8. Push the verified change.

## Commit Guidelines

Commit messages should describe the actual change.

Examples:

```text
feat: add transaction validation
fix: reject invalid transaction encoding
test: expand transaction serialization coverage
docs: document transaction serialization
refactor: simplify transaction validation
```

Commits should avoid unrelated formatting or generated changes whenever possible.

## Future Development

Planned areas include:

* Transaction pool and mempool behavior
* State representation
* Block structure
* Block validation
* Consensus implementation
* Peer-to-peer networking
* State execution
* Virtual machine implementation
* Wallet functionality
* Development network tooling

Each component should be introduced with clear interfaces and corresponding tests.
