rust web development explorer
=============================

A series of opinioned seletions for web development

## Requirements

* Actix-web
* Postgresql

## Features 

* [x] Follow DDD patterns
* [x] Integration test sample
* [x] Using Postgresql instead of MySQL
  
  Sorry I don't use MySQL, uuid is internally handled by Postgresql, I need a way to create the id for an entity before persisting it into database.
* Rollback transactino for functional test
* Repository for database operations
* Model for business logic
* JWT Authentication
* Integrate GraphQL
* [Mutiple languages on Fluent](https://blog.logrocket.com/rust-internationalization-localization-and-translation/#gettext-rs)
  
  It seems Fluent is more promising for website, check the link for details.
  
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
