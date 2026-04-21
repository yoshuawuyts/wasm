//! Package row component.
//!
//! List-style row for search results and all-packages pages. Shows name,
//! version, and description in a responsive flex layout.

use html::inline_text::Span;
use html::text_content::Division;
use wasm_meta_registry_client::KnownPackage;

/// Render a package as a list row (name · version · description).
pub(crate) fn render(pkg: &KnownPackage) -> Division {
    let (display_name, href) = identity(pkg);
    let description = pkg.description.as_deref().unwrap_or("");
    let version = pkg.tags.first().map_or("\u{2014}", String::as_str);
    let name_color = if href.is_some() {
        "text-accent"
    } else {
        "text-ink-900"
    };

    let [name_span, version_span, description_span] =
        spans(&display_name, version, description, name_color);

    if let Some(href) = href {
        let mut row = Division::builder();
        row.anchor(|a| {
            a.href(href)
                .class("flex flex-wrap sm:flex-nowrap items-baseline gap-x-3 gap-y-1 py-3 hover:bg-surfaceMuted -mx-2 px-2 transition-colors")
                .push(name_span)
                .push(version_span)
                .push(description_span)
        });
        row.build()
    } else {
        let mut row = Division::builder();
        row.class("flex flex-wrap sm:flex-nowrap items-baseline gap-x-3 gap-y-1 py-3 -mx-2 px-2")
            .push(name_span)
            .push(version_span)
            .push(description_span);
        row.build()
    }
}

/// Extract display name and optional href from a package.
fn identity(pkg: &KnownPackage) -> (String, Option<String>) {
    match (&pkg.wit_namespace, &pkg.wit_name) {
        (Some(ns), Some(name)) => (format!("{ns}:{name}"), Some(format!("/{ns}/{name}"))),
        _ => (pkg.repository.clone(), None),
    }
}

/// Build the three column spans for a package row.
fn spans(
    display_name: &str,
    version: &str,
    description: &str,
    name_color_class: &str,
) -> [Span; 3] {
    [
        Span::builder()
            .class(format!(
                "sm:w-48 sm:shrink-0 font-medium {name_color_class} truncate"
            ))
            .text(display_name.to_owned())
            .build(),
        Span::builder()
            .class("text-[12px] sm:text-[13px] text-ink-400 sm:w-20 sm:shrink-0")
            .text(version.to_owned())
            .build(),
        Span::builder()
            .class("text-[13px] text-ink-500 truncate")
            .text(crate::markdown::render_inline(description))
            .build(),
    ]
}

/// Class string for the table header row above package rows.
pub(crate) const HEADER_CLASS: &str =
    "hidden sm:flex items-baseline gap-3 px-2 pb-2 text-[13px] text-ink-400";
