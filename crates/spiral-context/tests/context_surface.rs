//! Integration tests for the `spiral-context` public surface.

use spiral_context::{
    Brand, CapabilitySet, ClockCap, Context, ContextOps, DomCap, FsCap, InProcess, NetCap, Origin,
    RngCap,
};

#[test]
fn test_context_surface_wiring() {
    let _brand: Option<Brand> = None;
    let _caps: Option<CapabilitySet> = None;
    let _clock: Option<ClockCap> = None;
    let _dom: Option<DomCap> = None;
    let _fs: Option<FsCap> = None;
    let _net: Option<NetCap> = None;
    let _rng: Option<RngCap> = None;
    let _in_process: Option<InProcess> = None;
    let _origin: Option<Origin> = None;
    let _ctx: Option<Context<InProcess>> = None;
    let _ops: Option<&dyn ContextOps<'static>> = None;
}
