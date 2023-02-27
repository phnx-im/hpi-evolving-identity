# Create an MLS EID Member

An MLS EID Member is defined by its [openMLS key package](https://openmls.tech/book/user_manual/create_key_package.html)
and [openMLS signature key](https://openmls.tech/book/user_manual/identity.html).  
It is needed

1. To initialize an EID with its first member
2. To add new members to the EID
3. To remove a member from the EID

For the first scenario, a member generated this way

```rust,no_run,noplayground
{{#include ../eid_mls_example/src/main.rs:create_member}}
```

For adding and removing a member, we assume there exists a way to retrieve the needed key package or signature key.
