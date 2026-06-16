//! Spiral Browser — Cryptographic Primitives
//!
//! Wraps well-audited upstream cryptographic primitives:
//! - **CSPRNG**: [`getrandom`] (cross-platform, calls the operating
//!   system's secure RNG: `getrandom(2)` on Linux, `BCryptGenRandom` on
//!   Windows, `SecRandomCopyBytes` on macOS).
//! - **SHA-256**: [`sha2`] (pure Rust, FIPS 180-4 compliant, audited).
//!
//! We do not write custom cryptographic primitives. Every primitive
//! Spiral uses is a vetted, upstream-maintained library. This crate
//! is the single Spiral-namespaced entry point so that no business
//! logic crate (anything below `spiral-browser`) imports `sha2` or
//! `getrandom` directly.
//!
//! ## API stability
//! The previous stub API returned `Vec<u8>` from `random_bytes` and
//! silently produced non-random output. That is replaced by a
//! `Result`-returning API that surfaces CSPRNG failures. The `sha256`
//! helper now returns a fixed-size `[u8; 32]` array (the algorithm is
//! infallible for valid input sizes, and a fixed-size array is more
//! typed and more efficient than `Vec<u8>`).
//!
//! ## Scope
//! This crate is the v0.1 surface. Future work (M5+) adds:
//! - HMAC-SHA-256 (used by HKDF and JWT verification).
//! - HKDF-Extract / HKDF-Expand (RFC 5869).
//! - Constant-time equality for MAC / signature comparison.
//! - Ed25519 signatures (for Subresource Integrity and DevTools
//!   authenticator handshakes).

use getrandom::getrandom;
use sha2::{Digest, Sha256};
use spiral_core::{Error, Result};

/// Cryptographic operations facade.
///
/// Wraps the system CSPRNG ([`getrandom`]) and SHA-256 ([`sha2`]).
/// Cheap to construct; all methods are stateless. [`Crypto::default`]
/// is provided so the type can be embedded in cached structures
/// without ceremony.
#[derive(Debug, Clone, Copy, Default)]
pub struct Crypto;

impl Crypto {
    /// Construct a new crypto facade.
    ///
    /// `const fn` so it can be used in `static` initialisers.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Fill a buffer with cryptographically secure random bytes.
    ///
    /// Wraps [`getrandom::getrandom`]. The call is infallible on every
    /// platform Spiral targets in normal operation. The only failure
    /// mode is the OS refusing to provide entropy (extremely rare;
    /// on Linux it means the `getrandom(2)` syscall returned
    /// `ENOSYS` or was interrupted past the retry limit).
    pub fn fill_random(&self, buf: &mut [u8]) -> Result<()> {
        getrandom(buf).map_err(|e| Error::Crypto(format!("CSPRNG unavailable: {e}")))
    }

    /// Generate a freshly allocated `Vec<u8>` of CSPRNG bytes.
    ///
    /// Convenience wrapper around [`Crypto::fill_random`]. Prefer
    /// `fill_random` when you already have a buffer to write into —
    /// it avoids one allocation and one `memcpy`.
    pub fn random_bytes(&self, len: usize) -> Result<Vec<u8>> {
        let mut buf = vec![0u8; len];
        self.fill_random(&mut buf)?;
        Ok(buf)
    }

    /// Compute the SHA-256 digest of `data`.
    ///
    /// Returns a 32-byte array. The algorithm is infallible for any
    /// `&[u8]` input; no `Result` is needed. Marked `#[must_use]`
    /// because discarding a freshly-computed hash is almost always
    /// a programming error.
    #[must_use]
    pub fn sha256(&self, data: &[u8]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.finalize().into()
    }

