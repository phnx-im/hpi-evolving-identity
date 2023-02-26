# Removing members from an EID

Removing members is as easy as adding them.
Simply call the EID `remove` function with the `EidMember` that's supposed to be removed.
Again, the returned evolvement should be sent to all existing EID members.

```rust,no_run,noplayground
{{#include ../src/eid_mls_example/src/main.rs:alice_adds_bob}}
```