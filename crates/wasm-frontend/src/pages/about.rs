//! About page (placeholder).

use crate::layout;

/// Render a simple about page.
#[must_use]
pub(crate) fn render() -> String {
    let body = r#"<div class="max-w-2xl">
  <h1 class="text-3xl font-bold mb-6">About</h1>
  <p class="text-gray-600 leading-relaxed">
    The WebAssembly Package Registry is a discovery service for WebAssembly
    components and interfaces. It indexes packages from OCI registries and
    provides a browsable frontend for exploring the ecosystem.
  </p>
  <p class="text-gray-600 leading-relaxed mt-4">
    This frontend is itself a WebAssembly component, compiled to
    <code class="bg-gray-100 px-1.5 py-0.5 rounded text-sm">wasm32-wasip2</code>
    and served via <code class="bg-gray-100 px-1.5 py-0.5 rounded text-sm">wasi:http</code>.
  </p>
</div>"#;

    layout::document("About", body)
}
