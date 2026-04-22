//! Front page — landing experience matching `references/landing.html`.

// r[impl frontend.pages.home]

use wasm_meta_registry_client::{ApiError, KnownPackage, RegistryClient};

use crate::components::ds::{
    cta_strip::{self, CtaStrip},
    hero::{self, Hero, HeroCta, HeroCtaStyle},
    install_card::{self, InstallCard},
    link_list::{self, LeftStyle, LinkRow, RightStyle},
    metrics_strip::{self, Metric},
    principles_grid::{self, Principle},
    search_bar,
};
use crate::layout;

/// Fetch recent packages and render the home page.
pub(crate) async fn render(client: &RegistryClient) -> String {
    match client.fetch_recent_packages(50).await {
        Ok(packages) => render_packages(&packages),
        Err(err) => render_error(&err),
    }
}

/// Render the home page with a list of packages.
fn render_packages(packages: &[KnownPackage]) -> String {
    let body = compose_body(packages.len());
    layout::document_landing("Home", &body)
}

/// Render the home page with an API error message — keep the chrome but
/// fall back to a placeholder package count.
fn render_error(_err: &ApiError) -> String {
    let body = compose_body(0);
    layout::document_landing("Home", &body)
}

/// Compose the full landing page body.
fn compose_body(total_packages: usize) -> String {
    let install = install_card::render(&InstallCard {
        platforms: &["macOS", "Linux", "Windows"],
        filename: "install.sh",
        snippet_html: &install_snippet(),
        sha: "9e4a…c0f1",
    });

    let hero_html = hero::render(&Hero {
        kicker: &["v0.4.0", "Stable · WASI 0.2"],
        title: "The package manager for components.",
        lede: "Resolve, vendor, and compose WebAssembly components from any registry. \
               Reproducible builds, semantic versioning, and an append-only index — so \
               the dependency you shipped is the dependency you keep.",
        ctas: &[
            HeroCta {
                label: "Get started",
                href: "/docs",
                style: HeroCtaStyle::Primary,
            },
            HeroCta {
                label: "Browse packages",
                href: "/all",
                style: HeroCtaStyle::Secondary,
            },
            HeroCta {
                label: "Source on GitHub",
                href: "https://github.com/yoshuawuyts/component-cli",
                style: HeroCtaStyle::Ghost,
            },
        ],
        right: &install,
    });

    // TODO: replace placeholder counts with real registry stats.
    let display_count = total_packages.max(73);
    let pkg_count = format_count(display_count);
    let metrics_html = metrics_strip::render(&[
        Metric {
            label: "Packages",
            value: &pkg_count,
            delta: Some("+2 this week"),
            verified: false,
        },
        Metric {
            label: "Authors",
            value: "13",
            delta: Some("+1"),
            verified: false,
        },
        Metric {
            label: "Versions published",
            value: "124",
            delta: Some("+4 this week"),
            verified: false,
        },
        Metric {
            label: "Index integrity",
            value: "100%",
            delta: Some("verified"),
            verified: true,
        },
    ]);

    let explore_html = render_explore(display_count);

    let principles_html = principles_grid::render(
        "Why wasm",
        "Built for components.",
        "A package manager designed around the WebAssembly Component Model — not \
         retrofitted from an older ecosystem.",
        PRINCIPLES,
    );

    let cta_html = cta_strip::render(&CtaStrip {
        kicker: "For maintainers",
        title: "Publish your component.",
        body_html: "Add your namespace to a registry config and run \
                    <code class=\"px-1 py-0.5 rounded-sm bg-surfaceMuted text-ink-900 mono text-[0.875em]\">wasm publish</code>. \
                    The index is append-only and signed end-to-end.",
        primary_label: "Open the publishing guide",
        primary_href: "/docs",
        secondary_label: "Read the spec",
        secondary_href: "/docs",
    });

    format!(
        r#"{hero_html}
{metrics_html}
<div class="bg-surface pt-12 md:pt-16 pb-14 md:pb-20">
{explore_html}
{principles_html}
{cta_html}
</div>"#
    )
}

/// Render the "Explore the ecosystem" section: kicker, search input, and
/// two link lists (Featured / Categories).
fn render_explore(package_count: usize) -> String {
    let count_str = format_count(package_count);
    let search_html = search_bar::landing(&count_str).to_string();
    let featured = link_list::render(
        "Featured",
        FEATURED,
        &LeftStyle::Mono,
        &RightStyle::Description,
    );
    let categories = link_list::render(
        "Categories",
        CATEGORIES,
        &LeftStyle::Plain,
        &RightStyle::Count,
    );

    format!(
        r#"<section class="mx-auto max-w-[1280px] px-4 md:px-8">
  <div class="max-w-2xl">
    <div class="text-[12px] mono uppercase tracking-wider text-ink-500">Explore</div>
    <h2 class="mt-2 text-[28px] md:text-[32px] font-semibold tracking-tight">The ecosystem.</h2>
    <p class="mt-3 text-[14px] text-ink-700 leading-relaxed">
      <span class="mono tabular-nums text-ink-900">{count_str}</span> packages from
      <span class="mono tabular-nums text-ink-900">13</span> authors across
      <span class="mono tabular-nums text-ink-900">8</span> categories.
    </p>
  </div>

  {search_html}

  <div class="mt-10 grid md:grid-cols-2 gap-x-12 gap-y-10">
    {featured}
    {categories}
  </div>

  <div class="mt-6 text-right text-[13px]">
    <a href="/all" class="text-ink-900 hover:underline no-underline">Browse all packages →</a>
  </div>
</section>"#
    )
}

