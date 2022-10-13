## Rust API(CRUD)

This my study about Rust Language. In this project I'm writing simple api.

![Rust](./rust-icon.png)

This project using [Carbo](https://doc.rust-lang.org/cargo/) module management and [Rocket](https://rocket.rs/) as framework to build the api.

## Project Structure

`api` is for modularizing API handlers.

`models` is for modularizing data logics.

`repository` is for modularizing database logics.

## Warning

If you didn't set rustup to nightly you need to do:

```shell
rustup override set nightly
```

## run project

```shell
cargo build --release
```

```shell
cargo run
```

With watch

```shell
cargo watch -x run
```

Watch chages in only the src folder use:

```shell
cargo watch -c -w src -x run
```
