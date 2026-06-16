//! JS string utilities.
//!
//! ECMAScript strings are sequences of UTF-16 code units. Vortex
//! internally stores them as Rust `String` (UTF-8) and converts
//! to/from UTF-16 only when the spec demands it (e.g. `String.fromCharCode`,
//! `String.prototype.charCodeAt`).

/// Get the UTF-16 code unit at index `i`.
///
/// Returns `None` if the index is out of bounds or the string is shorter
/// than `i` UTF-16 units.
pub fn char_code_at(s: &str, i: usize) -> Option<u16> {
    s.encode_utf16().nth(i)
}

/// Get the number of UTF-16 code units in the string.
pub fn utf16_len(s: &str) -> usize {
    s.encode_utf16().count()
}

/// Create a JS string from a single UTF-16 code unit.
pub fn from_char_code(code: u16) -> String {
    char::decode_utf16(std::iter::once(code))
        .map(|r| r.unwrap_or(char::REPLACEMENT_CHARACTER))
        .collect()
}

/// JS string concatenation (§6.1.5.1).
///
/// In JS, `a + b` where either operand is a string results in string
/// concatenation with ToString applied to the other operand.
/// This is a helper; the actual `+` operator logic is in the VM.
pub fn concat(a: &str, b: &str) -> String {
    let mut s = String::with_capacity(a.len() + b.len());
    s.push_str(a);
    s.push_str(b);
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_code_at() {
        assert_eq!(char_code_at("hello", 0), Some(b'h' as u16));
        assert_eq!(char_code_at("hello", 4), Some(b'o' as u16));
        assert_eq!(char_code_at("hello", 5), None);
        // Emoji is surrogate pair in UTF-16
        assert_eq!(char_code_at("a\u{1F600}b", 0), Some(b'a' as u16));
    }

    #[test]
    fn test_utf16_len() {
        assert_eq!(utf16_len("hello"), 5);
        assert_eq!(utf16_len(""), 0);
    }

    #[test]
    fn test_from_char_code() {
        assert_eq!(from_char_code(65), "A");
        // BMP characters work directly
        assert_eq!(from_char_code(0x00E9), "é");
    }
}
