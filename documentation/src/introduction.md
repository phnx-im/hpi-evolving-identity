# Evolving Identity

The library is implemented in the Rust programming language.
The full source code is provided on [GitHub](https://github.com/phnx-im/hpi-evolving-identity).

The core of the library is a generic interface for the EID protocol with generic tests.
This hides complexity and guides any (also possible future) implementations of the EID API.

Additionally, the library provides two concrete implementations of the generic interface.
One is a proof-of-concept, which doesn't have any cryptographic properties and thus doesn't meet any security
requirements.
We call this implementation _EID Dummy_.
The other implementation uses the [openMLS](https://github.com/openmls/openmls) library as a
backbone.
With this approach, the implementation makes use of MLS's security.
with the goal to meet the security properties defined for the EID.

This book covers the MLS EID implementation.