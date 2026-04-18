//! Navigation bar component.

use crate::components::search_bar;

/// A breadcrumb segment: (label, optional href).
pub(crate) struct Crumb {
    /// Display text.
    pub label: String,
    /// Link target, or `None` for the current (last) segment.
    pub href: Option<String>,
}

/// Render the site navigation bar with home link, breadcrumbs, and search.
#[must_use]
pub(crate) fn render(crumbs: &[Crumb]) -> String {
    let mut breadcrumb_html = String::new();
    for (i, crumb) in crumbs.iter().enumerate() {
        if i == 0 {
            breadcrumb_html.push(' ');
        } else {
            breadcrumb_html.push_str(r#" <span class="text-ink-400 mx-0.5">/</span> "#);
        }
        if let Some(href) = &crumb.href {
            use std::fmt::Write;
            write!(
                breadcrumb_html,
                r#"<a href="{href}" class="text-ink-500 hover:text-ink-900 transition-colors">{label}</a>"#,
                label = crumb.label
            )
            .unwrap();
        } else {
            use std::fmt::Write;
            write!(
                breadcrumb_html,
                r#"<span class="text-ink-900">{label}</span>"#,
                label = crumb.label
            )
            .unwrap();
        }
    }

    let search = search_bar::compact("search-input");

    format!(
        r#"<nav class="w-full max-w-6xl mx-auto px-6 sm:px-8 pt-6 pb-4 flex flex-wrap items-baseline justify-between gap-x-4 gap-y-2" aria-label="Main">
  <div class="flex flex-wrap items-baseline text-2xl font-mono font-medium">
    <a href="/" id="bunny" aria-label="Home" role="link" class="text-lg font-mono font-medium text-ink-900 hover:text-accent transition-colors shrink-0 inline-block text-left" style="cursor:pointer;min-width:10ch">(๑╹ᆺ╹)</a>{breadcrumb_html}
  </div>
  <div class="flex items-center gap-3 sm:gap-5 shrink-0">
    <a href="/docs" class="text-[13px] text-ink-500 hover:text-ink-900 transition-colors">Docs</a>
    <a href="/downloads" class="text-[13px] text-ink-500 hover:text-ink-900 transition-colors hidden sm:inline">Downloads</a>
    <div class="hidden sm:block">{search}</div>
  </div>
  <script>
  (function(){{
    var b=document.getElementById('bunny');
    if(!b)return;
    var anims=[
      ['(๑╹ᆺ╹)','(๑°ᆺ°)!','(๑◉ᆺ◉)!!'],
      ['(๑╹ᆺ╹)','(๑°ᆺ°)♪','ヽ(๑≧ᆺ≦)ノ'],
      ['(๑╹ᆺ╹)','(๑╹ᆺ╹)>','(๑°ᆺ°)>>']
    ];
    var seq=anims[Math.floor(Math.random()*anims.length)];
    var timer=null;
    b.addEventListener('mouseenter',function(){{
      if(timer)return;
      b.textContent=seq[1];
      timer=setTimeout(function(){{
        b.textContent=seq[2];
      }},80);
    }});
    b.addEventListener('mouseleave',function(){{
      if(timer){{clearTimeout(timer);timer=null;}}
      b.textContent=seq[0];
    }});
  }})();
  </script>
</nav>"#,
    )
}
