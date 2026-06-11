# Spiral Browser — Common Errors and Solutions

## Build Errors

### Missing Dependencies (Linux)
```
error: failed to run custom build script for `font-sys`
```
**Solution:**
```bash
sudo apt-get install -y libfontconfig1-dev libfreetype-dev pkg-config
```

### Missing Dependencies (macOS)
```
error: can't find pkg-config
```
**Solution:**
```bash
xcode-select --install
# or
brew install pkg-config
```

### Missing Dependencies (Windows)
```
error: linker `link.exe` not found
```
**Solution:** Install Visual Studio Build Tools with "Desktop development with C++" workload.

### Rust Version Too Old
```
error[E0658]: use of unstable library feature
```
**Solution:**
```bash
rustup update stable
```

---

## Test Errors

### IPC Deserialization Failure
```
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Io(Custom { kind: UnexpectedEof, error: "failed to fill whole buffer" })'
```
**Solution:** Check message framing. Ensure length prefix is correct. Verify bincode version matches between sender and receiver.

### Platform-Specific Test Failure
```
thread 'test_linux_sandbox' panicked at 'assertion failed'
```
**Solution:** Ensure `#[cfg(target_os = "linux")]` guard is present. Skip test on other platforms.

### Test Timeout
```
test result: FAILED. 0 passed; 1 failed; 0 ignored
```
**Solution:** Check for deadlocks in async code. Ensure tokio runtime is properly shut down.

---

## Runtime Errors

### IPC Connection Refused
```
Error: Connection refused (os error 111)
```
**Solution:**
- Check if browser process is running
- Verify socket path exists and has correct permissions
- On Linux: check `$XDG_RUNTIME_DIR/spiral/`

### Named Pipe Not Found (Windows)
```
Error: The system cannot find the file specified. (os error 2)
```
**Solution:**
- Verify named pipe path: `\\.\pipe\spiral-{pid}`
- Check if browser process is running
- Ensure correct permissions

### GPU Adapter Not Found
```
Error: No suitable GPU adapter found
```
**Solution:**
- Update GPU drivers
- Check Vulkan/Metal/DX12 support
- Try software rendering fallback

### Font Not Found
```
Error: Font not found for glyph
```
**Solution:**
- Ensure system fonts are installed
- Check fontconfig (Linux) or font cache
- Use bundled fallback font

---

## Sandbox Errors

### Landlock Not Supported (Linux)
```
Error: Function not implemented (os error 38)
```
**Solution:**
- Ensure kernel 5.13+ for Landlock
- Fallback to seccomp-bpf only

### Seatbelt Profile Rejected (macOS)
```
Error: sandbox_init: invalid operation
```
**Solution:**
- Verify sandbox profile syntax
- Ensure entitlements are correct
- Test with `sandbox-exec` command

### Restricted Token Failed (Windows)
```
Error: Access denied
```
**Solution:**
- Ensure running as standard user (not admin)
- Check Job Object permissions
- Verify integrity level settings

---

## IPC Errors

### Message Too Large
```
Error: Message size exceeds limit (10MB)
```
**Solution:**
- Increase message size limit in config
- Split large messages into chunks
- Use streaming for large data

### Message Type Mismatch
```
Error: Unknown message variant: 42
```
**Solution:**
- Ensure sender and receiver use same `IPCMessage` definition
- Check for version mismatch
- Regenerate serialization code

### Buffer Overflow
```
Error: Buffer overflow in deserialization
```
**Solution:**
- Validate message size before deserialization
- Add size limits to message framing
- Use bounded deserialization

---

## Layout Errors

### Infinite Layout Loop
```
test test_infinite_loop ... FAILED (timeout)
```
**Solution:**
- Check for circular dependencies in layout
- Add max iteration limit to layout engine
- Detect zero-size containers

### Flexbox Overflow
```
assertion failed: total_size <= container_size
```
**Solution:**
- Handle `overflow: hidden/scroll/auto`
- Check min/max constraints
- Verify Taffy integration

### Grid Template Invalid
```
Error: Invalid grid template
```
**Solution:**
- Validate template syntax
- Handle `auto`, `fr`, `minmax()` correctly
- Fallback to `auto` for invalid values

---

## Network Errors

### DNS Resolution Failed
```
Error: failed to lookup address: nodename nor servname provided
```
**Solution:**
- Check DNS server configuration
- Verify network connectivity
- Fallback to system resolver

### TLS Handshake Failed
```
Error: TLS handshake failed
```
**Solution:**
- Check certificate validity
- Verify rustls configuration
- Update root certificates

### Connection Timeout
```
Error: connection timed out
```
**Solution:**
- Increase timeout value
- Check proxy configuration
- Verify server availability

---

## Rendering Errors

### Vello Pipeline Error
```
Error: Vello pipeline creation failed
```
**Solution:**
- Check GPU driver version
- Verify wgpu adapter selection
- Try different backends (Vulkan/Metal/DX12)

### Texture Allocation Failed
```
Error: Out of GPU memory
```
**Solution:**
- Reduce texture atlas size
- Implement texture eviction
- Check for memory leaks

### Swap Chain Error
```
Error: Swap chain lost
```
**Solution:**
- Recreate swap chain on resize
- Handle window minimization
- Check for GPU reset

---

## Debugging Tips

### Enable Logging
```bash
RUST_LOG=debug cargo run     # debug level
RUST_LOG=trace cargo run     # trace level (very verbose)
```

### Check Backtrace
```bash
RUST_BACKTRACE=1 cargo run
RUST_BACKTRACE=full cargo run
```

### Profile Performance
```bash
cargo install flamegraph
cargo flamegraph
```

### Memory Analysis
```bash
cargo install cargo-valgrind
cargo valgrind
```