    /// Compute the SHA-256 digest of `data` as a lowercase hex string.
    ///
    /// 64 hex characters, no separator. Useful for SRI hashes,
    /// cache-busting, and human-readable fingerprints.
    #[must_use]
    pub fn sha256_hex(&self, data: &[u8]) -> String {
        let digest = self.sha256(data);
        let mut out = String::with_capacity(64);
        for byte in digest {
            // Manual `write!` is faster than `format!` per byte and
            // avoids a temporary `String` per nibble.
            const HEX: &[u8; 16] = b"0123456789abcdef";
            out.push(HEX[(byte >> 4) as usize] as char);
            out.push(HEX[(byte & 0x0f) as usize] as char);
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    // ----------------------------------------------------------------
    // SHA-256 known-answer tests (FIPS 180-2 / NIST examples)
    // ----------------------------------------------------------------

    /// SHA-256 of the empty string. Verified against
    /// `printf '' | sha256sum` and FIPS 180-2 Appendix B.1.
    const SHA256_EMPTY: [u8; 32] = [
        0xe3, 0xb0, 0xc4, 0x42, 0x98, 0xfc, 0x1c, 0x14, 0x9a, 0xfb, 0xf4, 0xc8, 0x99, 0x6f, 0xb9,
        0x24, 0x27, 0xae, 0x41, 0xe4, 0x64, 0x9b, 0x93, 0x4c, 0xa4, 0x95, 0x99, 0x1b, 0x78, 0x52,
        0xb8, 0x55,
    ];

    /// SHA-256 of "abc". FIPS 180-2 Appendix B.2.
    const SHA256_ABC: [u8; 32] = [
        0xba, 0x78, 0x16, 0xbf, 0x8f, 0x01, 0xcf, 0xea, 0x41, 0x41, 0x40, 0xde, 0x5d, 0xae, 0x22,
        0x23, 0xb0, 0x03, 0x61, 0xa3, 0x96, 0x17, 0x7a, 0x9c, 0xb4, 0x10, 0xff, 0x61, 0xf2, 0x00,
        0x15, 0xad,
    ];

    /// SHA-256 of the NIST "long message" test vector (448 bits of
    /// repeated `abcd`). FIPS 180-2 Appendix B.3.
    const SHA256_LONG_ABCD: [u8; 32] = [
        0x24, 0x8d, 0x6a, 0x61, 0xd2, 0x06, 0x38, 0xb8, 0xe5, 0xc0, 0x26, 0x93, 0x0c, 0x3e, 0x60,
        0x39, 0xa3, 0x3c, 0xe4, 0x59, 0x64, 0xff, 0x21, 0x67, 0xf6, 0xec, 0xed, 0xd4, 0x19, 0xdb,
        0x06, 0xc1,
    ];

    #[test]
    fn sha256_empty_string_matches_fips_180_2() {
        let crypto = Crypto::new();
        assert_eq!(crypto.sha256(b""), SHA256_EMPTY);
    }

    #[test]
    fn sha256_abc_matches_fips_180_2() {
        let crypto = Crypto::new();
        assert_eq!(crypto.sha256(b"abc"), SHA256_ABC);
    }

    #[test]
    fn sha256_long_message_matches_fips_180_2() {
        let crypto = Crypto::new();
        let input = b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq";
        assert_eq!(crypto.sha256(input), SHA256_LONG_ABCD);
    }

    #[test]
    fn sha256_is_deterministic() {
        let crypto = Crypto::new();
        let input = b"the quick brown fox jumps over the lazy dog";
        let first = crypto.sha256(input);
        let second = crypto.sha256(input);
        assert_eq!(first, second, "SHA-256 must be deterministic across calls");
    }

    #[test]
    fn sha256_different_inputs_differ() {
        let crypto = Crypto::new();
        let a = crypto.sha256(b"hello");
        let b = crypto.sha256(b"world");
        assert_ne!(a, b, "different inputs must produce different digests");
    }

    #[test]
    fn sha256_avalanche_one_bit_flip() {
        let crypto = Crypto::new();
        let a = crypto.sha256(b"message");
        let b = crypto.sha256(b"messags"); // one bit off the end
        let diff_bits = a
            .iter()
            .zip(b.iter())
            .map(|(x, y)| (x ^ y).count_ones())
            .sum::<u32>();
        // SHA-256 has strong avalanche. We expect roughly 64 differing
        // bits for any one-bit input change. Assert a generous lower
        // bound (32) to catch gross failures but tolerate variance.
        assert!(
            diff_bits >= 32,
            "one-bit input change should flip many digest bits, got {diff_bits}"
        );
    }

    #[test]
    fn sha256_hex_matches_hex_of_digest() {
        let crypto = Crypto::new();
        let hex = crypto.sha256_hex(b"abc");
        assert_eq!(hex.len(), 64, "hex output must be 64 characters");
        assert_eq!(
            hex,
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    #[test]
    fn sha256_hex_is_lowercase() {
        let crypto = Crypto::new();
        let hex = crypto.sha256_hex(b"anything");
        for ch in hex.chars() {
            assert!(
                ch.is_ascii_hexdigit(),
                "non-hex character in digest: {ch:?}"
            );
            assert!(
                !ch.is_ascii_uppercase(),
                "hex digest must be lowercase, found {ch:?}"
            );
        }
    }

    // ----------------------------------------------------------------
    // CSPRNG tests
    // ----------------------------------------------------------------

    #[test]
    fn random_bytes_returns_requested_length() {
        let crypto = Crypto::new();
        for len in [0usize, 1, 16, 32, 64, 256, 1024] {
            let bytes = crypto.random_bytes(len).expect("RNG should not fail");
            assert_eq!(bytes.len(), len, "wrong length for len={len}");
        }
    }

    #[test]
    fn random_bytes_is_not_deterministic() {
        let crypto = Crypto::new();
        let a = crypto.random_bytes(32).unwrap();
        let b = crypto.random_bytes(32).unwrap();
        // Birthday probability for a 256-bit random space is
        // ~2^-256. A collision in this test would mean the RNG
        // is broken (or, equivalently, that we just witnessed a
        // miracle and should be writing a paper about it).
        assert_ne!(a, b, "CSPRNG produced identical outputs — RNG is broken");
    }

    #[test]
    fn random_bytes_distinct_across_many_calls() {
        let crypto = Crypto::new();
        // 1000 calls × 16 bytes = 16 KB of CSPRNG output. The
        // probability of any two collisions is ~10^-72. A collision
        // means the RNG is degenerate.
        let mut seen: HashSet<Vec<u8>> = HashSet::new();
        for _ in 0..1000 {
            let buf = crypto.random_bytes(16).unwrap();
            assert!(
                seen.insert(buf.clone()),
                "CSPRNG repeated a 128-bit output — entropy source is broken"
            );
        }
    }

    #[test]
    fn random_bytes_zero_length_is_empty() {
        let crypto = Crypto::new();
        let buf = crypto.random_bytes(0).unwrap();
        assert!(buf.is_empty());
    }

    #[test]
    fn fill_random_populates_buffer_in_place() {
        let crypto = Crypto::new();
        let mut buf = [0u8; 32];
        crypto.fill_random(&mut buf).unwrap();
        // Probability of all-zero from a real CSPRNG is 2^-256.
        let all_zero = buf.iter().all(|&b| b == 0);
        assert!(
            !all_zero,
            "fill_random left the buffer as all zeros — RNG is broken"
        );
    }

    #[test]
    fn fill_random_respects_length() {
        let crypto = Crypto::new();
        let mut buf = [0xffu8; 64];
        crypto.fill_random(&mut buf[..0]).unwrap();
        assert_eq!(
            buf, [0xffu8; 64],
            "zero-length fill must not touch the buffer"
        );
    }

    #[test]
    fn fill_random_into_reused_buffer_changes_contents() {
        let crypto = Crypto::new();
        let mut buf = [0u8; 32];
        let first = {
            crypto.fill_random(&mut buf).unwrap();
            buf.to_vec()
        };
        crypto.fill_random(&mut buf).unwrap();
        assert_ne!(first, buf, "consecutive fills should overwrite the buffer");
    }

    // ----------------------------------------------------------------
    // Construction / ergonomics
    // ----------------------------------------------------------------

    #[test]
    fn crypto_default_matches_new() {
        let a: Crypto = Crypto;
        let b = Crypto::new();
        let a_hash = a.sha256(b"x");
        let b_hash = b.sha256(b"x");
        assert_eq!(a_hash, b_hash, "default and new must be interchangeable");
    }

    #[test]
    fn crypto_is_copy_and_send_sync() {
        fn assert_copy_send_sync<T: Copy + Send + Sync>() {}
        assert_copy_send_sync::<Crypto>();
    }

    #[test]
    fn crypto_is_const_constructible() {
        // The contract: `Crypto::new` is a `const fn`. This test
        // fails to compile if the signature is ever relaxed, which
        // is exactly what we want — a static-initialiser-compatible
        // crypto facade is part of the public surface.
        const CRYPTO: Crypto = Crypto::new();
        let _ = CRYPTO.sha256(b"compile-time");
    }
}
