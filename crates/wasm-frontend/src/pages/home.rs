//! Front page — recently updated components and interfaces.

// r[impl frontend.pages.home]

use wasm_meta_registry_client::KnownPackage;

use crate::api_client::ApiClient;
use crate::layout;

/// Fetch recent packages and render the home page.
pub(crate) async fn render(client: &ApiClient) -> String {
    let packages = client.fetch_recent_packages(50).await;

    let (components, interfaces) = split_by_kind(&packages);

    let mut body = String::new();

    body.push_str(r#"<h1 class="text-3xl font-bold mb-8">WebAssembly Package Registry</h1>"#);

    body.push_str(&render_section(
        "Recently Updated Interfaces",
        &interfaces,
    ));
    body.push_str(&render_section(
        "Recently Updated Components",
        &components,
    ));

    if packages.is_empty() {
        body.push_str(
            r#"<p class="text-gray-500 mt-8">No packages found. The registry may still be syncing.</p>"#,
        );
    }

    layout::document("Home", &body)
}

/// Split packages into (components, interfaces) based on WIT metadata.
///
/// Packages with a `wit_name` are considered interfaces unless their
/// repository path suggests they are a component (heuristic: no `/`
/// separator in the WIT name is ambiguous, so we default to interface).
fn split_by_kind(packages: &[KnownPackage]) -> (Vec<&KnownPackage>, Vec<&KnownPackage>) {
    let mut components = Vec::new();
    let mut interfaces = Vec::new();

    for pkg in packages {
        // Packages without WIT metadata go into components as a fallback
        if pkg.wit_namespace.is_none() {
            components.push(pkg);
        } else {
            interfaces.push(pkg);
        }
    }

    (components, interfaces)
}

/// Render a section with a heading and a grid of package cards.
fn render_section(heading: &str, packages: &[&KnownPackage]) -> String {
    if packages.is_empty() {
        return String::new();
    }

    let mut html = format!(
        r#"<section class="mb-10">
  <h2 class="text-xl font-semibold mb-4">{heading}</h2>
  <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">"#
    );

    for pkg in packages {
        html.push_str(&render_card(pkg));
    }

    html.push_str("  </div>\n</section>\n");
    html
}

/// Render a single package card.
fn render_card(pkg: &KnownPackage) -> String {
    let display_name = match (&pkg.wit_namespace, &pkg.wit_name) {
        (Some(ns), Some(name)) => format!("{ns}:{name}"),
        _ => pkg.repository.clone(),
    };

    let href = match (&pkg.wit_namespace, &pkg.wit_name) {
        (Some(ns), Some(name)) => format!("/{ns}/{name}"),
        _ => "#".to_string(),
    };

    let description = pkg
        .description
        .as_deref()
        .unwrap_or("No description available");

    let version = pkg
        .tags
        .first()
        .map(String::as_str)
        .unwrap_or("—");

    format!(
        r#"    <a href="{href}" class="block border border-gray-200 rounded-lg p-4 hover:border-accent hover:shadow-sm transition-colors">
      <h3 class="font-mono font-semibold text-accent">{display_name}</h3>
      <p class="text-sm text-gray-500 mt-1">{version}</p>
      <p class="text-sm text-gray-600 mt-2 line-clamp-2">{description}</p>
    </a>
"#
    )
}
