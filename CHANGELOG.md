# Changelog

All notable changes to SwasChain are documented in this file.

## Unreleased

Development is currently focused on strengthening the transaction and protocol layers.

### Transaction

* Added protocol-aware transaction decoding.
* Added rejection for unsupported transaction versions.
* Added rejection for invalid chain IDs.
* Added self-transfer validation.
* Added transaction serialization and deserialization coverage.
* Added transaction hash generation using the defined hash domain separator.

### Protocol

* Defined transaction protocol constants.
* Defined transaction size and serialization boundaries.
* Documented transaction lifecycle and validation requirements.

### Documentation

* Added transaction serialization documentation.
* Expanded protocol and consensus documentation.
* Documented current development status and implementation roadmap.

## Development Principles

SwasChain aims to keep protocol behavior explicit, deterministic, testable, and documented.

Changes to transaction formats or validation rules should be accompanied by tests and corresponding documentation.
