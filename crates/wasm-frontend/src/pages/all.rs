//! All packages listing page.

// r[impl frontend.pages.all]

use wasm_meta_registry_client::KnownPackage;

use crate::api_client::ApiClient;
use crate::layout;

/// Fetch all packages and render a paginated list.
pub(crate) async fn render(client: &ApiClient) -> String {
    let packages = client.fetch_all_packages(0, 100).await;

    let mut body = String::new();

    body.push_str(r#"<h1 class="text-3xl font-bold mb-8">All Packages</h1>"#);

    if packages.is_empty() {
        body.push_str(
            r#"<p class="text-gray-500">No packages found. The registry may still be syncing.</p>"#,
        );
    } else {
        body.push_str(r#"<div class="space-y-2">"#);
        for pkg in &packages {
            body.push_str(&render_row(pkg));
        }
        body.push_str("</div>");
    }

    layout::document("All Packages", &body)
}

/// Render a single package row.
fn render_row(pkg: &KnownPackage) -> String {
    let display_name = match (&pkg.wit_namespace, &pkg.wit_name) {
        (Some(ns), Some(name)) => format!("{ns}:{name}"),
        _ => pkg.repository.clone(),
    };

    let href = match (&pkg.wit_namespace, &pkg.wit_name) {
        (Some(ns), Some(name)) => format!("/{ns}/{name}"),
        _ => "#".to_string(),
    };

    let description = pkg.description.as_deref().unwrap_or("");

    let version = pkg.tags.first().map_or("—", String::as_str);

    format!(
        r#"  <a href="{href}" class="flex items-center justify-between border border-gray-200 rounded-lg px-4 py-3 hover:border-accent hover:shadow-sm transition-colors">
    <div>
      <span class="font-mono font-semibold text-accent">{display_name}</span>
      <span class="text-sm text-gray-500 ml-2">{version}</span>
      <p class="text-sm text-gray-600 mt-0.5 line-clamp-1">{description}</p>
    </div>
  </a>
"#
    )
}
