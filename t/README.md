This is the long-awaited Rust crate that solves the problem of uppercasing the
first letter of a string in Rust.

Full disclosure: this solution was shamelessly lifted from the
[leading response over on StackOverflow][so] by [Shepmaster].

```rust
use ucfirst::ucfirst;

assert_eq!(ucfirst("apple"), "Apple");
```

See also the [`ccase`] CLI utility and [`convert_case`] library crates.

[so]: https://stackoverflow.com/a/38406885
[shepmaster]: https://stackoverflow.com/users/155423/shepmaster
[`ccase`]: https://crates.io/crates/ccase
[`convert_case`]: https://crates.io/crates/convert_case

