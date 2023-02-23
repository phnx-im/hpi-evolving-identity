# User manual

The user manual describes how to use the different parts of the EID API.

## Prerequisites

Most EID operations require a `backend` object that provides all required cryptographic algorithms (via
the [`OpenMlsCryptoProvider`] trait).
Currently, there is only one implementation available through the [openmls_rust_crypto] crate.

Thus, you can create the `backend` object for the following examples using ...

```rust,no_run,noplayground
{{#include ../../../eid/tests/book_code.rs:create_backend_rust_crypto}}
```

[`openmlscryptoprovider`]: https://docs.rs/openmls/latest/openmls/prelude/trait.OpenMlsCryptoProvider.html

[openmls_rust_crypto]: https://crates.io/crates/openmls_rust_crypto

[openmls_evercrypt]: https://crates.io/crates/openmls_evercrypt
