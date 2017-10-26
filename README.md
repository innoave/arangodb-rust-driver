
# ArangoDB Rust Driver

[![Join the chat at https://gitter.im/innoave/arangodb-rust-driver](https://badges.gitter.im/innoave/arangodb-rust-driver.svg)](https://gitter.im/innoave/arangodb-rust-driver?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

[![Crates.io][crb]][crl]
[![Docs.rs][dcb]][dcl]
[![Build Status][tcb]][tcl]
[![codevoc.io][cvb]][cvl]
[![Apache-2.0][lib]][lil]
[![Join the chat][gcb]][gcl]

[crb]: https://img.shields.io/crates/v/arangodb_client.svg?style=flat-square
[dcb]: https://docs.rs/arangodb_client/badge.svg
[tcb]: https://img.shields.io/travis/innoave/arangodb-rust-driver/master.svg?style=flat-square
[cvb]: https://img.shields.io/codecov/c/github/innoave/arangodb-rust-driver/master.svg?style=flat-square
[lib]: https://img.shields.io/badge/license-Apache%2D%2D2%2E0-blue.svg?style=flat-square
[gcb]: https://badges.gitter.im/innoave/general.svg?style=flat-square

[crl]: https://crates.io/crates/arangodb_client/
[dcl]: https://docs.rs/arangodb_client
[tcl]: https://travis-ci.org/innoave/arangodb-rust-driver/
[cvl]: https://codecov.io/github/innoave/arangodb-rust-driver?branch=master
[lil]: https://www.apache.org/licenses/LICENSE-2.0
[gcl]: https://gitter.im/innoave/arangodb_client

An ArangoDB client for Rust. It enables low level access to ArangoDB in a
typesafe and Rust idiomatic manner. 

<!--TODO uncomment this section once the first release has been published
[Documentation](https://docs.rs/arangodb_client)
-->

The vision for this project is to provide a fast and typesafe client lib for
easy and flexible use of ArangoDB in applications.  

**Status: Experimental**

This project is under heavy development. There is no released version yet.

The plans are to provide:

* A typesafe and low level driver API for the REST API of ArangoDB. (WIP)
* Convenience 'Session'-API on top of the driver API. (PLANNED)
* API to compose AQL queries in a typesafe manner. (PLANNED)

## Ideas

In this section ideas for this project are collected. The provided code
snippets shall illustrate the ideas. This does not mean that they are
implemented yet or will be implemented in exact that way. So don't
expect that any code snippet provided here will compile or work yet. 

#### Typesafe and low level driver API for the REST API of ArangoDB

e.g. something like

```
    let method = CreateCollection::with_name("my_collection");
    let result = connection.execute(method);
```

#### Session API on top of the driver API

e.g. something like

```
    let session = datasource.create_session();
    let database = session.use_database("my_database");
    let collection = database.create_collection("my_new_collection");
    collection.add_document(..);
    let document = collection.get_document(..);
```

a main purpose of the session shall be:
* no need to specify the database and collection on each and every request.
* reuse of connections to the database, e.g. from a connection pool, for
  speed and efficient use of resources.
* convenient API for transaction handling
* efficient execution of batches of operations.

#### API to compose AQL queries in a typesafe manner

e.g. something like

```
    let query = Aql::From(customers)
        .filter(|c| c.age == 42)
        .limit(10)
        .return(|c| (c.name, c.age, c.city))
        ;
    let results = query.results(session);
```


<!--TODO uncomment this section once the first release has been published
## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
arangodb_client = "0.1"
```

And add this to your crate:

```rust
extern crate arangodb_client;
```

See the [client example](./examples/client.rs) for a working example.
-->

## License

Licensed under Apache License, Version 2.0<br/>
see [LICENSE](LICENSE) or http://www.apache.org/licenses/LICENSE-2.0) for details.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be licensed as above, without any
additional terms or conditions.
