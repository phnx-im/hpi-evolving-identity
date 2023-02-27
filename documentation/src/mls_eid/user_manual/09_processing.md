# Processing evolvements

Every change to the EID has to be explicitly applied (including the client who created the Evolvement).
To do so, simply call the `evolve` function of the client or transcript:

The `simulate_transfer` function simulates transfer over the wire by serializing and deserializing the
Evolvement.

```rust,no_run,noplayground
{{#include ../src/eid_mls_example/src/main.rs.rs:processing_evolvements}}
```