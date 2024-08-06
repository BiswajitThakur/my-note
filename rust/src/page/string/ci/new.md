# String::new()

create empty string.

```rust
impl String {

    #[inline]
    #[rustc_const_stable(feature = "const_string_new", since = "1.39.0")]
    #[stable(feature = "rust1", since = "1.0.0")]
    #[must_use]
    pub const fn new() -> String {
        String { vec: Vec::new() }
    }

}
```

## Example

```rust
let s = String::new();
println!("{}", s);
```
