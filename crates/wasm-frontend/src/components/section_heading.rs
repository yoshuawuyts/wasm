//! Section heading component.
//!
//! Muted `<h2>` headings used to label content sections.
//! Two variants: plain and bordered (with a bottom rule).

/// Class string for a plain section heading.
pub(crate) const CLASS: &str = "text-[16px] font-medium text-ink-500 mb-3";

/// Class string for a section heading with a bottom border.
pub(crate) const BORDERED_CLASS: &str =
    "text-[16px] font-medium text-ink-500 mb-3 pb-2 border-b border-lineSoft";
