//! Footer component.

/// Render the site footer.
#[must_use]
pub(crate) fn render() -> String {
    String::from(
        r#"<footer class="border-t border-gray-200 mt-12">
  <div class="max-w-5xl mx-auto px-4 py-6 text-center text-sm text-gray-500">
    <p>wasm registry &mdash; a <a href="https://bytecodealliance.org" class="text-accent hover:underline">Bytecode Alliance</a> project</p>
  </div>
</footer>"#,
    )
}
