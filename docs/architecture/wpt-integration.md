# Web Platform Tests (WPT) Integration Blueprint

This document defines the strategy for integrating the official W3C [Web Platform Tests](https://github.com/web-platform-tests/wpt) into Spiral Browser. Rather than deferring compliance verification to Phase 9, this blueprint establishes how WPT is shifted forward to support active development of `spiral-fmt` and `spiral-dom`.

---

## 1. Objectives

* **Early Verification:** Prevent architectural drift in the DOM and layout engines by catching spec-non-compliance during early feature design.
* **Fast CI Feedback:** Avoid running the entire millions-of-tests WPT corpus on every PR; subset tests to compile and run in under 30 seconds.
* **Strict Regression Gates:** Assert that once a WPT test sub-suite passes, it remains passing (zero-regression policy).

---

## 2. Ingestion & Subsetting

The workspace root maintains a `tests/wpt/` directory. Rather than checking in the entire WPT git repository, Spiral uses a targeted ingestion model:

```
tests/wpt/
├── Cargo.toml                # WPT test runner crate
├── src/
│   └── main.rs               # Custom runner executable
├── tests/
│   ├── html/                 # Subsetted HTML parser tests
│   └── css/                  # Subsetted CSS syntax & sizing tests
└── config.toml               # Active test manifest and exclusions
```

### Subsetting Strategy

1. **Manifest Mapping:** We list the explicit directories of interest in `tests/wpt/config.toml` (e.g., `html/syntax/parsing`, `css/css-syntax`, `css/css-box`).
2. **Exclusion Lists:** Tests that rely on unimplemented features (like scripting, active network fetching, or specific media APIs) are allowlisted/denied in the manifest to keep the baseline green.
3. **WPT Sync Script:** A helper script (`./scripts/sync-wpt.sh`) downloads the latest test fixtures from the official WPT upstream repository for the scoped folders, serialising them locally into JSON files.

---

## 3. The Custom Test Runner

Because Spiral runs in a safe, highly modular Rust environment, we do not require a full WebDriver / browser-shell environment to test parsing and DOM mutation. We implement a lightweight, custom WPT runner in `tests/wpt/src/main.rs`:

* **Parser Tests (`spiral-fmt`):** Read HTML/CSS tokenisation and tree-construction JSON files from WPT. Ingest them directly, invoke `spiral_fmt::parse_html` or `spiral_fmt::parse_css`, and assert that the output tree or token stream matches the expected WPT JSON output.
* **DOM Tests (`spiral-dom`):** Run JS-free DOM node creation and traversal tests by mocking the environment and asserting that Node/Element relationships align with WPT standards.

### Running WPT locally

To run the subset of Web Platform Tests:
```bash
cargo test -p wpt-runner
```

---

## 4. CI Integration

* WPT validation is run on every commit.
* The test runner returns a non-zero exit code if any active, un-excluded test fails.
* The status tracker ([implementation_tracker.md](file:///Users/james/spiral-browser/docs/implementation_tracker.md)) defines the target pass percentages per packet (e.g., 40% css-box pass rate). The runner checks these thresholds and exits 1 if they fall below target.
