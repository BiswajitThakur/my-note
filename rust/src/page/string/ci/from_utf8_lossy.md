# from_utf8_lossy

Converts a slice of bytes to a string, including invalid characters.

```rust
impl String {

    #[must_use]
    #[cfg(not(no_global_oom_handling))]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn from_utf8_lossy(v: &[u8]) -> Cow<'_, str> {
        let mut iter = v.utf8_chunks();

        let first_valid = if let Some(chunk) = iter.next() {
            let valid = chunk.valid();
            if chunk.invalid().is_empty() {
                debug_assert_eq!(valid.len(), v.len());
                return Cow::Borrowed(valid);
            }
            valid
        } else {
            return Cow::Borrowed("");
        };

        const REPLACEMENT: &str = "\u{FFFD}";

        let mut res = String::with_capacity(v.len());
        res.push_str(first_valid);
        res.push_str(REPLACEMENT);

        for chunk in iter {
            res.push_str(chunk.valid());
            if !chunk.invalid().is_empty() {
                res.push_str(REPLACEMENT);
            }
        }

        Cow::Owned(res)
    }

}
```
