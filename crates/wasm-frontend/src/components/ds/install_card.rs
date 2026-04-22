//! Install card — surface card with platform tabs, a shell snippet, and
//! a footer row showing a SHA + copy button.
//!
//! For v1 the platform tabs are rendered statically (macOS active); the
//! full multi-platform switch is a follow-up.

use html::text_content::Division;

/// Configuration for [`render`].
pub(crate) struct InstallCard<'a> {
    /// Platform labels in tab order; the first entry is rendered as active.
    pub platforms: &'a [&'a str],
    /// File-name shown on the right of the tab strip (e.g. `"install.sh"`).
    pub filename: &'a str,
    /// Pre-formatted HTML to drop into the `<pre>` body (already escaped /
    /// styled with the design system's code spans).
    pub snippet_html: &'a str,
    /// SHA hint shown at the bottom-left.
    pub sha: &'a str,
}

const COPY_ICON: &str = concat!(
    r#"<svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round">"#,
    include_str!("../../../../../vendor/lucide/copy.svg"),
    "</svg>"
);

/// Render the install card.
#[must_use]
pub(crate) fn render(card: &InstallCard<'_>) -> String {
    let filename = card.filename.to_owned();
    let snippet = card.snippet_html.to_owned();
    let sha = card.sha.to_owned();
    let platforms: Vec<String> = card.platforms.iter().map(|s| (*s).to_owned()).collect();

    Division::builder()
        .class("rounded-lg border border-line bg-surface shadow-card overflow-hidden")
        .division(|tabs| {
            let mut tabs = tabs.class("flex items-center gap-1 px-3 h-9 border-b border-lineSoft");
            for (i, label) in platforms.iter().enumerate() {
                let label = label.clone();
                let class = if i == 0 {
                    "h-7 px-2.5 rounded-md text-[12px] mono text-ink-900 bg-surfaceMuted"
                } else {
                    "h-7 px-2.5 rounded-md text-[12px] mono text-ink-500 hover:bg-surfaceMuted"
                };
                tabs = tabs.button(|b| b.type_("button").class(class).text(label));
            }
            tabs.span(|s| s.class("ml-auto text-[11px] text-ink-500 mono").text(filename))
        })
        .division(|body| {
            body.class("p-4").preformatted_text(|pre| {
                pre.class(
                    "mono text-[13px] leading-relaxed text-ink-900 whitespace-pre overflow-x-auto",
                )
                .text(snippet)
            })
        })
        .division(|foot| {
            foot.class("flex items-center justify-between px-3 h-9 border-t border-lineSoft text-[12px] text-ink-500")
                .span(|s| s.class("mono").text(format!("SHA256: {sha}")))
                .button(|b| {
                    b.type_("button")
                        .class("inline-flex items-center gap-1.5 hover:text-ink-900")
                        .text(COPY_ICON)
                        .text(" Copy")
                })
        })
        .build()
        .to_string()
}

/// Convenience helper to wrap a shell prompt with the design-system code
/// styling used in the install card snippet.
#[must_use]
pub(crate) fn prompt(rest: &str) -> String {
    format!(r#"<span class="text-ink-400">$</span> {rest}"#)
}

/// Convenience helper for a muted-info line in the install card snippet.
#[must_use]
pub(crate) fn muted(text: &str) -> String {
    format!(r#"<span class="text-ink-500">{text}</span>"#)
}

/// Convenience helper for a positive (success) line in the install card snippet.
#[must_use]
pub(crate) fn positive(text: &str) -> String {
    format!(r#"<span class="text-positive">{text}</span>"#)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snapshot() {
        let snippet = format!(
            "{}\n{}\n{}\n{}",
            muted("# install the wasm CLI"),
            prompt("curl -sSf https://get.wasm.run | sh"),
            muted("# verify"),
            positive("\u{2713} wasm 0.4.0 installed"),
        );
        let html = render(&InstallCard {
            platforms: &["macOS", "Linux", "Windows"],
            filename: "install.sh",
            snippet_html: &snippet,
            sha: "9f3c\u{2026}a217",
        });
        insta::assert_snapshot!(crate::components::ds::pretty_html(&html));
    }
}
