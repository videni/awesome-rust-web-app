SSRProxy
========
Server side rendering for your Single Web Application

## Getting started

* Copy (`cp`) [.env.example](./.env.example) to `.env` within this directory, and change the environment variables accordingly to your system.
  
```
cp .env.example .env
```
* Setup database
```
cargo install diesel_cli --no-default-features --features mysql
diesel setup
```
* Setup your database by running `diesel database setup`. Make sure it has completed successfully.
* Build this project with `cargo build`. You are welcome to compile with `--release` if you'd like.
* Run with `cargo run`.