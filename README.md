# ConstDefault Trait

A `Default`-like trait and derive macros for `const` evaluation contexts.

This crate defines the `ConstDefault` trait and implements it for
Rust primitives, prelude types, tuples and arrays. Furthermore it
provides a derive macro so that users can implement `ConstDefault`
easily for their custom types.

- 100% safe Rust
- `no_std` compatible
- Full macro hygiene
- Rust Edition 2018

## Usage

Add
```toml
[dependencies]
const_default_2 = { version = "0.1", features = ["derive"] }
```
to your `Cargo.toml` and start using it.

## Example

```rust
#[derive(ConstDefault, Debug, Default, PartialEq)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

fn main() {
    assert_eq!(
        <Color as ConstDefault>::DEFAULT,
        Color::default(),
    );
}
```
