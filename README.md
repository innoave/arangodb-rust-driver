
# Rincon

[![Join the chat at https://gitter.im/innoave/rincon](https://badges.gitter.im/innoave/rincon.svg)](https://gitter.im/innoave/rincon?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

[![Crates.io][crb]][crl]
[![Docs.rs][dcb]][dcl]
[![Linux Build Status][tcb]][tcl]
[![Windows Build Status][avb]][avl]
[![codevoc.io][cvb]][cvl]
[![Apache-2.0][lib]][lil]
[![Join the chat][gcb]][gcl]

[crb]: https://img.shields.io/crates/v/rincon_client.svg
[dcb]: https://docs.rs/rincon_client/badge.svg
[tcb]: https://travis-ci.org/innoave/rincon.svg?branch=master
[avb]: https://ci.appveyor.com/api/projects/status/github/innoave/rincon?branch=master&svg=true
[cvb]: https://codecov.io/gh/innoave/rincon/branch/master/graph/badge.svg
[lib]: https://img.shields.io/badge/license-Apache%2D%2D2%2E0-blue.svg
[gcb]: https://badges.gitter.im/innoave/arangodb-rust-driver.svg

[crl]: https://crates.io/crates/rincon_client
[dcl]: https://docs.rs/rincon_client
[tcl]: https://travis-ci.org/innoave/rincon
[avl]: https://ci.appveyor.com/project/innoave/rincon
[cvl]: https://codecov.io/github/innoave/rincon?branch=master
[lil]: https://www.apache.org/licenses/LICENSE-2.0
[gcl]: https://gitter.im/innoave/arangodb-rust-driver

[Rincon] is an [ArangoDB] driver for [Rust]. It enables low level access to
[ArangoDB] in a typesafe and [Rust] idiomatic manner. 

The name [Rincon][Rincon name] is derived from the [Avocado variety list].

<!--TODO uncomment this section once the first release has been published
[Documentation]
-->

The vision for this project is to provide a fast and typesafe client lib for
easy and flexible use of [ArangoDB] in applications.  

**Status: Experimental**

This project is under heavy development. There is no released version yet.

The plans are to provide:

* A typesafe low level driver API for the [ArangoDB REST API]. (WIP)
* Convenience 'Session'-API on top of the driver API. (PLANNED)
* API to compose [AQL] queries in a typesafe manner. (PLANNED)

Currently I am working on the driver API for the REST API. There are a lot 
methods in the REST API which require a lot of coding. Therefore I have
split the implementation into 2 milestones. The details about the planned
milestones and progress is documented in
[docs/arangodb_rest_api_methods.md](docs/arangodb_rest_api_methods.md)

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

The main purpose of the session shall be:
* no need to specify the database and collection on each and every request.
* reuse of connections to the database, e.g. from a connection pool, for
  speed and efficient use of resources.
* convenient API for transaction handling
* efficient execution of batches of operations.

#### API to compose AQL queries in a typesafe manner

e.g. something like

```
    let query = Aql::from(customers)
        .filter(|c| c.age == 42)
        .limit(10)
        .return(|c| (c.name, c.age, c.city))
        ;
    let results = query.results(session);
```

## Crate Features

This crate can be compiled with optional features to adapt to the configuration
of the [ArangoDB] server to be used.

The provided crate features are:

* `cluster` : support for cluster specific features (optional)
* `enterprise` : support for [AangoDB] enterprise specific features (optional)
* `mmfiles` : support for MMFiles storage engine specific features (default)
* `rocksdb` : support for RocksDB storage engine specific features (optional)

Note1: If `rincon_client` is compiled with the `cluster` feature some API
       methods which return cluster specific fields do not work with an
       [ArangoDB] server that is not configured in a cluster. This is due to
       the [ArangoDB] server does not return cluster specific fields in a 
       single server configuration.
       
Note2: A deployed [ArangoDB] server uses either MMFiles or RocksDB storage
       engine. Therefore this crate must be compiled either with the
       `mmfiles` feature enabled or the `rocksdb` feature, but not both.  

<!--TODO uncomment this section once the first release has been published
## Usage

#### Examples:

**Single server with MMFiles storage engine**

By default `rincon_client` is compiled with support for single server
configurations using the MMFiles storage engine.

Add this to your `Cargo.toml` to use this crate with default features:

```toml
[dependencies]
rincon_client = "0.1"
```

This is equivalent to:

```toml
[dependencies]
rincon_client = { version = "0.1", default-features = false, features = ["mmfiles"] }
```

**Using RocksDB storage engine**

If the [ArangoDB] server is configured to use the RocksDB storage engine,
`rincon_client` should be compiled with the `rocksdb` feature to support
RocksDB specific attributes and fields within the API methods.

```toml
[dependencies]
rincon_client = { version = "0.1", default-features = false, features = ["rocksdb"] }
```

**Using an [ArangoDB] Cluster**

To use the [ArangoDB] cluster specific features of the API, `rincon_client`
must be compiled with the `cluster` feature enabled.

To use a clustered server with MMFiles storage engine and enterprise features
add this to your dependencies:

```toml
[dependencies]
rincon_client = { version = "0.1", features = ["cluster"] }
```

To use a clustered server with RocksDB storage engine add this to your dependencies:

```toml
[dependencies]
rincon_client = { version = "0.1", default-features = false, features = ["rocksdb", "cluster"] }
```

**Using [ArangoDB] Enterprise features**

To add support for [ArangoDB] enterprise features in the client API add this to
your dependencies:

```toml
[dependencies]
rincon_client = { version = "0.1", features = ["enterprise"] }
```

And with RocksDB storage engine instead of MMFiles:

```toml
[dependencies]
rincon_client = { version = "0.1", default-features = false, features = ["rocksdb", "enterprise"] }
```
-->

## License

Licensed under Apache License, Version 2.0<br/>
see [LICENSE] or http://www.apache.org/licenses/LICENSE-2.0 for details.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be licensed as above, without any
additional terms or conditions.


[ArangoDB]: https://www.arangodb.org
[ArangoDB REST API]: https://docs.arangodb.com/3.2/HTTP/index.html
[AQL]: https://docs.arangodb.com/3.2/AQL/index.html
[Avocado variety list]: http://www.ucavo.ucr.edu/AvocadoVarieties/VarietyFrame.html
[Documentation]: https://docs.rs/rincon_client
[LICENSE]: LICENSE
[Rincon]: https://github.com/innoave/rincon
[Rincon name]: http://ucavo.ucr.edu/avocadovarieties/VarietyList/Rincon.html
[Rust]: https://www.rust-lang.org
