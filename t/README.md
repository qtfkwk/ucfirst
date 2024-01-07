This is the long-awaited Rust crate that solves the problem of uppercasing the
first letter of a string in Rust.

Full disclosure: this solution was shamelessly lifted from the
[leading response over on StackOverflow][so] by [Shepmaster].

```rust
let lc_str = "apple";

// Using FromStr/parse and Display/to_string methods:
use ucfirst::Ucfirst;
let uc_str = lc_str.parse::<Ucfirst>().unwrap().to_string();
assert_eq!(uc_str, "Apple");

// Using ucfirst function:
use ucfirst::ucfirst;
let uc_str = ucfirst(lc_str);
assert_eq!(uc_str, "Apple");
```

See also the [`ccase`] CLI utility and [`convert_case`] library crates.

[so]: https://stackoverflow.com/a/38406885
[shepmaster]: https://stackoverflow.com/users/155423/shepmaster
[`ccase`]: https://crates.io/crates/ccase
[`convert_case`]: https://crates.io/crates/convert_case

