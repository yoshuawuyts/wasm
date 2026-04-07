//! Navigation bar component.

/// Render the site navigation bar.
#[must_use]
pub(crate) fn render() -> String {
    format!(
        r#"<header class="bg-accent text-white">
  <nav class="max-w-5xl mx-auto px-4 py-3 flex items-center justify-between">
    <a href="/" class="text-xl font-bold tracking-tight hover:opacity-90">wasm</a>
    <div class="hidden sm:flex gap-6 text-sm font-medium">
      <a href="/all" class="hover:opacity-80">All Packages</a>
      <a href="/about" class="hover:opacity-80">About</a>
    </div>
    <button
      class="sm:hidden p-1"
      onclick="document.getElementById('mobile-nav').classList.toggle('hidden')"
      aria-label="Toggle menu"
    >
      <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
              d="M4 6h16M4 12h16M4 18h16"/>
      </svg>
    </button>
  </nav>
  <div id="mobile-nav" class="hidden sm:hidden px-4 pb-3 space-y-2 text-sm font-medium">
    <a href="/all" class="block hover:opacity-80">All Packages</a>
    <a href="/about" class="block hover:opacity-80">About</a>
  </div>
</header>"#
    )
}
