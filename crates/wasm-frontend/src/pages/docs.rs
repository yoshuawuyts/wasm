//! Documentation page.

use html::text_content::Division;

use crate::layout;

/// Render the documentation page.
#[must_use]
pub(crate) fn render() -> String {
    let body = Division::builder()
        .class("pt-8 max-w-[65ch]")
        .heading_1(|h1| {
            h1.class("text-3xl font-normal tracking-display mb-6")
                .text("Documentation")
        })
        .paragraph(|p| {
            p.class("text-fg-secondary leading-relaxed")
                .text("Documentation is coming soon.")
        })
        .build();

    layout::document("Docs", &body.to_string())
}
