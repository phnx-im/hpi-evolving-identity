# Join a group from an Invitation

To join a group from an `Invitation`, a new EID can be instantiated directly
with the Evolvement that was created by another client calling `add`.

```rust,no_run,noplayground
{{#include ../src/eid_mls_example/src/main.rs:bob_joins_with_invitation}}
```