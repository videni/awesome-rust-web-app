Actix web tutorial
===================

An actix-web demo following DDD patterns.

## Project scope

* Follow DDD patterns
* Integration test sample
* Using Postgresql instead of MySQL
  
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

## Todo 

* Rollback after integration test
* Repository for database operations
* Model for business logic
* JWT Authentication