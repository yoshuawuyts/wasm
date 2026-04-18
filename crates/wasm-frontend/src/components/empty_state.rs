//! Empty state component.
//!
//! Centered illustration, title, body text, and optional CTA.
//! Used for empty tables, search misses, 404, and first-run views.

use html::text_content::Division;

/// Render an empty state with icon, title, description, and optional action.
pub(crate) fn render(title: &str, description: &str) -> Division {
    Division::builder()
        .class("border border-line rounded-lg p-12 text-center bg-surface")
        .division(|icon| {
            icon.class(
                "mx-auto h-12 w-12 grid place-items-center rounded-full bg-surfaceMuted text-ink-500",
            )
        })
        .division(|d| {
            d.class("mt-4 text-[16px] font-semibold tracking-tight")
                .text(title.to_owned())
        })
        .paragraph(|p| {
            p.class("mt-1 text-[13px] text-ink-500 max-w-xs mx-auto")
                .text(description.to_owned())
        })
        .build()
}
