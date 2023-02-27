# Create a Transcript

A Transcript is always created from an exported Client state.
This state is considered as trusted and will be the Transcript's first state.
To allow anyone to create a Transcript at any point in time the Transcript construction also takes a list of
Evolvements.
This allows a third party to recalculate the current EIDs state while verifying each evolvement.

```rust,no_run,noplayground
{{#include ../eid_mls_example/src/main.rs:create_transcript}}
```
