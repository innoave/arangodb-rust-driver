
# Rincon

[![Crates.io][crates_badge]][crate]
[![Docs.rs][docs_badge]][documentation]
[![Linux Build Status][travis_badge]][Travis CI]
[![Windows Build Status][appveyor_badge]][Appveyor CI]
[![codevoc.io][codecov_badge]][codecoverage]
[![Apache-2.0][license_badge]][Apache-2.0]
[![Join the chat][gitter_badge]][chat]

[crates_badge]: https://img.shields.io/crates/v/rincon.svg
[docs_badge]: https://docs.rs/rincon/badge.svg
[travis_badge]: https://travis-ci.org/innoave/rincon.svg?branch=master
[appveyor_badge]: https://ci.appveyor.com/api/projects/status/github/innoave/rincon?branch=master&svg=true
[codecov_badge]: https://codecov.io/gh/innoave/rincon/branch/master/graph/badge.svg
[license_badge]: https://img.shields.io/badge/license-Apache%2D%2D2%2E0-blue.svg
[gitter_badge]: https://badges.gitter.im/innoave/rincon.svg

[crate]: https://crates.io/crates/rincon
[documentation]: https://docs.rs/rincon
[Travis CI]: https://travis-ci.org/innoave/rincon
[Appveyor CI]: https://ci.appveyor.com/project/innoave/arangodb-rust-driver
[codecoverage]: https://codecov.io/github/innoave/rincon?branch=master
[Apache-2.0]: https://www.apache.org/licenses/LICENSE-2.0
[chat]: https://gitter.im/innoave/rincon
[license]: LICENSE
[rincon]: rincon

[Rincon] is an [ArangoDB] driver for [Rust]. It enables low level access to
[ArangoDB] in a typesafe and [Rust] idiomatic manner. 

The name [Rincon][Rincon name] is derived from the [Avocado variety list].

The vision for this project is to provide a fast and typesafe client lib for
easy and flexible use of [ArangoDB] in applications.  

**Status: Experimental**

This project is under heavy development. There is no released version yet.

The plans are to provide:

* A typesafe low level driver API for the [ArangoDB REST API]. (WIP)
* Convenience 'Session'-API on top of the driver API. (WIP)
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

```rust
    let method = CreateCollection::with_name("my_collection");
    let result = connection.execute(method);
```

#### Session API on top of the driver API

e.g. something like

```rust
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

```rust
    let query = Aql::from(customers)
        .filter(|c| c.age == 42)
        .limit(10)
        .return(|c| (c.name, c.age, c.city))
        ;
    let results = query.results(session);
```

## Multiple Crates

The functionality of the [Rincon] [ArangoDB] Rust driver is split up into several crates for a
modular design. This modular design enforces a clean code base. Applications can flexible combine
the functionality they need by adding different combinations of the crates to their project.
Developers can easily customize and extend the functionality of the existing crates.

For an overview of crates in this project see the [README](rincon/README.md) in the [rincon] crates
subdirectory.

Each crate comes with its own README file which describes the purpose of the crate and how to use
it.

## License

Licensed under Apache License, Version 2.0<br/>
see [LICENSE] or http://www.apache.org/licenses/LICENSE-2.0 for details.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
licensed as above, without any additional terms or conditions.


[ArangoDB]: https://www.arangodb.org
[ArangoDB REST API]: https://docs.arangodb.com/3.2/HTTP/index.html
[AQL]: https://docs.arangodb.com/3.2/AQL/index.html
[Avocado variety list]: http://www.ucavo.ucr.edu/AvocadoVarieties/VarietyFrame.html
[Rincon name]: http://ucavo.ucr.edu/avocadovarieties/VarietyList/Rincon.html
[Rust]: https://www.rust-lang.org
