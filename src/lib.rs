use num_traits::Num;
use tap::Pipe;

/// Defines a prefix format.
#[derive(Debug)]
pub struct PrefixFmt<'a> {
    pub prefix: &'a str,
    pub radix: u32,
}

/// '0x' prefix for hexadecimal numbers
pub const HEX: PrefixFmt = PrefixFmt {
    prefix: "0x",
    radix: 16,
};
/// '0o' prefix for octal numbers
pub const OCT: PrefixFmt = PrefixFmt {
    prefix: "0o",
    radix: 8,
};

/// '0b' prefix for binary numbers
pub const BIN: PrefixFmt = PrefixFmt {
    prefix: "0b",
    radix: 2,
};

/// '' prefix for decimal numbers
pub const DEC: PrefixFmt = PrefixFmt {
    prefix: "",
    radix: 10,
};

/// Trait for parsing prefixed numbers
pub trait PrefixParse {
    /// Parse a number prefixed with `0x`, `0o`, and `0b`
    ///
    /// # Example
    /// ```rust
    /// use prefix_parse::PrefixParse;
    ///
    /// assert_eq!(u32::parse("0x10"), Ok(16));
    /// assert_eq!(u32::parse("0o10"), Ok(8));
    /// assert_eq!(u32::parse("0b10"), Ok(2));
    /// assert_eq!(u32::parse("10"), Ok(10));
    /// ```
    fn parse(src: &str) -> Result<Self, ParseError<Self>>
    where
        Self: Sized + Num,
    {
        // SAFETY: if src is a valid UTF-8 string, and we strip no multibyte characters from the start,
        // then the remaining string will be valid UTF-8
        match src.as_bytes() {
            [b'0', b'x', rest @ ..] => {
                Self::from_str_radix(unsafe { str::from_utf8_unchecked(rest) }, 16)
                    .map_err(ParseError::RadixParseFailed)
            }
            [b'0', b'o', rest @ ..] => {
                Self::from_str_radix(unsafe { str::from_utf8_unchecked(rest) }, 8)
                    .map_err(ParseError::RadixParseFailed)
            }
            [b'0', b'b', rest @ ..] => {
                Self::from_str_radix(unsafe { str::from_utf8_unchecked(rest) }, 2)
                    .map_err(ParseError::RadixParseFailed)
            }
            _ => Self::from_str_radix(src, 10).map_err(ParseError::RadixParseFailed),
        }
    }

    /// Parse a number with a custom prefix
    ///
    /// # Example
    /// ```
    /// use prefix_parse::{PrefixParse, ParseError, PrefixFmt, HEX};
    ///
    /// assert_eq!(u32::parse_with(&HEX, "0x10"), Ok(16));
    ///
    /// let custom_fmt = PrefixFmt {
    ///     prefix: "0z",
    ///     radix: 36,
    /// };
    /// assert_eq!(u32::parse_with(&custom_fmt, "0z1jz"), Ok(2015));
    /// ```
    fn parse_with(fmt: &PrefixFmt, src: &str) -> Result<Self, ParseError<Self>>
    where
        Self: Sized + Num,
    {
        src.strip_prefix(fmt.prefix)
            .ok_or(ParseError::NoPrefixMatch)?
            .pipe(|rest| Self::from_str_radix(rest, fmt.radix))
            .map_err(ParseError::RadixParseFailed)
    }
}

/// Implementation for all number types that implement the `Num` interface.
impl<T: Num> PrefixParse for T {}

/// Error type for `PrefixParse`
#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum ParseError<T: Num> {
    #[error("No Prefix Match")]
    NoPrefixMatch,
    #[error(transparent)]
    RadixParseFailed(T::FromStrRadixErr),
}
