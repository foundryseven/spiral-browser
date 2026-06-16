# Design — `spiral-context` Capability-Typed API Surface

**Status:** design pass complete (2026-06-15), user-approved
**Author:** implementer agent (synthesis from research passes)
**Crate:** `crates/spiral-context/`
**Phase:** M4 (skeleton) → M25 (runtime)
**Implements:** Bet 1 from `docs/architecture/design/shared-everything.md`

---

## 1. Purpose

`spiral-context` provides capability-based security for Spiral's
shared-everything multi-process renderer. One renderer process, N
typed-isolated contexts (one per origin). A context has a set of
*capabilities* — unforgeable tokens that grant access to specific
resources (filesystem, network, time, random). The default context
has no filesystem capability, no network capability. It can only
call the typed APIs we expose.

The security boundary is Rust's type system, not OS process walls.

---

## 2. Branded Types in Stable Rust

### 2.1 The brand

A brand is a *purely compile-time tag* attached to a type via
`PhantomData` so that two structurally identical values are treated
as different types by the type checker. The brand has no runtime
representation.

The canonical reference is the [GhostCell paper
(Yanovski et al., ICFP 2021)](https://plv.mpi-sws.org/rustbelt/ghostcell/)
which formally proves this is sound.

In Spiral, the *brand* is the origin. Two values of type
`Handle<'example_com, DomNode>` and `Handle<'bank_com, DomNode>`
are *different types* at compile time.

### 2.2 The `PhantomData<*const ()>` pattern

The raw pointer `*const ()` is `!Send + !Sync`. This is the
canonical way to make a wrapper "pinned to a single thread"
without using a `Mutex`. [Rustonomicon reference](https://doc.rust-lang.org/nomicon/phantom-data.html).

```rust
pub struct File {
    _handle: std::fs::File,
    _brand: PhantomData<*const ()>,  // !Send + !Sync, ZST
}
```

### 2.3 Invariant-lifetime brands

Ordinary `&'a T` is *covariant* in `'a` — a long-lived reference
can be silently shortened. The brand must be *invariant* to prevent
this:

```rust
#[derive(Debug)]
pub struct Brand<'brand> {
    _p: PhantomData<fn(&'brand ()) -> &'brand ()>,
}
```

The `fn(T) -> T` shape is invariant in `T`. Two brands with
different lifetimes cannot unify.

### 2.4 Unforgeable tokens

A capability token is a zero-sized type whose constructor is
`pub(crate)`:

```rust
pub struct FsCap { _private: () }

impl FsCap {
    pub(crate) fn grant() -> Self { Self { _private: () } }
}
```

Outside the `spiral-context` crate, `FsCap::grant` does not
exist. The token is `pub` (so consumers can hold and pass it)
but cannot be *created*.

---

## 3. Origin Isolation

### 3.1 The branded context

```rust
pub struct Context<'brand, Mode = InProcess> {
    origin: Origin,
    brand: PhantomData<*const ()>,           // !Send + !Sync
    _mode: PhantomData<Mode>,
    caps: CapabilitySet<'brand>,
}

pub enum InProcess {}
pub struct Escalated {
    ipc: spiral_ipc::Client,
}
```

The three `PhantomData` fields achieve:

1. `*const ()` → `!Send + !Sync`
2. `Brand<'brand>` → origin identity tied to a lifetime
3. `Mode` → runtime mode as a type-level tag

### 3.2 Arena-allocated handles

```rust
pub struct Handle<'brand, T: ?Sized> {
    index: u32,
    _brand: PhantomData<fn(&'brand T) -> &'brand T>,
}

pub struct Arena<'brand, T> {
    slots: Vec<Slot<T>>,
    _brand: PhantomData<fn(&'brand ()) -> &'brand ()>,
}

impl<'brand, T> Arena<'brand, T> {
    pub fn alloc(&mut self, value: T) -> Handle<'brand, T> {
        let index = self.slots.len() as u32;
        self.slots.push(Slot::new(value));
        Handle { index, _brand: PhantomData }
    }

    pub fn get(&self, h: Handle<'brand, T>) -> &T {
        &self.slots[h.index as usize].value
    }
}
```

`Handle<'a, DomNode>` and `Handle<'b, DomNode>` are different
types unless the lifetimes unify. Cross-origin access is a
compile error.

### 3.3 Cross-origin access attempt — COMPILE ERROR

```rust
fn cross_origin<'a, 'b>(
    dom_a: &Dom<'a>,
    dom_b: &Dom<'b>,
    h: Handle<'a, NodeData>,
) -> &NodeData {
    dom_b.nodes.get(h)  // ERROR: 'a cannot unify with 'b
}
```

---

## 4. The Capability Set

### 4.1 Struct of Options (not a HashSet)

```rust
pub struct CapabilitySet<'brand> {
    pub fs: Option<FsCap>,
    pub net: Option<NetCap>,
    pub clock: Option<ClockCap>,
    pub rng: Option<RngCap>,
    pub dom: DomCap<'brand>,       // required — every context has a DOM
    _brand: PhantomData<fn(&'brand ()) -> &'brand ()>,
}

impl<'brand> CapabilitySet<'brand> {
    pub fn empty(brand: Brand<'brand>) -> Self {
        Self {
            fs: None,
            net: None,
            clock: None,
            rng: None,
            dom: DomCap::new(brand),
            _brand: PhantomData,
        }
    }
}
```

### 4.2 The grant builder

```rust
#[derive(TypedBuilder)]
pub struct ContextBuilder<'brand> {
    origin: Origin,
    #[builder(default)]
    fs: Option<FsCap>,
    #[builder(default)]
    net: Option<NetCap>,
    #[builder(default)]
    clock: Option<ClockCap>,
    #[builder(default)]
    rng: Option<RngCap>,
    dom: DomCap<'brand>,
    #[builder(default = Mode::InProcess)]
    mode: Mode,
}
```

Usage from the browser runtime:

```rust
let brand = Brand::new();
let ctx = ContextBuilder::new()
    .origin(Origin::from_str("https://example.com")?)
    .dom(DomCap::new(brand))
    .build(brand);
```

### 4.3 The grant path

The grant flows downward:

1. The browser runtime (which has all authority) constructs a
   `Context`.
2. The runtime picks which capabilities to grant.
3. The runtime hands the resulting `Context` to Vortex, Gyre, etc.

Only the FS module can construct an `FsCap`. The FS module exposes
`FsModule::mount_for_origin(origin) -> FsCap` that the runtime
calls.

---

## 5. Capability-Specific APIs

### 5.1 Network without raw sockets

```rust
pub struct NetCap { _private: () }

impl NetCap {
    pub(crate) fn grant() -> Self { Self { _private: () } }

    pub async fn fetch(&self, url: &Url) -> NetResult<Vec<u8>> {
        spiral_net::dispatch(url).await
    }
}
```

The context sees `NetCap::fetch`, not `std::net::TcpStream`.

### 5.2 DOM without other origins' DOMs

```rust
pub struct DomCap<'brand> {
    arena: RefCell<Dom<'brand>>,
    _brand: PhantomData<fn(&'brand ()) -> &'brand ()>,
}

impl<'brand> DomCap<'brand> {
    pub fn append_child(
        &self,
        parent: Handle<'brand, NodeData>,
        child: NodeData,
    ) -> Handle<'brand, NodeData> {
        self.arena.borrow_mut().append_child(parent, child)
    }
}
```

The `Handle<'brand, NodeData>` parameter makes it impossible to
call `dom_a.append_child(handle_from_dom_b, ...)`.

---

## 6. The ContextOps Trait

A concrete `Context<'brand, Mode>` for the common case, with a
blanket impl of `ContextOps` for both modes:

```rust
pub trait ContextOps<'brand> {
    fn run_script(&self, src: &str) -> Result<Value, ScriptError>;
    fn hit_test(&self, point: Point) -> Option<NodeId>;
    fn dom(&self) -> &DomCap<'brand>;
    fn fetch(&self, url: &Url) -> impl Future<Output = NetResult<Vec<u8>>>;
}

impl<'brand> ContextOps<'brand> for Context<'brand, InProcess> {
    fn run_script(&self, src: &str) -> Result<Value, ScriptError> {
        self.isolate.run(src)
    }
    // ... direct method calls
}

impl<'brand> ContextOps<'brand> for Context<'brand, Escalated> {
    fn run_script(&self, src: &str) -> Result<Value, ScriptError> {
        self.escalated_ipc
            .as_ref()
            .unwrap()
            .request_sync(&BrowserToRenderer::RunScript { src: src.into() })
    }
    // ... IPC round-trips
}
```

Code that is generic over `C: ContextOps` works for both in-process
and escalated contexts.

---

## 7. The Escalation Path

### 7.1 Two modes

- **Default (InProcess):** single renderer process, N typed contexts.
  The type system is the security boundary.
- **Escalated:** the context runs in a separate OS process with
  Landlock+seccomp (Linux), Seatbelt (macOS), or Restricted Token
  (Windows). The type system AND the kernel are the security
  boundary.

### 7.2 Belt and braces

| Property | Enforced by | Strength |
|----------|-------------|----------|
| "Context A cannot read context B's DOM" | Rust brands + handle types | Strongest, compile-time |
| "Context A cannot escape the renderer process" | OS sandbox | Strong, kernel-level |
| "Context A cannot make syscalls" | seccomp-bpf / sandbox profile | Strong, kernel-level |
| "A script bug cannot reach `std::fs::File`" | `pub(crate)` constructor | Strong, type-level |

### 7.3 Escalated context is an IPC stub

The escalated `Context` holds a `spiral_ipc::Client` connection to a
child process that holds the real state. Every method call is a
serialise-and-send round-trip.

### 7.4 The lifecycle

1. Browser decides origin X should be escalated.
2. Browser runtime calls `spiral-sandbox::Sandbox::new()`.
3. Browser `fork()`s a child process.
4. Child constructs a `Context<'_, InProcess>` internally.
5. Parent constructs a `Context<'_, Escalated>` over IPC.
6. From the parent's perspective, the origin is just a `Context`.

---

## 8. Stable vs. Nightly

All designs use **stable Rust only**:

- `PhantomData<*const ()>` — stable
- `PhantomData<fn(&'brand ()) -> &'brand ()>` — stable
- `pub(crate)` / `pub(in path)` visibility — stable
- Lifetime branding — stable
- Lifetime branding — stable
- `typed-builder` pattern (inline, not a crate dep) — stable
- `make_guard!`-style fresh brand generation (~200 lines Spiral-native)

Not needed:

- `arbitrary_self_types` (nightly)
- `generic_associated_types` (nightly)
- `type_alias_impl_trait` (nightly)

---

## 9. M4 Skeleton Scope

### 9.1 Files to create

- `crates/spiral-context/Cargo.toml`
- `crates/spiral-context/src/lib.rs` — re-exports, module wiring
- `crates/spiral-context/src/brand.rs` — `Brand<'brand>`
- `crates/spiral-context/src/origin.rs` — `Origin` (stringly-typed initially)
- `crates/spiral-context/src/caps.rs` — `FsCap`, `NetCap`, `ClockCap`, `RngCap`, `CapabilitySet<'brand>`
- `crates/spiral-context/src/dom.rs` — `DomCap<'brand>`, `Dom<'brand>`, `NodeData`
- `crates/spiral-context/src/arena.rs` — `Arena<'brand, T>`
- `crates/spiral-context/src/context.rs` — `Context<'brand, Mode>`, `InProcess`, `Escalated`, `ContextOps<'brand>`
- `crates/spiral-context/src/builder.rs` — `ContextBuilder` with `typed-builder`
- `crates/spiral-context/src/escalated.rs` — IPC glue, `Escalated::connect`
- `crates/spiral-context/tests/compile_fail.rs` — `trybuild` compile-fail tests
- `crates/spiral-context/tests/brand_safety.rs` — runtime trait dispatch tests

### 9.2 Dependencies

```toml
[dependencies]
spiral-core = { workspace = true }
spiral-ipc = { workspace = true }
thiserror = { workspace = true }

[dev-dependencies]
trybuild = "1"
```

No `ambient-authority`, `generativity`, or `typed-builder` external
crates. The branded lifetime pattern and capability tokens are
implemented from scratch in Spiral-native code. The `Brand<'brand>`,
`Handle<'brand, T>`, and `Arena<'brand, T>` types are ~200 lines
of Rust that we own entirely.

### 9.3 In scope for M4

- Brand types (`Brand<'brand>`, `Origin`, `Handle<'brand, T>`, `Arena<'brand, T>`)
- Capability set struct and its empty constructor
- Four capability token types (`FsCap`, `NetCap`, `ClockCap`, `RngCap`)
- `DomCap<'brand>` and a minimal `Dom<'brand>` (allocate, get, no DOM operations)
- `Context<'brand, Mode>` with `InProcess` and `Escalated` markers
- `ContextOps` trait with `run_script` and `dom`
- `ContextBuilder` with `typed-builder`
- Compile-fail tests via `trybuild`
- Runtime trait dispatch tests

### 9.4 Out of scope for M4

- Actual `spiral-net` / `spiral-storage` implementations
- OS-level sandbox integration (already partly in `spiral-sandbox`)
- Vortex isolate integration (needs `spiral-vortex` heap rewrite first)
- IPC-based escalated context (needs `spiral-ipc` protocol update)

---

## 10. Test Plan

1. `trybuild` compile-fail test: `Brand::<'static>::new()` should fail
   to compile (the brand cannot be forged).
2. Compile-fail test: cross-origin `Handle` passed to wrong `Dom` fails
   to compile.
3. Runtime test: empty capability set compiles; calling `fs_open` on it
   fails at runtime.
4. Runtime test: `InProcess` and `Escalated` contexts do not satisfy the
   same generic bound without `ContextOps`.
5. Fuzz test: adversarial brand construction attempts are rejected.

---

## 11. Sources

- [GhostCell paper (ICFP 2021)](https://plv.mpi-sws.org/rustbelt/ghostcell/)
- [Rustonomicon — PhantomData](https://doc.rust-lang.org/nomicon/phantomdata.html)
- [`cap-std` docs](https://docs.rs/cap-std/) — filesystem capability model
- [`ambient-authority` crate](https://github.com/sunfishcode/ambient-authority)
- [`generativity` crate](https://docs.rs/generativity/) — minimal brand generator
- [`qcell` crate](https://docs.rs/qcell/) — `LCell`, `QCell` branded-cell variants
- [WASI preopens](https://docs.rs/wasi/latest/wasi/filesystem/preopens/)
- [FreeBSD Capsicum](https://en.wikipedia.org/wiki/Capsicum_(Unix))
- [seL4 capabilities tutorial](https://docs.sel4.systems/Tutorials/capabilities.html)
- [`typed-builder` crate](https://docs.rs/typed-builder/)

---

## 12. Network filter hook (Packet 1.6.4 / ADR 0005)

The `FilterHook` is the capability-typed boundary
between the network stack and the policy engine.
A `FilterHook` is **not** a capability in the
type-system sense above (it is a free-standing
trait, not a `CapabilitySet` member), but it
follows the same "explicit grant, no implicit
authority" principle: the network stack
explicitly takes a `&impl FilterHook` (or a
generic bound) on `Client::request`, and the
caller must pass one. There is no global default
"allow" hook in production — only the test
suite's "always allow" mock.

The 1.6.4 packet + ADR 0005 (2026-06-16) moved
`FilterHook` + `Decision` + `Party` from
`spiral-filter` to `spiral-core` so the network
crate (`spiral-network`) could depend on them
without depending on the whole
HTML/CSS/rule-DSL dep graph. The current state:

- `spiral_core::FilterHook` (the trait),
  `spiral_core::Decision` (the `Allow` / `Block`
  enum), and `spiral_core::Party` (`First` /
  `Third`) are the canonical definitions.
- `spiral-filter::lib.rs` re-exports them for
  backwards compatibility:
  `pub use spiral_core::{Decision, FilterHook, Party};`
- `spiral-network` depends on `spiral-core`
  (not `spiral-filter`); the integration test
  that needs the full `Filter` impl is in
  `spiral-filter` (the only consumer outside
  the symbol's home crate that needs the
  generic `Filter` struct, not the trait).

The trait signature:

```rust
pub trait FilterHook {
    fn decide(&self, ctx: &FilterContext) -> Decision;
}

pub enum Decision { Allow, Block { reason: String } }
pub enum Party { First, Third }
```

`FilterContext` carries the request URL, the
first-party origin, and a few flags
(`is_third_party`, `resource_type`). The
default `impl FilterHook for Filter`
(`spiral-filter/src/runtime/mod.rs`) wraps the
`CompiledFilter` + `PolicyLevel` pair, calls
`match_url::extract_host` on the URL, and
returns `Decision::Allow` or `Decision::Block`
based on the hostname-trie lookup.

See
[`docs/decisions/0005-filter-hook-architecture.md`](../../decisions/0005-filter-hook-architecture.md)
for the architecture decision rationale,
[`docs/architecture/net.md`](../net.md) §"Filter
hook integration" for the call-site wiring,
and [`docs/architecture/filter.md`](../filter.md)
§"Fork 2 — process-global `FilterHook`" for the
runtime side.
