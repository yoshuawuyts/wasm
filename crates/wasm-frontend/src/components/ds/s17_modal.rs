//! 17 — Modal.

use html::text_content::Division;

const SVG_CLOSE: &str = concat!(
    r#"<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round">"#,
    include_str!("../../../../../vendor/lucide/x.svg"),
    "</svg>"
);

/// Render this section.
pub(crate) fn render() -> String {
    let content = Division::builder()
        .class("relative rounded-lg p-8 md:p-12 overflow-hidden bg-canvas")
        // Page skeleton beneath scrim
        .division(|skel| {
            skel.class("absolute inset-0 p-6 select-none pointer-events-none")
                .aria_hidden(true)
                .division(|d| d.class("h-3 w-40 rounded mb-3").style("background: var(--c-ink-300);"))
                .division(|d| d.class("h-2 w-72 rounded mb-2").style("background: var(--c-line);"))
                .division(|d| d.class("h-2 w-64 rounded mb-2").style("background: var(--c-line);"))
                .division(|d| d.class("h-2 w-56 rounded").style("background: var(--c-line);"))
        })
        // Scrim
        .division(|scrim| {
            scrim.class("absolute inset-0")
                .style("background: rgba(15, 15, 17, 0.55); backdrop-filter: blur(2px);")
        })
        // Dialog
        .division(|dialog| {
            dialog.class("relative mx-auto max-w-md bg-surface border border-line rounded-lg shadow-tooltip")
                // Header
                .division(|hdr| {
                    hdr.class("flex items-start justify-between p-5 border-b border-lineSoft")
                        .division(|t| {
                            t.division(|n| n.class("text-[15px] font-semibold tracking-tight").text("Confirm action"))
                                .division(|s| s.class("text-[12px] text-ink-500 mt-1").text("Lorem ipsum dolor sit amet"))
                        })
                        .button(|b| b.class("text-ink-500 hover:text-ink-900").text(SVG_CLOSE))
                })
                // Body
                .division(|body| {
                    body.class("p-5 text-[14px] text-ink-700 leading-relaxed")
                        .text("Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. Vestibulum tortor quam.")
                })
                // Footer
                .division(|footer| {
                    footer.class("flex items-center justify-end gap-2 p-4 border-t border-lineSoft bg-canvas rounded-b-lg")
                        .button(|b| b.class("h-8 px-3 rounded-lg border border-line bg-surface text-[13px] hover:bg-surfaceMuted").text("Cancel"))
                        .button(|b| b.class("h-8 px-3 rounded-lg bg-surfaceMuted text-ink-900 text-[13px] hover:bg-ink-300").text("Confirm"))
                })
        })
        .build()
        .to_string();

    super::section(
        "modal",
        "17",
        "Modal",
        "Centered dialog over a 50% ink scrim. 8px radius, 1px gray border, 24px padding. Header / body / footer rhythm.",
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
