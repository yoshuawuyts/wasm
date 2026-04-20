//! 16 — Dropdown.

use html::text_content::Division;

const SVG_EDIT: &str = concat!(
    r#"<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round">"#,
    include_str!("../../../../../vendor/lucide/square-pen.svg"),
    "</svg>"
);
const SVG_COPY: &str = concat!(
    r#"<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round">"#,
    include_str!("../../../../../vendor/lucide/copy.svg"),
    "</svg>"
);
const SVG_SHARE: &str = concat!(
    r#"<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round">"#,
    include_str!("../../../../../vendor/lucide/share.svg"),
    "</svg>"
);
const SVG_DELETE: &str = concat!(
    r#"<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round">"#,
    include_str!("../../../../../vendor/lucide/trash-2.svg"),
    "</svg>"
);

/// Render this section.
pub(crate) fn render() -> String {
    let content = Division::builder()
        .class("p-12 bg-canvas border border-line rounded-lg flex items-start justify-center")
        .division(|menu| {
            menu.class("w-56 rounded-md bg-surface border border-line shadow-tooltip py-1 text-[13px]")
                .division(|lbl| {
                    lbl.class("px-3 py-1.5 text-[11px] mono uppercase tracking-wider text-ink-400")
                        .text("Aenean")
                })
                .button(|b| {
                    b.class("w-full text-left px-3 h-8 hover:bg-surfaceMuted flex items-center gap-2")
                        .text(SVG_EDIT)
                        .text(" Edit lorem")
                })
                .button(|b| {
                    b.class("w-full text-left px-3 h-8 hover:bg-surfaceMuted flex items-center gap-2")
                        .text(SVG_COPY)
                        .text(" Duplicate")
                })
                .division(|sep| sep.class("my-1 border-t border-lineSoft"))
                .button(|b| {
                    b.class("w-full text-left px-3 h-8 hover:bg-surfaceMuted flex items-center gap-2")
                        .text(SVG_SHARE)
                        .text(" Share")
                        .span(|s| s.class("ml-auto text-[11px] mono text-ink-400").text("\u{2318}S"))
                })
                .division(|sep| sep.class("my-1 border-t border-lineSoft"))
                .button(|b| {
                    b.class("w-full text-left px-3 h-8 hover:bg-surfaceMuted flex items-center gap-2 text-negative")
                        .text(SVG_DELETE)
                        .text(" Delete")
                })
        })
        .build()
        .to_string();

    super::section(
        "dropdown",
        "16",
        "Dropdown",
        "Floating menu on white. 1px gray border + tooltip-grade shadow. Section dividers separate logical groups.",
        &content,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snapshot() {
        insta::assert_snapshot!(crate::components::ds::pretty_html(&render()));
    }
}
