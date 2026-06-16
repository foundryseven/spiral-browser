//! `console` builtin — `console.log`, `console.info`, `console.warn`,
//! `console.error`, `console.table`, `console.time`, `console.timeEnd`.

use crate::value::JsValue;

/// Format a list of JS values for console output (§2.1.1 — spec-like).
///
/// The format is intentionally simple: space-separated ToString of each
/// argument, with `%c`, `%d`, `%s`, `%o` substitution support deferred
/// to Phase 2.
pub fn format_args(args: &[JsValue]) -> String {
    args.iter()
        .map(|v| v.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_args() {
        let args = vec![
            JsValue::String("hello".into()),
            JsValue::Number(42.0),
            JsValue::Bool(true),
        ];
        assert_eq!(format_args(&args), "hello 42 true");
    }

    #[test]
    fn test_format_empty() {
        assert_eq!(format_args(&[]), "");
    }
}
