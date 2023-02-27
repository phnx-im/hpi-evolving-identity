# Add members to an identity

To create an Evolvement that adds a member the Clients `add` function has to be called with the `EidMember` that will be
added.
The `add` function returns evolvement that contains the actual change message and an `Invitation` for the new member.

```rust,no_run,noplayground
{{#include ../eid_mls_example/src/main.rs:alice_adds_bob}}
```

The new member can then [join the group themselves](create_client/06_join_from_invitation.md) after the evolvement has
been applied.