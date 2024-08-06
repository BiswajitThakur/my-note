# [String::with_capacity()](https://doc.rust-lang.org/std/string/struct.String.html#method.with_capacity)

```rust
impl String {

    #[cfg(not(no_global_oom_handling))]
    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    #[must_use]
    pub fn with_capacity(capacity: usize) -> String {
        String { vec: Vec::with_capacity(capacity) }
    }

}
```
