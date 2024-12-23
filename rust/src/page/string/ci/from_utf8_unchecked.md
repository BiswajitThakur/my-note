# from_utf8_unchecked

```rust
impl String {

    #[inline]
    #[must_use]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub unsafe fn from_utf8_unchecked(bytes: Vec<u8>) -> String {
        String { vec: bytes }
    }

}
```

Converts a vector of bytes to a String without checking that the string contains valid UTF-8.

This function is unsafe because it does not check that the bytes passed to it are valid UTF-8. If this constraint is violated, it may cause memory unsafety issues with future users of the String, as the rest of the standard library assumes that Strings are valid UTF-8.

## Examples

```rust
// some bytes, in a vector
let sparkle_heart = vec![240, 159, 146, 150];

let sparkle_heart = unsafe {
    String::from_utf8_unchecked(sparkle_heart)
};

assert_eq!("💖", sparkle_heart);
```
