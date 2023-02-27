# Removing members from an EID
To remove a member call a Client's `remove` function and pass the `EidMember` that's supposed to be removed.

```rust,no_run,noplayground
{{#include ../../eid_mls_example/src/main.rs:charlie_removes_bob}}
```