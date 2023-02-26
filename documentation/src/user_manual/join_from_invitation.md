# Join a group from an Invitation

To join a group from an `Invitation` message, a new EID can be instantiated directly from the evolvement with
the `Invitation`.

```rust,no_run,noplayground
{{#include ../src/eid_mls_example/src/main.rs:bob_joins_with_invitation}}
```

Pay attention not to forward an Invitation message to a client before its associated commit has been accepted by the
Delivery Service.
Otherwise, you would end up with an invalid EID instance.
