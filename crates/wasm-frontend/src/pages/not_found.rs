//! 404 Not Found page.

// r[impl frontend.pages.not-found]

use crate::layout;

/// Render a user-friendly 404 page.
#[must_use]
pub(crate) fn render() -> String {
    let body = r#"<div class="text-center py-20">
  <h1 class="text-6xl font-bold text-accent">404</h1>
  <p class="text-xl text-gray-600 mt-4">Page not found</p>
  <p class="text-gray-500 mt-2">The page you're looking for doesn't exist or has been moved.</p>
  <a href="/" class="inline-block mt-8 px-6 py-3 bg-accent text-white rounded-lg font-medium hover:opacity-90 transition-opacity">
    Go to Home
  </a>
</div>"#;

    layout::document("Not Found", body)
}
