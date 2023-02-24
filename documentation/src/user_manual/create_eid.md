# EID creation

With a credential and a key package created, one can now create a new Evolving Identity.

Firstly, a new `EidMember` has to be created from the generated keys.
This member can then be used to initialise a new EID (`EidClient`).

```rust,no_run,noplayground
{{#include ../../../eid/tests/book_code.rs:alice_create_eid}}
```