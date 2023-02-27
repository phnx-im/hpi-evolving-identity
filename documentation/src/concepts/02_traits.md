# Traits

We provide several traits to describe the public interfaces of each EID component and to express further requirements to
implementations by trait bounds.

## Client

A Client of an EID that can create new Evolvements and evolve its EidState by applying any Evolvements.
For more context, see section [Concepts – Client and Evolvements](01_evolving_identity.md#client-and-evolvements).

```rust,no_run,noplayground

{{#include ../../../traits/src/client.rs:15:228}}

```

## EID Evolvement

An Evolvement represents one change in an EID.
The History of an EID is the log of all evolvements.

```rust,no_run,noplayground

{{#include ../../../traits/src/evolvement.rs:6:}}

```

## Transcript

The Transcript holds a trusted State and a log of Evolvements.
It is the public state of an EID that should be verifiable by any third party.
It calculates its current State by applying Evolvements like clients do. It knows all Members that are in the EID.
It cannot create any Evolvement.
For more context, see section [Concepts – Transcript](01_evolving_identity.md#transcript).

```rust,no_run,noplayground

{{#include ../../../traits/src/transcript.rs:13:}}

```

## State

The state of the EID. Each Client and the Transcript have their own State.

```rust,no_run,noplayground

{{#include ../../../traits/src/state.rs:8:}}

```

## Member

An EID Member represents a member of the EID in the EID State. A member can be added or removed from the EID.

```rust,no_run,noplayground

{{#include ../../../traits/src/member.rs:4:}}

```