//! Box model and geometry types for the Gyre layout engine.

/// Layout dimensions.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct LayoutDimensions {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

/// Box model.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct BoxModel {
    pub margin: EdgeSizes,
    pub border: EdgeSizes,
    pub padding: EdgeSizes,
    pub content: LayoutDimensions,
}

/// Edge sizes (margin, border, padding).
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct EdgeSizes {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl EdgeSizes {
    /// Create zero-sized edges.
    pub fn zero() -> Self {
        Self {
            top: 0.0,
            right: 0.0,
            bottom: 0.0,
            left: 0.0,
        }
    }
}
