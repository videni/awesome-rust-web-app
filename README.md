Actix web tutorial
===================
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
  `diesel database setup`
```

* Build this project with `cargo build`. You are welcome to compile with `--release` if you'd like.
* Run with `cargo run`.