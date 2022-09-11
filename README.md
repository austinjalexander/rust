# rust

## The Rust Programming Language

### Install and Run

```shell
curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh
```

```shell
rustc --version
# rustc 1.63.0 (4b91a6ea7 2022-08-08)
cargo --version
# cargo 1.63.0 (fd9c4297c 2022-07-01)
```

```shell
rustup update
```

```shell
rustup docs
rustup docs --book
```

```shell
rustc main.rs && ./main
```

```shell
cargo new hello_cargo
cd hello_cargo
cargo build
./target/debug/hello_cargo
```

```shell
cargo check
```

```shell
cargo run
```

```shell
cargo build --release
./target/release/hello_cargo
```

```shell
cargo update
```

```shell
cargo doc --open
```
