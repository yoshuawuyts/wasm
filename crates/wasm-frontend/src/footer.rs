//! Footer component.

use html::content::Footer;

/// Render the site footer.
#[must_use]
pub(crate) fn render() -> String {
    Footer::builder()
        .class("border-t border-border mt-16")
        .division(|div| {
            div.class("max-w-5xl mx-auto px-4 py-6 text-center text-sm text-fg-muted")
                .paragraph(|p| {
                    p.text("wasm registry — ")
                        .anchor(|a| {
                            a.href("https://github.com/yoshuawuyts/wasm-cli")
                                .class("text-accent hover:underline transition-colors")
                                .text("open-source")
                        })
                        .text(" on GitHub")
                })
        })
        .build()
        .to_string()
}
