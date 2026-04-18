//! Sidebar section component.
//!
//! A labelled section with a bordered content box, used for Dependencies,
//! Dependents, Imports, Exports in the package sidebar.

use html::text_content::Division;
use html::text_content::builders::DivisionBuilder;

/// Render a sidebar section: h3 label + bordered content container.
///
/// Returns a `DivisionBuilder` so callers can push content into it.
pub(crate) fn render(title: &str) -> Division {
    Division::builder()
        .heading_3(|h3| {
            h3.class("text-[12px] font-mono uppercase tracking-wider text-ink-500 mb-2")
                .text(title.to_owned())
        })
        .build()
}

/// Render the bordered content box that goes inside a sidebar section.
pub(crate) fn content_box() -> &'static str {
    "border border-line p-3 space-y-1"
}

/// Render a package link inside a sidebar section (e.g. dependency item).
pub(crate) fn package_link(display_name: &str, href: &str) -> Division {
    Division::builder()
        .class("text-[13px]")
        .anchor(|a| {
            a.href(href.to_owned())
                .class("text-accent hover:underline font-mono")
                .text(display_name.to_owned())
        })
        .build()
}
