//! Downloads page.

use html::text_content::Division;

use crate::components::code_block;
use crate::layout;

/// Render the downloads page with install instructions.
#[must_use]
pub(crate) fn render() -> String {
    let body = Division::builder()
        .class("pt-8 max-w-[65ch]")
        .heading_1(|h1| {
            h1.class("text-[28px] font-semibold tracking-tight font-mono mb-6")
                .text("Downloads")
        })
        .paragraph(|p| {
            p.class("text-ink-700 leading-relaxed")
                .text("Install the wasm CLI to manage WebAssembly components from your terminal.")
        })
        .heading_2(|h2| {
            h2.class("text-[22px] font-semibold tracking-tight font-mono mt-10 mb-4")
                .text("Quick install")
        })
        .division(|d| {
            d.class("space-y-4")
                .division(|block| {
                    block
                        .paragraph(|p| {
                            p.class("text-ink-700 mb-2").text("macOS / Linux:")
                        })
                        .push(
                            html::text_content::PreformattedText::builder()
                                .class(code_block::CLASS)
                                .code(|c| {
                                    c.text("curl -fsSL https://raw.githubusercontent.com/yoshuawuyts/wasm-cli/main/scripts/install.sh | sh")
                                })
                                .build(),
                        )
                })
                .division(|block| {
                    block
                        .paragraph(|p| {
                            p.class("text-ink-700 mb-2").text("Windows (PowerShell):")
                        })
                        .push(
                            html::text_content::PreformattedText::builder()
                                .class(code_block::CLASS)
                                .code(|c| {
                                    c.text("irm https://raw.githubusercontent.com/yoshuawuyts/wasm-cli/main/scripts/install.ps1 | iex")
                                })
                                .build(),
                        )
                })
        })
        .heading_2(|h2| {
            h2.class("text-[22px] font-semibold tracking-tight font-mono mt-10 mb-4")
                .text("From source")
        })
        .push(
            html::text_content::PreformattedText::builder()
                .class(code_block::CLASS)
                .code(|c| c.text("cargo install wasm-cli"))
                .build(),
        )
        .build();

    layout::document_with_nav("Downloads", &body.to_string())
}
