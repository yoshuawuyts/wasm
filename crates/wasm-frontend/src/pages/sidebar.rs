//! Shared sidebar components for detail pages.
//!
//! Provides a navigation sidebar showing sibling interfaces/worlds and
//! package metadata, using the DS nested sidebar component (C01).

use crate::components::ds::sidebar::{self, SidebarEntry, SidebarGroup, SidebarItem};
use crate::wit_doc::WitDocument;
use html::content::Aside;
use wasm_meta_registry_client::OciAnnotations;

/// Context needed to render the detail page sidebar.
pub(crate) struct SidebarContext<'a> {
    /// The package display name (e.g. `"wasi:cli"`).
    pub display_name: &'a str,
    /// The current version string.
    pub version: &'a str,
    /// All available version tags (newest first).
    pub versions: &'a [String],
    /// The parsed WIT document for navigation links.
    pub doc: &'a WitDocument,
    /// Which sidebar item is currently active.
    pub active: SidebarActive<'a>,
    /// OCI annotations for the current version (optional).
    pub annotations: Option<&'a OciAnnotations>,
}

/// Which item in the sidebar is currently active.
pub(crate) enum SidebarActive<'a> {
    /// An interface page (name of the interface).
    Interface(&'a str),
    /// An item within an interface (interface name, item name).
    Item(&'a str, #[allow(dead_code)] &'a str),
    /// A world page (name of the world).
    World(&'a str),
}

/// Render the sidebar for a detail page using the DS nested sidebar.
pub(crate) fn render_sidebar(ctx: &SidebarContext<'_>) -> Aside {
    let pkg_url = format!("/{}/{}", ctx.display_name.replace(':', "/"), ctx.version);

    let mut items: Vec<SidebarItem> = Vec::new();

    // Package root entry
    items.push(SidebarItem::Entry(SidebarEntry {
        sigil_bg: "var(--c-cat-slate)",
        sigil_color: "var(--c-cat-slate-ink)",
        sigil_text: "\u{00b7}",
        name: ctx.display_name.to_owned(),
        href: pkg_url,
        meta: String::new(),
        active: false,
    }));

    // Worlds — each world is a group with its imports/exports as children
    for world in &ctx.doc.worlds {
        let is_active = matches!(ctx.active, SidebarActive::World(name) if name == world.name);
        let mut children = Vec::new();
        for item in world.imports.iter().chain(world.exports.iter()) {
            if let crate::wit_doc::WorldItemDoc::Interface {
                name,
                url: Some(url),
                ..
            } = item
            {
                children.push(SidebarEntry {
                    sigil_bg: "var(--c-cat-lilac)",
                    sigil_color: "var(--c-cat-lilac-ink)",
                    sigil_text: "I",
                    name: name.clone(),
                    href: url.clone(),
                    meta: String::new(),
                    active: false,
                });
            }
        }
        items.push(SidebarItem::Group(SidebarGroup {
            label: world.name.clone(),
            href: Some(world.url.clone()),
            sigil_bg: Some("var(--c-cat-green)"),
            sigil_color: Some("var(--c-cat-green-ink)"),
            sigil_text: Some("W"),
            open: is_active,
            count: None,
            children,
        }));
    }

    // Interfaces — each interface is a group with its types and functions as children
    for iface in &ctx.doc.interfaces {
        let is_active = matches!(
            ctx.active,
            SidebarActive::Interface(name) if name == iface.name
        ) || matches!(
            ctx.active,
            SidebarActive::Item(iface_name, _) if iface_name == iface.name
        );
        let mut children = Vec::new();
        for ty in &iface.types {
            children.push(SidebarEntry {
                sigil_bg: "var(--c-cat-blue)",
                sigil_color: "var(--c-cat-blue-ink)",
                sigil_text: "T",
                name: ty.name.clone(),
                href: ty.url.clone(),
                meta: String::new(),
                active: matches!(
                    ctx.active,
                    SidebarActive::Item(_, item_name) if item_name == ty.name
                ),
            });
        }
        for func in &iface.functions {
            children.push(SidebarEntry {
                sigil_bg: "var(--c-cat-green)",
                sigil_color: "var(--c-cat-green-ink)",
                sigil_text: "f",
                name: func.name.clone(),
                href: func.url.clone(),
                meta: String::new(),
                active: matches!(
                    ctx.active,
                    SidebarActive::Item(_, item_name) if item_name == func.name
                ),
            });
        }
        items.push(SidebarItem::Group(SidebarGroup {
            label: iface.name.clone(),
            href: Some(iface.url.clone()),
            sigil_bg: Some("var(--c-cat-lilac)"),
            sigil_color: Some("var(--c-cat-lilac-ink)"),
            sigil_text: Some("I"),
            open: is_active,
            count: None,
            children,
        }));
    }

    let version_strs: Vec<&str> = ctx.versions.iter().map(String::as_str).collect();
    let footer = build_project_footer(ctx.annotations);
    let sidebar_html = sidebar::render_nested_sidebar(
        ctx.version,
        &version_strs,
        Some("Items"),
        &items,
        footer.as_deref(),
    );

    Aside::builder()
        .class("space-y-4")
        .text(sidebar_html)
        .build()
}

/// Build a "Project" footer section from OCI annotations.
///
/// Shows links (source, documentation, url) and metadata rows
/// (license, authors, vendor) when available.
fn build_project_footer(annotations: Option<&OciAnnotations>) -> Option<String> {
    let ann = annotations?;

    let mut rows = Vec::new();

    // Link rows
    if let Some(source) = &ann.source {
        let label = source_label(source);
        rows.push(format!(
            r#"<a href="{source}" class="tree-link" target="_blank" rel="noopener">{label}</a>"#
        ));
    }
    if let Some(docs) = &ann.documentation {
        rows.push(format!(
            r#"<a href="{docs}" class="tree-link" target="_blank" rel="noopener">Documentation</a>"#
        ));
    }
    if let Some(url) = &ann.url {
        // Only show if different from source
        if ann.source.as_deref() != Some(url) {
            rows.push(format!(
                r#"<a href="{url}" class="tree-link" target="_blank" rel="noopener">Homepage</a>"#
            ));
        }
    }

    // Metadata rows
    if let Some(license) = &ann.licenses {
        rows.push(detail_row("License", license));
    }
    if let Some(authors) = &ann.authors {
        rows.push(detail_row("Authors", authors));
    }
    if let Some(vendor) = &ann.vendor {
        rows.push(detail_row("Vendor", vendor));
    }

    if rows.is_empty() {
        return None;
    }

    let items = rows.join("");
    Some(format!(
        r#"<div class="mt-5 pt-4 border-t-[1.5px] border-rule"><div class="mono uppercase tracking-wider text-[10px] text-ink-500 mb-2">Project</div><nav class="space-y-px">{items}</nav></div>"#
    ))
}

/// Render a key-value detail row for the project section.
fn detail_row(label: &str, value: &str) -> String {
    format!(
        r#"<div class="flex items-baseline justify-between gap-4 py-1 text-[12px]"><span class="text-ink-500">{label}</span><span class="text-ink-700 mono text-right truncate">{value}</span></div>"#
    )
}

/// Derive a friendly label from a source URL.
fn source_label(url: &str) -> &'static str {
    if url.contains("github.com") {
        "GitHub"
    } else if url.contains("gitlab.com") {
        "GitLab"
    } else if url.contains("codeberg.org") {
        "Codeberg"
    } else {
        "Source"
    }
}
