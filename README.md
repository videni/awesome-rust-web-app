Awesome Rust Web App
===================

This project shows you how to develop Rust web application by implementing a simple blog management system.
but it is not a simple toy, I try to implement most common features for website development, check the [Features](./docs/features.md) doc for if a feature you want is implemented.

## Requirements

* Actix-web
* Postgresql

## [Features](./docs/features.md)

All features included in this package.

## Getting started

* Create .env

```bash
cp .env.example .env
```

* Install diesel cli with mysql

```bash
cargo install diesel_cli --no-default-features --features postgres
diesel setup
```

* Setup your database

```bash
  diesel database setup
```

* Build this project with `cargo build`. You are welcome to compile with `--release` if you'd like.
* Run with `cargo run`.
  
## Test

```bash
cp .env.example .env.test
```

then run all tests

```
cargo test
```

You can also run specific test

```
cargo test test_login
```

For local development, you usually want to print something into std output, 
then you can run your test this way 
```
cargo test test_login -- --nocapture
```
check  [`--nocapture`](https://doc.rust-lang.org/cargo/commands/cargo-test.html) for details.
