//! Front page — recently updated components and interfaces.

// r[impl frontend.pages.home]

use html::text_content::Division;
use html::text_content::builders::DivisionBuilder;
use wasm_meta_registry_client::KnownPackage;

use crate::components::ds::package_card;
use crate::components::ds::search_bar::{self, SearchBar};
use crate::layout;
use wasm_meta_registry_client::{ApiError, RegistryClient};

/// Maximum number of packages to show per tab on the home page (4 cols × 10 rows).
const HOME_SECTION_LIMIT: usize = 40;

/// Fetch recent packages and render the home page.
pub(crate) async fn render(client: &RegistryClient) -> String {
    match client.fetch_recent_packages(50).await {
        Ok(packages) => render_packages(&packages),
        Err(err) => render_error(&err),
    }
}

/// Packages pinned to the top of the home page, in display order.
const PINNED_PACKAGES: &[(&str, &str)] = &[
    ("ba", "sample-wasi-http-rust"),
    ("wasi", "http"),
    ("wasi", "cli"),
    ("wasi", "io"),
    ("wasi", "clocks"),
    ("wasi", "logging"),
];

/// Render the home page with a list of packages.
fn render_packages(packages: &[KnownPackage]) -> String {
    let ordered = pin_and_sort(packages);
    let (components, interfaces) = split_by_kind(&ordered);

    let mut body = Division::builder();

    // Hero area
    body.push(render_hero(ordered.len()));

    // Tabbed package listing
    body.push(render_tabs(&ordered, &interfaces, &components));

    layout::document("Home", &body.build().to_string())
}

/// Re-order packages so pinned entries appear first (in `PINNED_PACKAGES`
/// order), followed by the remaining packages sorted most-recently-published
/// first.
fn pin_and_sort(packages: &[KnownPackage]) -> Vec<KnownPackage> {
    let mut pinned: Vec<KnownPackage> = Vec::with_capacity(PINNED_PACKAGES.len());
    let mut rest: Vec<KnownPackage> = Vec::new();

    // Collect pinned packages in their declared order.
    for &(ns, name) in PINNED_PACKAGES {
        if let Some(pkg) = packages
            .iter()
            .find(|p| p.wit_namespace.as_deref() == Some(ns) && p.wit_name.as_deref() == Some(name))
        {
            pinned.push(pkg.clone());
        }
    }

    // Collect everything else, preserving the existing most-recent-first order.
    for pkg in packages {
        let is_pinned = PINNED_PACKAGES.iter().any(|&(ns, name)| {
            pkg.wit_namespace.as_deref() == Some(ns) && pkg.wit_name.as_deref() == Some(name)
        });
        if !is_pinned {
            rest.push(pkg.clone());
        }
    }

    pinned.extend(rest);
    pinned
}

/// Render the home page with an API error message.
fn render_error(_err: &ApiError) -> String {
    let mut body = Division::builder();
    body.push(render_hero(0));
    body.division(|div| {
        div.class("py-16 text-center")
            .paragraph(|p| {
                p.class("text-ink-900 font-medium")
                    .text("Could not load components")
            })
            .paragraph(|p| {
                p.class("text-[13px] text-ink-500 mt-2")
                    .text("The registry may be temporarily unavailable. Try refreshing the page.")
            })
    });
    layout::document("Home", &body.build().to_string())
}

/// Render the hero area with heading, nav, search, and CTA.
fn render_hero(_total: usize) -> Division {
    let mut hero = Division::builder();
    hero.class("pt-8 sm:pt-16 pb-8 sm:pb-12");

    hero.division(|row| {
        row.class("flex flex-wrap items-start justify-between gap-4")
            .heading_1(|h1| {
                h1.class("text-[24px] sm:text-[36px] font-semibold tracking-tight leading-[1.1]")
                    .text("WebAssembly Component Registry")
            })
            .division(|nav| {
                nav.class("flex gap-5 text-[13px]")
                    .anchor(|a| {
                        a.href("/docs")
                            .class("text-ink-500 hover:text-ink-900 transition-colors")
                            .text("Docs")
                    })
                    .anchor(|a| {
                        a.href("/downloads")
                            .class("text-ink-500 hover:text-ink-900 transition-colors")
                            .text("Downloads")
                    })
            })
    });

    hero.division(|row| {
        row.class("mt-6 sm:mt-10 flex flex-col sm:flex-row gap-4 sm:gap-6 sm:items-center")
            .push(search_bar::hero(&SearchBar {
                carousel: true,
                ..SearchBar::default()
            }))
            .anchor(|a| {
                a.href("/docs")
                    .class("group text-[13px] text-ink-500 hover:text-ink-900 transition-colors shrink-0")
                    .span(|s| s.text("Publish a component ".to_owned()))
                    .span(|s| {
                        s.class("inline-block transition-transform group-hover:translate-x-1")
                            .text("\u{2192}")
                    })
            })
    });

    hero.build()
}

/// Split packages into (components, interfaces) based on package kind.
fn split_by_kind(packages: &[KnownPackage]) -> (Vec<&KnownPackage>, Vec<&KnownPackage>) {
    let mut components = Vec::new();
    let mut interfaces = Vec::new();

    for pkg in packages {
        match pkg.kind {
            Some(wasm_meta_registry_client::PackageKind::Interface) => interfaces.push(pkg),
            _ => components.push(pkg),
        }
    }

    (components, interfaces)
}

