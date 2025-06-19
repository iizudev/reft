# ref

## The Problem

Rust's `AsRef<T>` (as well as `AsMut<T>`) isn't implemented for `&T` (or `&mut T`).

Rust's borrowing system is powerful, but sometimes we need to abstract over references without tying our code to &T

```rust
struct MyStruct;

fn do_something<T: AsRef<MyStruct>>(t: T) { 
    todo!("do something very generic")
}

fn main() {
    let t = MyStruct;

    do_something(&t); // -> Won't compile, since AsRef is not implemented for &MyStruct!
    do_something(reft::Ref::new(&t)); // -> Now works!
}
```

## How to use

Add this to your `Cargo.toml`:

```toml
[dependencies]
reft = "*" # check for the latest version at crates.io
```

Or run:

```bash
cargo add reft
```

## How to contribute

Fork repository, make changes, and send us a pull request.

We will review your changes and apply them to the main
branch shortly, provided they don't violate our quality standards.

## License

This project is dual-licensed under:

- [MIT License](LICENSE-MIT)
- [Apache 2.0 License](LICENSE-APACHE-2.0)

You may choose either license at your option.
