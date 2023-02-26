# Processing evolvements

Every change to the EID has to be explicitly applied by all members (including the one who executed the change).
To do so, simply call the `evolve` function for the client:

(The `simulate_transfer` function simulates some kind of transfer over the wire by serialising and deserialising the
message.)

```rust,no_run,noplayground
{{#include ../src/eid_mls_example/src/main.rs.rs:processing_evolvements}}
```