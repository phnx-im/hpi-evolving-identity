# Evolving Identity

1. [Building the library and running tests](#building-the-library-and-running-tests)
2. [Building Docs](#building-docs)

## Building the library and running tests

The following commands have to be run from the libraries root directory.
To build the library, run

```bash
cargo build
```

The output files will be created in the target/debug subdirectory.

To run tests, do:

```bash
cargo test -F test
```

## Building Docs

A documentation of all the public functions and structs will be built and opened in the browser using

```bash
cargo doc --no-deps --open
```

The files can also be found in the `target/doc` subdirectory.

To build a high-level md-book documentation, the _mdbook_ and _plantuml_ binaries are required.
These can be installed using cargo:

```bash
cargo install mdbook
cargo install mdbook-plantuml --no-default-features --features plantuml-ssl-server
```

The md-book can be created and opened by doing

```bash
cd documentation
mdbook serve --open
```

The book files will then be located in the `documentation/book` subdirectory.