# [string](https://doc.rust-lang.org/std/string/struct.String.html)

### Struct `std::string::String`

```rust
#[derive(PartialEq, PartialOrd, Eq, Ord)]
#[stable(feature = "rust1", since = "1.0.0")]
#[cfg_attr(not(test), lang = "String")]
pub struct String {
    vec: Vec<u8>,
}
```
