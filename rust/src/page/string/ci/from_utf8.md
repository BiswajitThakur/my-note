# from_utf8

```rust
impl String {

    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn from_utf8(vec: Vec<u8>) -> Result<String, FromUtf8Error> {
        match str::from_utf8(&vec) {
            Ok(..) => Ok(String { vec }),
            Err(e) => Err(FromUtf8Error { bytes: vec, error: e }),
        }
    }

}
```

## Example

Basic usage:

```rust
// some bytes, in a vector
let sparkle_heart = vec![240, 159, 146, 150];

// We know these bytes are valid, so we'll use `unwrap()`.
let sparkle_heart = String::from_utf8(sparkle_heart).unwrap();

assert_eq!("💖", sparkle_heart);
```

Incorrect bytes:

```rust
// some invalid bytes, in a vector
let sparkle_heart = vec![0, 159, 146, 150];

assert!(String::from_utf8(sparkle_heart).is_err());
```