/// Format a count with a thin space as the thousands separator (e.g.
/// `1248 -> "1 248"`), matching the visual style in `landing.html`.
fn format_count(n: usize) -> String {
    let s = n.to_string();
    let mut out = String::with_capacity(s.len() + s.len() / 3);
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            out.push('\u{2009}');
        }
        out.push(c);
    }
    out.chars().rev().collect()
}

/// Build the install card shell snippet using the helper spans.
fn install_snippet() -> String {
    let mut s = String::new();
    s.push_str(&install_card::prompt(
        "curl -sSf https://wasm.dev/install.sh | sh",
    ));
    s.push_str("\n\n");
    s.push_str(&install_card::prompt(
        "wasm <span class=\"font-semibold\">init</span> hello-world",
    ));
    s.push('\n');
    s.push_str(&install_card::muted("  Created hello-world/wasm.toml"));
    s.push_str("\n\n");
    s.push_str(&install_card::prompt(
        "wasm <span class=\"font-semibold\">add</span> wasi:http@0.2",
    ));
    s.push('\n');
    s.push_str(&install_card::muted("  Resolved 4 packages in 312 ms"));
    s.push('\n');
    s.push_str(&install_card::positive("  ✓ Locked"));
    s.push_str(" wasi:http ");
    s.push_str(&install_card::muted("0.2.3"));
    s.push('\n');
    s.push_str(&install_card::positive("  ✓ Locked"));
    s.push_str(" wasi:io   ");
    s.push_str(&install_card::muted("0.2.3"));
    s.push_str("\n\n");
    s.push_str(&install_card::prompt(
        "wasm <span class=\"font-semibold\">build</span>",
    ));
    s.push('\n');
    s.push_str(&install_card::positive("  ✓ Compiled"));
    s.push_str(" hello-world ");
    s.push_str(&install_card::muted("v0.1.0"));
    s.push(' ');
    s.push_str("<span class=\"text-ink-400\">(2.18s)</span>");
    s
}

/// Curated featured packages with hand-picked descriptions.
const FEATURED: &[LinkRow<'static>] = &[
    LinkRow {
        left: "wasi:http",
        right: "WASI standard for HTTP",
        href: "/wasi/http",
    },
    LinkRow {
        left: "wasi:cli",
        right: "Command-line entry points",
        href: "/wasi/cli",
    },
    LinkRow {
        left: "wasi:io",
        right: "Streams and pollables",
        href: "/wasi/io",
    },
    LinkRow {
        left: "wasi:clocks",
        right: "Wall-clock and monotonic time",
        href: "/wasi/clocks",
    },
    LinkRow {
        left: "wasi:logging",
        right: "Structured logging interface",
        href: "/wasi/logging",
    },
];

/// Hard-coded category list — taxonomy work is a follow-up.
// TODO: replace with a real taxonomy backed by registry metadata.
const CATEGORIES: &[LinkRow<'static>] = &[
    LinkRow {
        left: "HTTP & networking",
        right: "1 248",
        href: "/all",
    },
    LinkRow {
        left: "CLI & shell",
        right: "906",
        href: "/all",
    },
    LinkRow {
        left: "Storage",
        right: "512",
        href: "/all",
    },
    LinkRow {
        left: "Parsers",
        right: "464",
        href: "/all",
    },
    LinkRow {
        left: "Cryptography",
        right: "388",
        href: "/all",
    },
];

const SHIELD_SVG: &str = r#"<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/></svg>"#;
const STACK_SVG: &str = concat!(
    r#"<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round">"#,
    include_str!("../../../../vendor/lucide/layers.svg"),
    "</svg>",
);
const GLOBE_SVG: &str = r#"<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="9"/><path d="M3 12h18"/><path d="M12 3a14 14 0 0 1 0 18a14 14 0 0 1 0-18z"/></svg>"#;
const GRID_SVG: &str = r#"<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="3" width="7" height="7" rx="1"/><rect x="14" y="3" width="7" height="7" rx="1"/><rect x="3" y="14" width="7" height="7" rx="1"/><rect x="14" y="14" width="7" height="7" rx="1"/></svg>"#;

const PRINCIPLES: &[Principle<'static>] = &[
    Principle {
        bg_class: "bg-cat-blue",
        fg_class: "text-cat-blueInk",
        icon_svg: SHIELD_SVG,
        title: "Reproducible by default",
        body: "Every dependency is locked by content hash. The build you ship today \
               is the build you can ship in five years.",
    },
    Principle {
        bg_class: "bg-cat-green",
        fg_class: "text-cat-greenInk",
        icon_svg: STACK_SVG,
        title: "Federated registries",
        body: "Pull from any OCI-compatible registry — host your own, mirror upstream, \
               or compose private and public packages in one manifest.",
    },
    Principle {
        bg_class: "bg-cat-peach",
        fg_class: "text-cat-peachInk",
        icon_svg: GLOBE_SVG,
        title: "Semantic versioning",
        body: "Versions are strict semver — breaking changes require a major bump, \
               and once published, a version cannot be overwritten, only yanked.",
    },
    Principle {
        bg_class: "bg-cat-lilac",
        fg_class: "text-cat-lilacInk",
        icon_svg: GRID_SVG,
        title: "Compose, don't link",
        body: "Components are wired together at the WIT interface boundary — \
               no shared globals, no symbol clashes, no surprise side effects.",
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    // r[verify frontend.pages.home]
    #[test]
    fn format_count_inserts_thin_spaces() {
        assert_eq!(format_count(73), "73");
        assert_eq!(format_count(1248), "1\u{2009}248");
        assert_eq!(format_count(1_234_567), "1\u{2009}234\u{2009}567");
    }

    #[test]
    fn install_snippet_contains_expected_commands() {
        let snippet = install_snippet();
        assert!(snippet.contains("install.sh"));
        assert!(snippet.contains("wasi:http"));
    }
}
