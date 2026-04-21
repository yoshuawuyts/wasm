//! Reusable UI components encoding the design system.
//!
//! Each submodule provides builder functions that return `html` crate types
//! with design-system Tailwind classes baked in. Pages call these instead of
//! writing raw class strings.

pub(crate) mod ds;
