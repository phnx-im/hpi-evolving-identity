# Adding members to an identity

To add members, again an `EidMember` is required.
The EID `add` function has to be called, and the returned evolvement contains both the actual change message and
an `Invitation` for the new member.
It should be sent to all existing EID members and the newly added member.

```rust,no_run,noplayground
{{#include ../src/eid_mls_example/src/main.rs:alice_adds_bob}}
```

The new member can then [join the group themselves](join_from_invitation.md) after the evolvement has been applied.