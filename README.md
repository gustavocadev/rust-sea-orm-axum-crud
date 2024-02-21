# A simple example about a Rest API written in Rust 🦀, axum, Sea ORM

## Setup

Rename the `.env.example` file to `.env`.

Create a migration file with(which means create a table in the database):

```bash
sea-orm-cli migrate up
```

If you don't have `sea-orm-cli` installed, you can install it with:

```bash
cargo install sea-orm-cli
```

## Run the server

```bash
cargo run
```

Or for development you probably want to use `cargo-watch` crate

```bash
cargo-watch -x run
```

if you don't have `cargo-watch` installed, you can install it with:

```bash
cargo install cargo-watch
```
