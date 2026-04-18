//! Icon constants.
//!
//! Stroke icons at 1.75 weight per design system. All icons use
//! `currentColor` so they inherit the parent's text color.

/// Icon identity.
pub(crate) enum Icon {
    /// Clipboard copy icon.
    Copy,
    /// Checkmark (copy-success feedback).
    Check,
    /// Breadcrumb chevron separator.
    ChevronRight,
    /// Search magnifier.
    Search,
}

/// Icon display size.
pub(crate) enum IconSize {
    /// 12×12 — small chevrons, inline markers.
    Sm,
    /// 14×14 — inside buttons, inline actions.
    Md,
    /// 16×16 — toolbar icons, standalone actions.
    Lg,
}

/// Return the raw SVG string for the given icon at the given size.
pub(crate) fn svg(icon: Icon, size: IconSize) -> &'static str {
    let w = match size {
        IconSize::Sm => 12,
        IconSize::Md => 14,
        IconSize::Lg => 16,
    };
    match (icon, w) {
        (Icon::Copy, 14) => {
            "<svg width='14' height='14' viewBox='0 0 24 24' fill='none' stroke='currentColor' stroke-width='1.75' stroke-linecap='round' stroke-linejoin='round'><rect x='9' y='9' width='13' height='13' rx='2' ry='2'/><path d='M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1'/></svg>"
        }
        (Icon::Copy, 16) => {
            "<svg width='16' height='16' viewBox='0 0 24 24' fill='none' stroke='currentColor' stroke-width='1.75' stroke-linecap='round' stroke-linejoin='round'><rect x='9' y='9' width='13' height='13' rx='2' ry='2'/><path d='M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1'/></svg>"
        }
        (Icon::Check, 14) => {
            "<svg width='14' height='14' viewBox='0 0 24 24' fill='none' stroke='currentColor' stroke-width='1.75' stroke-linecap='round' stroke-linejoin='round'><polyline points='20 6 9 17 4 12'/></svg>"
        }
        (Icon::Check, 16) => {
            "<svg width='16' height='16' viewBox='0 0 24 24' fill='none' stroke='currentColor' stroke-width='1.75' stroke-linecap='round' stroke-linejoin='round'><polyline points='20 6 9 17 4 12'/></svg>"
        }
        (Icon::ChevronRight, 12) => {
            "<svg width='12' height='12' viewBox='0 0 24 24' fill='none' stroke='currentColor' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'><path d='m9 18 6-6-6-6'/></svg>"
        }
        (Icon::Search, 14) => {
            "<svg width='14' height='14' viewBox='0 0 24 24' fill='none' stroke='currentColor' stroke-width='1.75' stroke-linecap='round' stroke-linejoin='round'><circle cx='11' cy='11' r='8'/><path d='m21 21-4.3-4.3'/></svg>"
        }
        // Fallback: return the most common size for each icon
        (Icon::Copy, _) => svg(Icon::Copy, IconSize::Md),
        (Icon::Check, _) => svg(Icon::Check, IconSize::Md),
        (Icon::ChevronRight, _) => svg(Icon::ChevronRight, IconSize::Sm),
        (Icon::Search, _) => svg(Icon::Search, IconSize::Md),
    }
}
