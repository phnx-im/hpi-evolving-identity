# User manual

The user manual describes how to use the different parts of the EID API.

## Prerequisites

Most EID operations require a `backend` object that provides all required cryptographic algorithms (via the `EidBackend`
trait).
Currently, there is only one implementation available (using openMLS).

Thus, you can create the `backend` object for the following examples using ...

```rust,no_run,noplayground
{{#include ../src/eid_mls_example/src/main.rs:create_backend}}
```