# EID Backend

Most EID operations require a `backend` object that provides all required cryptographic algorithms (via the `EidBackend`
trait).
Currently, there is only one implementation available that actually uses cryptography (via openMLS).

You can create the `backend` object by using:
```rust,no_run,noplayground
{{#include ../../eid_mls_example/src/main.rs:create_backend}}
```