/// Render the tabbed package listing with All / Interfaces / Components tabs.
fn render_tabs(
    all: &[KnownPackage],
    interfaces: &[&KnownPackage],
    components: &[&KnownPackage],
) -> Division {
    let all_refs: Vec<&KnownPackage> = all.iter().collect();

    let tabs: &[(&str, &str, &[&KnownPackage])] = &[
        ("all", "All", &all_refs),
        ("interfaces", "Types", interfaces),
        ("components", "Components", components),
    ];

    let mut wrapper = Division::builder();
    wrapper.class("tab-group");

    // Tab bar — pills style per design system
    let mut bar = Division::builder();
    bar.class("flex flex-wrap items-center gap-2 text-[13px] mb-6");
    bar.role("tablist");
    for (i, &(id, label, pkgs)) in tabs.iter().enumerate() {
        let count = pkgs.len();
        let selected = i == 0;
        let cls = if selected {
            "inline-flex items-center gap-2 px-3 h-8 rounded-pill bg-ink-900 text-canvas font-medium cursor-pointer"
        } else {
            "inline-flex items-center gap-2 px-3 h-8 rounded-pill bg-surfaceMuted text-ink-700 cursor-pointer hover:bg-ink-300 transition-colors"
        };
        bar.button(|btn| {
            btn.type_("button")
                .role("tab")
                .class(format!("tab-btn {cls}"))
                .data("tab", id)
                .aria_selected(selected)
                .aria_controls_elements(format!("panel-{id}"))
                .span(|s: &mut html::inline_text::builders::SpanBuilder| s.text(label.to_owned()))
                .span(|s: &mut html::inline_text::builders::SpanBuilder| {
                    s.class("text-[11px] opacity-70").text(format!("{count}"))
                })
        });
    }
    wrapper.push(bar.build());

    // Panels
    for (i, &(id, _label, pkgs)) in tabs.iter().enumerate() {
        let mut panel = Division::builder();
        panel
            .id(format!("panel-{id}"))
            .role("tabpanel")
            .class("tab-panel");
        if i != 0 {
            panel.style("display:none");
        }
        render_card_grid(&mut panel, pkgs);
        wrapper.push(panel.build());
    }

    wrapper.build()
}

/// Render a grid of package cards into a container, with a "view all" link
/// when the list is truncated.
fn render_card_grid(container: &mut DivisionBuilder, packages: &[&KnownPackage]) {
    if packages.is_empty() {
        container.paragraph(|p| {
            p.class("py-8 text-[13px] text-ink-400")
                .text("Nothing published yet. ")
                .anchor(|a| {
                    a.href("/docs")
                        .class("text-accent hover:underline")
                        .text("Learn how to publish")
                })
        });
        return;
    }

    let visible = packages.get(..HOME_SECTION_LIMIT).unwrap_or(packages);

    let mut grid = Division::builder();
    grid.class(package_card::grid(4));
    for pkg in visible {
        grid.push(package_card::render(pkg));
    }
    container.push(grid.build());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn package(kind: Option<wasm_meta_registry_client::PackageKind>) -> KnownPackage {
        KnownPackage {
            registry: "ghcr.io".to_string(),
            repository: "example/pkg".to_string(),
            kind,
            description: None,
            tags: vec!["1.0.0".to_string()],
            signature_tags: vec![],
            attestation_tags: vec![],
            last_seen_at: "2026-01-01T00:00:00Z".to_string(),
            created_at: "2026-01-01T00:00:00Z".to_string(),
            wit_namespace: Some("test".to_string()),
            wit_name: Some("demo".to_string()),
            dependencies: vec![],
        }
    }

    // r[verify frontend.pages.home]
    #[test]
    fn split_by_kind_uses_package_kind() {
        use wasm_meta_registry_client::PackageKind;

        let interface = package(Some(PackageKind::Interface));
        let component = package(Some(PackageKind::Component));
        let unknown = package(None);
        let input = vec![interface, component, unknown];

        let (components, interfaces) = split_by_kind(&input);
        assert_eq!(interfaces.len(), 1);
        assert_eq!(components.len(), 2);
        assert_eq!(interfaces[0].kind, Some(PackageKind::Interface));
        assert_eq!(components[0].kind, Some(PackageKind::Component));
        assert_eq!(components[1].kind, None);
    }

    fn named_package(ns: &str, name: &str) -> KnownPackage {
        KnownPackage {
            wit_namespace: Some(ns.to_string()),
            wit_name: Some(name.to_string()),
            ..package(Some(wasm_meta_registry_client::PackageKind::Interface))
        }
    }

    #[test]
    fn pin_and_sort_puts_pinned_first() {
        let packages = vec![
            named_package("other", "pkg"),
            named_package("wasi", "cli"),
            named_package("wasi", "http"),
        ];

        let sorted = pin_and_sort(&packages);
        assert_eq!(sorted[0].wit_name.as_deref(), Some("http"));
        assert_eq!(sorted[1].wit_name.as_deref(), Some("cli"));
        assert_eq!(sorted[2].wit_name.as_deref(), Some("pkg"));
    }

    #[test]
    fn pin_and_sort_handles_missing_pinned() {
        let packages = vec![
            named_package("other", "a"),
            named_package("wasi", "http"),
            named_package("other", "b"),
        ];

        let sorted = pin_and_sort(&packages);
        assert_eq!(sorted[0].wit_name.as_deref(), Some("http"));
        assert_eq!(sorted[1].wit_name.as_deref(), Some("a"));
        assert_eq!(sorted[2].wit_name.as_deref(), Some("b"));
    }
}
