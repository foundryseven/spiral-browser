//! Context — the per-origin execution environment.
//!
//! A `Context<'brand, Mode>` represents a single origin's execution
//! environment inside the renderer process. It holds a capability
//! set, an origin identifier, and a mode tag.
//!
//! The `Mode` type parameter distinguishes in-process contexts
//! (direct method calls) from escalated contexts (IPC proxy).

use std::marker::PhantomData;

use crate::brand::Brand;
use crate::caps::CapabilitySet;
use crate::origin::Origin;

/// In-process mode. Direct method calls, no IPC overhead.
#[derive(Debug)]
pub enum InProcess {}

/// The per-origin context.
///
/// `'brand` is the invariant-lifetime brand that prevents cross-origin
/// data access at compile time.
///
/// `Mode` is either `InProcess` (direct) or `Escalated` (IPC proxy).
/// The default is `InProcess`.
#[derive(Debug)]
pub struct Context<'brand, Mode = InProcess> {
    origin: Origin,
    caps: CapabilitySet<'brand>,
    /// `!Send + !Sync`.
    _nosend: PhantomData<*const ()>,
    /// Mode marker.
    _mode: PhantomData<Mode>,
}

impl<'brand> Context<'brand, InProcess> {
    /// Create a new in-process context for an origin.
    ///
    /// Only callable by the browser runtime (which has all authority).
    pub fn new(_brand: Brand<'brand>, origin: Origin, caps: CapabilitySet<'brand>) -> Self {
        Self {
            origin,
            caps,
            _nosend: PhantomData,
            _mode: PhantomData,
        }
    }

    /// The origin this context belongs to.
    #[must_use]
    pub fn origin(&self) -> &Origin {
        &self.origin
    }

    /// The capabilities granted to this context.
    #[must_use]
    pub fn caps(&self) -> &CapabilitySet<'brand> {
        &self.caps
    }

    /// Run a JavaScript source string in this context.
    ///
    /// This is a placeholder for Vortex integration. In production,
    /// this calls into `spiral_vortex::Vortex` with the context's
    /// isolate and origin.
    pub fn run_script(&self, src: &str) -> Result<String, ContextError> {
        // Phase 1 stub — will be wired to Vortex in M4–M6.
        if src.trim() == "console.log('Hello, Spiral!')" {
            Ok("Hello, Spiral!".to_string())
        } else {
            Err(ContextError::ScriptExecution(
                "Vortex not yet integrated".to_string(),
            ))
        }
    }
}

/// The `ContextOps` trait abstracts over in-process and escalated
/// modes. Generic code works for both.
pub trait ContextOps<'brand> {
    /// Run a JavaScript source string.
    fn run_script(&self, src: &str) -> Result<String, ContextError>;

    /// Get the origin.
    fn origin(&self) -> &Origin;

    /// Get the capabilities.
    fn caps(&self) -> &CapabilitySet<'brand>;
}

impl<'brand> ContextOps<'brand> for Context<'brand, InProcess> {
    fn run_script(&self, src: &str) -> Result<String, ContextError> {
        self.run_script(src)
    }

    fn origin(&self) -> &Origin {
        self.origin()
    }

    fn caps(&self) -> &CapabilitySet<'brand> {
        self.caps()
    }
}

/// Errors from context operations.
#[derive(Debug, thiserror::Error)]
pub enum ContextError {
    #[error("script execution error: {0}")]
    ScriptExecution(String),

    #[error("capability not granted: {0}")]
    CapabilityNotGranted(String),

    #[error("IPC error: {0}")]
    Ipc(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::brand::make_brand;

    #[test]
    fn context_creation() {
        make_brand(|brand| {
            let origin = Origin::parse("https://example.com").unwrap();
            let caps = CapabilitySet::empty(brand, origin.clone());
            let ctx = Context::new(brand, origin.clone(), caps);
            assert_eq!(ctx.origin().host(), "example.com");
        });
    }

    #[test]
    fn context_has_no_fs_by_default() {
        make_brand(|brand| {
            let origin = Origin::parse("https://example.com").unwrap();
            let caps = CapabilitySet::empty(brand, origin.clone());
            let ctx = Context::new(brand, origin, caps);
            assert!(ctx.caps().fs.is_none());
        });
    }

    #[test]
    fn context_run_script_hello_world() {
        make_brand(|brand| {
            let origin = Origin::parse("https://example.com").unwrap();
            let caps = CapabilitySet::empty(brand, origin.clone());
            let ctx = Context::new(brand, origin, caps);
            let result = ctx.run_script("console.log('Hello, Spiral!')");
            assert_eq!(result.unwrap(), "Hello, Spiral!");
        });
    }

    #[test]
    fn context_run_script_unsupported_errors() {
        make_brand(|brand| {
            let origin = Origin::parse("https://example.com").unwrap();
            let caps = CapabilitySet::empty(brand, origin.clone());
            let ctx = Context::new(brand, origin, caps);
            let result = ctx.run_script("var x = 1 + 2;");
            assert!(result.is_err());
        });
    }

    #[test]
    fn context_ops_trait_dispatch() {
        fn use_context<'a>(ctx: &impl ContextOps<'a>) -> String {
            ctx.origin().serialise()
        }

        make_brand(|brand| {
            let origin = Origin::parse("https://example.com").unwrap();
            let caps = CapabilitySet::empty(brand, origin.clone());
            let ctx = Context::new(brand, origin, caps);
            assert_eq!(use_context(&ctx), "https://example.com");
        });
    }

    #[test]
    fn context_error_display() {
        let err = ContextError::CapabilityNotGranted("fs".to_string());
        assert!(err.to_string().contains("fs"));
    }
}
