# Join a group from an Invitation

To join a group from an invitation a new client is instantiated directly
with the `Evolvement` that was created by another client calling `add`.

```rust,no_run,noplayground
{{#include ../../eid_mls_example/src/main.rs:bob_joins_with_invitation}}
```

> **_NOTE:_**  To complete the joining process, additionally a cross sign is needed.
