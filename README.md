# 📦 prefix_parse
A lightweight crate for parsing numeric strings with radix-indicating prefixes like `0x`, `0o`, and `0b`, or any other custom prefix, supporting any type that implements [num_traits::Num](https://docs.rs/num-traits/latest/num_traits/trait.Num.html). This includes all base rust numeric types.

The implementation relies on [from_str_radix](https://docs.rs/num-traits/0.2.19/num_traits/trait.Num.html#tymethod.from_str_radix) internally, so the same limitations and capabilities present there apply.

## ✨ Features
- ✅ Parse hexadecimal (`0x`), octal (`0o`), binary (`0b`), and decimal numbers.
- ✅ Support for custom prefix formats and arbitrary radices.
- ✅ Works with any type implementing [Num](https://docs.rs/num-traits/latest/num_traits/trait.Num.html), including `u32`, `i64`, `u8`, `usize`, etc.

### Auto Detect common prefixes
Handles, hexadecimal (`0x`), octal (`0o`), binary (`0b`), and decimal numbers ('').
```rust
use prefix_parse::PrefixParse;

assert_eq!(u32::parse("0x10"), Ok(16));
assert_eq!(u32::parse("0o10"), Ok(8));
assert_eq!(u32::parse("0b10"), Ok(2));
assert_eq!(u32::parse("10"), Ok(10));
```

### Built-in Prefixes
Handles preconfigured prefixes.
```rust
use prefix_parse::{PrefixParse, HEX, OCT, BIN, DEC};

assert_eq!(u32::parse_with(&HEX, "0xFF"), Ok(255));
assert_eq!(u32::parse_with(&OCT, "0o77"), Ok(63));
assert_eq!(u32::parse_with(&BIN, "0b1010"), Ok(10));
assert_eq!(u32::parse_with(&DEC, "42"), Ok(42));
```

### Custom Prefixes
Allows definition of custom prefixes.
```rust
use prefix_parse::{PrefixFmt, PrefixParse};
let base36 = PrefixFmt {
    prefix: "0z",
    radix: 36,
};

assert_eq!(u32::parse_with(&base36, "0z1jz"), Ok(2015));
```

## 🔧 Extending
Any new type that implements [Num](https://docs.rs/num-traits/latest/num_traits/trait.Num.html) from [num_traits](https://crates.io/crates/num-traits) will automatically implement this.

## 📝 License
MIT or Apache-2.0 — your choice.

## 💬 Feedback
Contributions welcome!
