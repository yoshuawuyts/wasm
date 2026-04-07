//! Package detail page.

// r[impl frontend.pages.package-detail]

use wasm_meta_registry_client::KnownPackage;

use crate::layout;

/// Render the package detail page for a given package and version.
#[must_use]
pub(crate) fn render(pkg: &KnownPackage, version: &str) -> String {
    let display_name = match (&pkg.wit_namespace, &pkg.wit_name) {
        (Some(ns), Some(name)) => format!("{ns}:{name}"),
        _ => pkg.repository.clone(),
    };

    let description = pkg
        .description
        .as_deref()
        .unwrap_or("No description available");

    let mut body = String::new();

    // Breadcrumb + title
    body.push_str(&format!(
        r#"<nav class="text-sm text-gray-500 mb-4">
  <a href="/" class="hover:text-accent">Home</a>
  <span class="mx-1">/</span>
  <span>{display_name}</span>
</nav>
<div class="mb-8">
  <h1 class="text-3xl font-bold font-mono text-accent">{display_name}</h1>
  <p class="text-lg text-gray-600 mt-2">{description}</p>
</div>"#
    ));

    // Metadata section
    body.push_str(r#"<div class="grid grid-cols-1 md:grid-cols-3 gap-8">"#);

    // Main content
    body.push_str(r#"<div class="md:col-span-2 space-y-6">"#);

    // Tags / versions
    body.push_str(&render_tags(pkg, version));

    // Dependencies
    body.push_str(&render_dependencies(pkg));

    body.push_str("</div>");

    // Sidebar
    body.push_str(&render_sidebar(pkg));

    body.push_str("</div>");

    layout::document(&display_name, &body)
}

/// Render the tags/versions section.
fn render_tags(pkg: &KnownPackage, current_version: &str) -> String {
    if pkg.tags.is_empty() {
        return String::new();
    }

    let display_name = match (&pkg.wit_namespace, &pkg.wit_name) {
        (Some(ns), Some(name)) => format!("{ns}/{name}"),
        _ => pkg.repository.clone(),
    };

    let mut html = String::from(
        r#"<section>
  <h2 class="text-lg font-semibold mb-3">Versions</h2>
  <div class="flex flex-wrap gap-2">"#,
    );

    for tag in &pkg.tags {
        let is_current = tag == current_version;
        let classes = if is_current {
            "bg-accent text-white"
        } else {
            "bg-gray-100 text-gray-700 hover:bg-gray-200"
        };
        html.push_str(&format!(
            r#"    <a href="/{display_name}/{tag}" class="px-3 py-1 rounded-full text-sm font-mono {classes}">{tag}</a>
"#
        ));
    }

    html.push_str("  </div>\n</section>\n");
    html
}

/// Render the dependencies section.
fn render_dependencies(pkg: &KnownPackage) -> String {
    if pkg.dependencies.is_empty() {
        return String::new();
    }

    let mut html = String::from(
        r#"<section>
  <h2 class="text-lg font-semibold mb-3">Dependencies</h2>
  <ul class="space-y-1">"#,
    );

    for dep in &pkg.dependencies {
        let version_badge = dep
            .version
            .as_deref()
            .map(|v| format!(r#" <span class="text-gray-400">{v}</span>"#))
            .unwrap_or_default();
        html.push_str(&format!(
            r#"    <li class="font-mono text-sm">
      <span class="text-accent">{}</span>{version_badge}
    </li>
"#,
            dep.package
        ));
    }

    html.push_str("  </ul>\n</section>\n");
    html
}

/// Render the sidebar with metadata.
fn render_sidebar(pkg: &KnownPackage) -> String {
    let mut html = String::from(
        r#"<aside class="space-y-4">
  <div class="border border-gray-200 rounded-lg p-4 space-y-3 text-sm">"#,
    );

    html.push_str(&sidebar_row("Registry", &pkg.registry));
    html.push_str(&sidebar_row("Repository", &pkg.repository));
    html.push_str(&sidebar_row("Created", &pkg.created_at));
    html.push_str(&sidebar_row("Last updated", &pkg.last_seen_at));

    html.push_str("  </div>\n</aside>\n");
    html
}

/// Render a single sidebar metadata row.
fn sidebar_row(label: &str, value: &str) -> String {
    format!(
        r#"    <div>
      <dt class="text-gray-500 text-xs uppercase tracking-wide">{label}</dt>
      <dd class="font-mono text-gray-900 mt-0.5 break-all">{value}</dd>
    </div>
"#
    )
}
