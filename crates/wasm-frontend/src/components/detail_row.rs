//! Detail row component.
//!
//! Key-value metadata rows for sidebars and inspector panels.
//! Matches design system section 23 Details (Inline variant).

use html::text_content::Division;

/// Value type for a detail row.
pub(crate) enum Value {
    /// Plain text value.
    Text(String),
    /// Linked value.
    Link { text: String, href: String },
}

/// Render a key-value detail row.
pub(crate) fn row(label: &str, value: Value) -> Division {
    let mut d = Division::builder();
    d.class("flex items-baseline justify-between gap-4 py-1.5");
    d.span(|s| {
        s.class("text-ink-500 text-[13px] shrink-0")
            .text(label.to_owned())
    });
    match value {
        Value::Text(text) => {
            d.span(|s| {
                s.class("text-ink-900 text-[13px] font-mono text-right")
                    .text(text)
            });
        }
        Value::Link { text, href } => {
            d.anchor(|a| {
                a.href(href)
                    .class("text-accent hover:underline font-mono text-[13px] text-right truncate")
                    .text(text)
            });
        }
    }
    d.build()
}
