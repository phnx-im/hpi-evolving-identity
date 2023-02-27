# EID creation

To create an initial client a corresponding `EidMember` must be created (
see [Create EidMember](../create_eid_member.md)).
This member can then be used to initialize the `EidClient`.

```rust,no_run,noplayground
{{#include ../../eid_mls_example/src/main.rs:alice_create_eid}}
```

> **_NOTE:_** The client has to cross-sign its own membership to be member of the EID.