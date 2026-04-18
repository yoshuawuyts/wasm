//! Base HTML document layout.
//!
//! Provides the shared page shell — `<html>`, `<head>`, and `<body>` wrapper —
//! used by all pages.

// r[impl frontend.rendering.html-crate]
// r[impl frontend.styling.tailwind]
// r[impl frontend.styling.light-theme]
// r[impl frontend.styling.accent-color]
// r[impl frontend.styling.responsive]

use crate::footer;

/// Accent color used throughout the UI.
pub(crate) const ACCENT_COLOR: &str = "#18181B";

/// Render a complete HTML document with the given title and body content.
///
/// Includes the shared navigation bar, Tailwind CSS via CDN, custom accent
/// color CSS variables, and footer.
#[must_use]
pub(crate) fn document(title: &str, body_content: &str) -> String {
    document_inner(title, body_content, "", MAIN_CLASS_CENTERED, true)
}

/// Render a complete HTML document with nav bar, title, and body content.
#[must_use]
pub(crate) fn document_with_nav(title: &str, body_content: &str) -> String {
    let nav = crate::nav::render(&[]);
    document_inner(title, body_content, &nav, MAIN_CLASS_CENTERED, true)
}

/// Render a full-width document (no centered max-width, no top nav, no footer).
///
/// Used by the golden-layout pages where the sidebar is flush left.
#[must_use]
pub(crate) fn document_full_width(title: &str, body_content: &str) -> String {
    document_inner(title, body_content, "", MAIN_CLASS_FULL, false)
}

const MAIN_CLASS_CENTERED: &str = "flex-1 w-full max-w-6xl mx-auto px-6 sm:px-8 pb-12";
const MAIN_CLASS_FULL: &str = "flex-1 w-full";

/// Inner document renderer.
fn document_inner(
    title: &str,
    body_content: &str,
    nav: &str,
    main_class: &str,
    show_footer: bool,
) -> String {
    let escaped_title = escape_html_text(title);

    format!(
        r#"<!DOCTYPE html>
<html lang="en" style="view-transition-name:root">
<head>
  <meta charset="utf-8">
  <meta name="color-scheme" content="light dark">
  <style>html{{background:#F4F4F5}}@media(prefers-color-scheme:dark){{html:not([data-theme=light]){{background:#1C1C20}}}}html[data-theme=dark]{{background:#1C1C20}}html[data-theme=light]{{background:#F4F4F5}}</style>
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <meta name="description" content="Browse and discover WebAssembly components and WIT interfaces published to OCI registries.">
  <title>{escaped_title} — wasm registry</title>
  <script src="https://cdn.tailwindcss.com"></script>
  <script>
    /* Early theme init — prevent flash of wrong theme */
    (function() {{
      var t = localStorage.getItem('ds-theme');
      if (t === 'dark' || t === 'light') {{
        document.documentElement.setAttribute('data-theme', t);
        document.documentElement.style.background = t === 'dark' ? '#1C1C20' : '#F4F4F5';
      }} else if (window.matchMedia('(prefers-color-scheme: dark)').matches) {{
        document.documentElement.style.background = '#1C1C20';
      }}
    }})();
  </script>
  <script>
    tailwind.config = {{
      theme: {{
        extend: {{
          colors: {{
            /* — New design system tokens — */
            canvas: 'var(--c-canvas)',
            surface: {{
              DEFAULT: 'var(--c-surface)',
              muted:   'var(--c-surface-muted)',
            }},
            surfaceMuted: 'var(--c-surface-muted)',
            ink: {{
              900: 'var(--c-ink-900)',
              700: 'var(--c-ink-700)',
              500: 'var(--c-ink-500)',
              400: 'var(--c-ink-400)',
              300: 'var(--c-ink-300)',
            }},
            line: 'var(--c-line)',
            lineSoft: 'var(--c-line-soft)',
            rule: 'var(--c-rule)',
            positive: 'var(--c-positive)',
            negative: 'var(--c-negative)',
            accent: 'var(--c-accent)',
            cat: {{
              blue: 'var(--c-cat-blue)',       blueInk: 'var(--c-cat-blue-ink)',
              pink: 'var(--c-cat-pink)',       pinkInk: 'var(--c-cat-pink-ink)',
              green: 'var(--c-cat-green)',     greenInk: 'var(--c-cat-green-ink)',
              peach: 'var(--c-cat-peach)',     peachInk: 'var(--c-cat-peach-ink)',
              lilac: 'var(--c-cat-lilac)',     lilacInk: 'var(--c-cat-lilac-ink)',
              cream: 'var(--c-cat-cream)',     creamInk: 'var(--c-cat-cream-ink)',
              teal: 'var(--c-cat-teal)',       tealInk: 'var(--c-cat-teal-ink)',
              rust: 'var(--c-cat-rust)',       rustInk: 'var(--c-cat-rust-ink)',
              plum: 'var(--c-cat-plum)',       plumInk: 'var(--c-cat-plum-ink)',
              slate: 'var(--c-cat-slate)',     slateInk: 'var(--c-cat-slate-ink)',
            }},
            /* WIT semantic colors */
            wit: {{
              struct:   'var(--color-wit-struct)',
              enum:     'var(--color-wit-enum)',
              resource: 'var(--color-wit-resource)',
              func:     'var(--color-wit-func)',
              world:    'var(--color-wit-world)',
              iface:    'var(--color-wit-iface)',
              import:   'var(--color-wit-import)',
              module:   'var(--color-wit-module)',
            }},
          }},
          fontFamily: {{
            sans: ['-apple-system', 'BlinkMacSystemFont', 'system-ui', '"Segoe UI"', '"Helvetica Neue"', 'Helvetica', 'Arial', 'sans-serif'],
            mono: ['ui-monospace', 'SFMono-Regular', 'Menlo', 'Monaco', 'Consolas', 'monospace'],
          }},
          letterSpacing: {{
            display: '-0.06em',
          }},
          fontSize: {{
            sm: ['0.875rem', {{ lineHeight: '1.375rem' }}],
            lg: ['1.125rem', {{ lineHeight: '1.625rem' }}],
          }},
          boxShadow: {{
            tooltip: 'var(--shadow-tooltip)',
            card: 'var(--shadow-card)',
          }},
          borderRadius: {{
            DEFAULT: '3px',
            sm: '2px',
            md: '4px',
            lg: '5px',
            pill: '9999px',
          }},
          transitionTimingFunction: {{
            standard: 'cubic-bezier(0.2, 0, 0, 1)',
            entrance: 'cubic-bezier(0, 0, 0, 1)',
            exit: 'cubic-bezier(0.4, 0, 1, 1)',
            spring: 'cubic-bezier(0.34, 1.56, 0.64, 1)',
          }},
          transitionDuration: {{
            fast: '120ms',
            base: '180ms',
            slow: '260ms',
            page: '360ms',
          }},
        }}
      }}
    }}
  </script>
  <style>
    /* ── Design system tokens ─────────────────────────────── */
    :root {{
      color-scheme: light dark;

      /* Light mode — calm paper, near-black ink, pastel categoricals */
      --c-canvas:        #F4F4F5;
      --c-surface:       #FFFFFF;
      --c-surface-muted: #E8E8EA;
      --c-ink-900:       {ACCENT_COLOR};
      --c-ink-700:       #3F3F46;
      --c-ink-500:       #71717A;
      --c-ink-400:       #A1A1AA;
      --c-ink-300:       #D4D4D8;
      --c-line:          #D4D4D8;
      --c-line-soft:     #E4E4E7;
      --c-positive:      #1F8A4C;
      --c-negative:      #9B4F5E;
      --c-accent:        {ACCENT_COLOR};

      --c-cat-blue:      #D6E4FF;  --c-cat-blue-ink:   #3D5A99;
      --c-cat-pink:      #FBD9DF;  --c-cat-pink-ink:   #9B4F5E;
      --c-cat-green:     #D2ECD8;  --c-cat-green-ink:  #3F7A52;
      --c-cat-peach:     #F8E2C2;  --c-cat-peach-ink:  #8E6529;
      --c-cat-lilac:     #E4DAF1;  --c-cat-lilac-ink:  #6B528F;
      --c-cat-cream:     #F4ECC2;  --c-cat-cream-ink:  #7A6A2A;
      --c-cat-teal:      #BFE3EE;  --c-cat-teal-ink:   #1F6F87;
      --c-cat-rust:      #F4D2C0;  --c-cat-rust-ink:   #9F5536;
      --c-cat-plum:      #E8C5E8;  --c-cat-plum-ink:   #7E2E7E;
      --c-cat-slate:     #DADCE0;  --c-cat-slate-ink:  #535A66;

      --c-rule:          var(--c-ink-900);
      --c-scrollbar:     #D4D4D8;

      --shadow-tooltip:  0 8px 24px -8px rgba(20,22,28,0.35);
      --shadow-card:     0 1px 0 0 rgba(20,22,28,0.04);

      /* WIT syntax coloring */
      --color-wit-struct:   #4338ca;
      --color-wit-enum:     #0d7377;
      --color-wit-resource: #b45309;
      --color-wit-func:     #15803d;
      --color-wit-world:    #9333ea;
      --color-wit-iface:    #0369a1;
      --color-wit-import:   #b91c1c;
      --color-wit-module:   #be185d;

    }}

    @media (prefers-color-scheme: dark) {{
      :root:not([data-theme="light"]) {{
        --c-canvas:        #1C1C20;
        --c-surface:       #26262B;
        --c-surface-muted: #2F2F35;
        --c-ink-900:       #ECECEE;
        --c-ink-700:       #B5B5BB;
        --c-ink-500:       #8B8B92;
        --c-ink-400:       #76767D;
        --c-ink-300:       #4A4A50;
        --c-line:          #3A3A40;
        --c-line-soft:     #323238;
        --c-positive:      #5EC787;
        --c-negative:      #EE7B8E;
        --c-accent:        #8FB1F5;

        --c-cat-blue:      #B8D0FF;  --c-cat-blue-ink:   #1F3F8C;
        --c-cat-pink:      #FFB8B0;  --c-cat-pink-ink:   #9E2823;
        --c-cat-green:     #B5E8C0;  --c-cat-green-ink:  #1F6738;
        --c-cat-peach:     #FBD3A0;  --c-cat-peach-ink:  #7A4E10;
        --c-cat-lilac:     #C6B1F0;  --c-cat-lilac-ink:  #422684;
        --c-cat-cream:     #F5E696;  --c-cat-cream-ink:  #6B5610;
        --c-cat-teal:      #A6DDF0;  --c-cat-teal-ink:   #0F5C7A;
        --c-cat-rust:      #F5BFA0;  --c-cat-rust-ink:   #87401C;
        --c-cat-plum:      #DDB2EF;  --c-cat-plum-ink:   #571485;
        --c-cat-slate:     #C6CDD8;  --c-cat-slate-ink:  #424B5C;

        --c-rule:          #6B6B72;
        --c-scrollbar:     #4A4A50;

        --shadow-tooltip:  0 10px 28px -10px rgba(0,0,0,0.7);
        --shadow-card:     inset 0 1px 0 0 rgba(255,255,255,0.06), 0 1px 0 0 rgba(0,0,0,0.5), 0 8px 16px -12px rgba(0,0,0,0.6);

        /* WIT dark variants — brighter to read against dark canvas */
        --color-wit-struct:   #818cf8;
        --color-wit-enum:     #2dd4bf;
        --color-wit-resource: #fbbf24;
        --color-wit-func:     #4ade80;
        --color-wit-world:    #c084fc;
        --color-wit-iface:    #38bdf8;
        --color-wit-import:   #f87171;
        --color-wit-module:   #f472b6;
      }}
    }}

    :root[data-theme="dark"] {{
      --c-canvas:        #1C1C20;
      --c-surface:       #26262B;
      --c-surface-muted: #2F2F35;
      --c-ink-900:       #ECECEE;
      --c-ink-700:       #B5B5BB;
      --c-ink-500:       #8B8B92;
      --c-ink-400:       #76767D;
      --c-ink-300:       #4A4A50;
      --c-line:          #3A3A40;
      --c-line-soft:     #323238;
      --c-positive:      #5EC787;
      --c-negative:      #EE7B8E;
      --c-accent:        #8FB1F5;

      --c-cat-blue:      #B8D0FF;  --c-cat-blue-ink:   #1F3F8C;
      --c-cat-pink:      #FFB8B0;  --c-cat-pink-ink:   #9E2823;
      --c-cat-green:     #B5E8C0;  --c-cat-green-ink:  #1F6738;
      --c-cat-peach:     #FBD3A0;  --c-cat-peach-ink:  #7A4E10;
      --c-cat-lilac:     #C6B1F0;  --c-cat-lilac-ink:  #422684;
      --c-cat-cream:     #F5E696;  --c-cat-cream-ink:  #6B5610;
      --c-cat-teal:      #A6DDF0;  --c-cat-teal-ink:   #0F5C7A;
      --c-cat-rust:      #F5BFA0;  --c-cat-rust-ink:   #87401C;
      --c-cat-plum:      #DDB2EF;  --c-cat-plum-ink:   #571485;
      --c-cat-slate:     #C6CDD8;  --c-cat-slate-ink:  #424B5C;

      --c-rule:          #6B6B72;
      --c-scrollbar:     #4A4A50;

      --shadow-tooltip:  0 10px 28px -10px rgba(0,0,0,0.7);
      --shadow-card:     inset 0 1px 0 0 rgba(255,255,255,0.06), 0 1px 0 0 rgba(0,0,0,0.5), 0 8px 16px -12px rgba(0,0,0,0.6);

      --color-wit-struct:   #818cf8;
      --color-wit-enum:     #2dd4bf;
      --color-wit-resource: #fbbf24;
      --color-wit-func:     #4ade80;
      --color-wit-world:    #c084fc;
      --color-wit-iface:    #38bdf8;
      --color-wit-import:   #f87171;
      --color-wit-module:   #f472b6;
    }}

    html, body {{
      background-color: var(--c-canvas);
      color: var(--c-ink-900);
      -webkit-font-smoothing: antialiased;
    }}
    /* Consistent focus ring for keyboard navigation */
    :focus-visible {{
      outline: 2px solid var(--c-accent);
      outline-offset: 2px;
    }}
    :focus:not(:focus-visible) {{
      outline: none;
    }}
    ::selection {{
      background: color-mix(in oklab, var(--c-accent) 35%, transparent);
      color: var(--c-ink-900);
    }}
    @view-transition {{
      navigation: auto;
    }}
    ::view-transition-old(root) {{
      animation: none;
    }}
    ::view-transition-new(root) {{
      animation: none;
    }}
    @media (prefers-reduced-motion: reduce) {{
      ::view-transition-old(root),
      ::view-transition-new(root) {{
        animation: none;
      }}
    }}
    /* Card hover — pop out with scale, shadow, and strong border */
    .card-lift {{
      transition: transform 120ms cubic-bezier(0.2, 0, 0, 1), box-shadow 120ms cubic-bezier(0.2, 0, 0, 1);
      transform-origin: center center;
    }}
    /* Prose styling for rendered markdown documentation */
    .prose-doc p {{
      margin-bottom: 0.75em;
    }}
    .prose-doc p:last-child {{
      margin-bottom: 0;
    }}
    .prose-doc code {{
      background: var(--c-surface-muted);
      padding: 0.1em 0.3em;
      font-size: 0.9em;
    }}
    .prose-doc a {{
      color: var(--c-accent);
      text-decoration: underline;
      text-underline-offset: 2px;
    }}
    .prose-doc a:hover {{
      opacity: 0.8;
    }}
    .prose-doc ul, .prose-doc ol {{
      margin: 0.5em 0;
      padding-left: 1.5em;
    }}
    .prose-doc li {{
      margin-bottom: 0.25em;
    }}
    .prose-doc pre {{
      background: var(--c-surface-muted);
      padding: 0.75em 1em;
      overflow-x: auto;
      margin: 0.75em 0;
      font-size: 0.875em;
    }}
    .card-lift:hover {{
      transform: scale(1.03);
      box-shadow: var(--shadow-card);
      z-index: 1;
      position: relative;
      outline: 2px solid var(--c-ink-900);
      outline-offset: -2px;
    }}
    @media (prefers-reduced-motion: reduce) {{
      .card-lift {{ transition: none; }}
      .card-lift:hover {{ transform: none; box-shadow: none; }}
    }}
    /* Card kind variants — thin left border for categorization */
    .card-interface {{
      border-left: 2px solid var(--color-wit-iface);
    }}
    .card-component {{
      border-left: 2px solid var(--c-accent);
    }}
    /* Copy hint */
    .copy-hint {{
      cursor: pointer;
      position: relative;
    }}
    .copy-hint::after {{
      content: 'click to copy';
      position: absolute;
      right: -0.25rem;
      top: 50%;
      transform: translateX(100%) translateY(-50%);
      font-size: 0.65rem;
      color: var(--c-ink-400);
      opacity: 0;
      transition: opacity 0.15s;
      white-space: nowrap;
      pointer-events: none;
    }}
    .copy-hint:hover::after {{
      opacity: 1;
    }}
    .copy-hint.copied::after {{
      content: 'copied!';
      color: var(--c-accent);
      opacity: 1;
    }}
    @media (prefers-reduced-motion: reduce) {{
      .copy-hint::after {{ transition: none; }}
    }}
    /* Keyboard shortcut badge — inside search input, Linear-style */
    .search-kbd {{
      position: absolute;
      right: 0.5rem;
      top: 50%;
      transform: translateY(-50%);
      display: inline-flex;
      align-items: center;
      justify-content: center;
      width: 1.5rem;
      height: 1.5rem;
      border: 2px solid var(--c-line);
      border-radius: 0;
      font-size: 0.8125rem;
      font-family: inherit;
      color: var(--c-ink-500);
      background: var(--c-surface-muted);
      line-height: 1;
      pointer-events: none;
      transition: opacity 0.1s;
    }}
    .search-form:focus-within .search-kbd {{
      opacity: 0;
      pointer-events: none;
    }}
    /* Search carousel placeholder */
    .search-carousel {{
      position: absolute;
      left: 1rem;
      top: 50%;
      transform: translateY(-50%);
      font-size: 1rem;
      color: var(--c-ink-400);
      pointer-events: none;
      white-space: nowrap;
      overflow: hidden;
      transition: opacity 0.3s cubic-bezier(0.25, 1, 0.5, 1);
    }}
    .carousel-word {{
      display: inline;
    }}
    @media (prefers-reduced-motion: reduce) {{
      .carousel-word {{
        transition: none;
      }}
    }}
    /* Tab buttons — pill style, managed via Tailwind classes.
       The .tab-btn class is only used as a JS selector. */
    .tab-btn {{
      cursor: pointer;
      transition: background-color 0.15s, color 0.15s;
    }}
    @media (prefers-reduced-motion: reduce) {{
      .tab-btn {{ transition: none; }}
    }}
    /* Theme toggle icon visibility */
    .theme-icon-sun  {{ display: none; }}
    .theme-icon-moon {{ display: inline-block; }}
    @media (prefers-color-scheme: dark) {{
      :root:not([data-theme="light"]) .theme-icon-sun  {{ display: inline-block; }}
      :root:not([data-theme="light"]) .theme-icon-moon {{ display: none; }}
    }}
    :root[data-theme="dark"] .theme-icon-sun  {{ display: inline-block; }}
    :root[data-theme="dark"] .theme-icon-moon {{ display: none; }}
    :root[data-theme="light"] .theme-icon-sun  {{ display: none; }}
    :root[data-theme="light"] .theme-icon-moon {{ display: inline-block; }}
  </style>
</head>
<body class="bg-canvas text-ink-900 min-h-screen flex flex-col leading-relaxed font-sans antialiased">
  {nav}
  <main class="{main_class}">
    {body_content}
  </main>
  {footer}
  <!-- Theme toggle button (fixed, bottom-right) -->
  <button id="theme-toggle" type="button" aria-label="Toggle dark mode"
    class="fixed bottom-4 right-4 z-50 w-9 h-9 flex items-center justify-center rounded-md bg-surface border border-line text-ink-500 hover:text-ink-900 transition-colors cursor-pointer"
    style="box-shadow:var(--shadow-card)">
    <svg class="theme-icon-moon w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M21 12.79A9 9 0 1111.21 3 7 7 0 0021 12.79z"/></svg>
    <svg class="theme-icon-sun w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="5"/><line x1="12" y1="1" x2="12" y2="3"/><line x1="12" y1="21" x2="12" y2="23"/><line x1="4.22" y1="4.22" x2="5.64" y2="5.64"/><line x1="18.36" y1="18.36" x2="19.78" y2="19.78"/><line x1="1" y1="12" x2="3" y2="12"/><line x1="21" y1="12" x2="23" y2="12"/><line x1="4.22" y1="19.78" x2="5.64" y2="18.36"/><line x1="18.36" y1="5.64" x2="19.78" y2="4.22"/></svg>
  </button>
  <script>
    // Focus search on / key (developer convention)
    document.addEventListener('keydown', function(e) {{
      if (e.key === '/' && !e.ctrlKey && !e.metaKey && !e.altKey) {{
        var el = document.activeElement;
        var tag = el && el.tagName;
        if (tag === 'INPUT' || tag === 'TEXTAREA' || tag === 'SELECT' || (el && el.isContentEditable)) return;
        var search = document.getElementById('search-input');
        if (search && search.offsetParent === null) search = document.getElementById('search-input-lg');
        if (search) {{ e.preventDefault(); search.focus(); }}
      }}
    }});
    // Click-to-copy for install hint
    document.addEventListener('click', function(e) {{
      var el = e.target.closest('.copy-hint');
      if (!el) return;
      var text = el.textContent || '';
      if (navigator.clipboard) {{
        navigator.clipboard.writeText(text).then(function() {{
          el.classList.add('copied');
          setTimeout(function() {{ el.classList.remove('copied'); }}, 1200);
        }});
      }}
    }});
    // Tab switching
    document.addEventListener('click', function(e) {{
      var btn = e.target.closest('.tab-btn');
      if (!btn) return;
      var group = btn.closest('.tab-group');
      if (!group) return;
      var tab = btn.getAttribute('data-tab');
      var activeClass = 'bg-ink-900 text-canvas font-medium';
      var inactiveClass = 'bg-surfaceMuted text-ink-700 hover:bg-ink-300';
      // Update tab buttons
      group.querySelectorAll('.tab-btn').forEach(function(b) {{
        var isActive = b === btn;
        b.setAttribute('aria-selected', isActive ? 'true' : 'false');
        activeClass.split(' ').forEach(function(c) {{
          if (isActive) b.classList.add(c); else b.classList.remove(c);
        }});
        inactiveClass.split(' ').forEach(function(c) {{
          if (isActive) b.classList.remove(c); else b.classList.add(c);
        }});
      }});
      // Show/hide panels
      group.querySelectorAll('.tab-panel').forEach(function(p) {{
        if (p.id === 'panel-' + tab) {{
          p.style.display = '';
        }} else {{
          p.style.display = 'none';
        }}
      }});
    }});
    // Search placeholder carousel
    (function() {{
      var words = [
        'components\u2026',
        'interfaces\u2026',
        'libraries\u2026',
        'plugins\u2026',
        'servers\u2026',
        'tools\u2026',
        'apps\u2026',
        'extensions\u2026',
        'handlers\u2026',
        'services\u2026',
        'applets\u2026',
        'clients\u2026',
        'addons\u2026',
        'modules\u2026',
        'packages\u2026',
        'widgets\u2026',
        'expansions\u2026',
        'augmentations\u2026',
        'supplements\u2026',
        'accessories\u2026',
        'middleware\u2026',
        'hooks\u2026',
        'mods\u2026',
        'bundles\u2026',
        'toolkits\u2026',
        'SDKs\u2026',
        'adapters\u2026',
        'drivers\u2026',
        'providers\u2026',
        'connectors\u2026',
        'shims\u2026',
        'polyfills\u2026',
      ];
      var el = document.getElementById('carousel-word');
      var overlay = document.getElementById('search-carousel');
      var input = document.getElementById('search-input');
      if (!el || !overlay || !input) return;
      var idx = 0;
      var reducedMotion = window.matchMedia('(prefers-reduced-motion: reduce)').matches;
      function updateVisibility() {{
        var hasValue = input.value.length > 0;
        overlay.style.opacity = hasValue ? '0' : '';
      }}
      input.addEventListener('input', updateVisibility);
      input.addEventListener('focus', updateVisibility);
      input.addEventListener('blur', updateVisibility);
      updateVisibility();

      var currentWord = words[idx];
      el.textContent = currentWord;
      var typing = false;

      function jitter() {{
        return 50 + Math.random() * 90;
      }}

      function deleteWord(cb) {{
        var text = el.textContent;
        if (text.length === 0) {{ cb(); return; }}
        typing = true;
        var first = true;
        function step() {{
          text = text.slice(0, -1);
          el.textContent = text;
          if (text.length > 0) {{
            if (first) {{
              first = false;
              setTimeout(step, 300);
            }} else {{
              setTimeout(step, 20 + Math.random() * 25);
            }}
          }} else {{
            typing = false;
            cb();
          }}
        }}
        setTimeout(step, 20);
      }}

      function typeWord(word, cb) {{
        var i = 0;
        typing = true;
        function step() {{
          i++;
          el.textContent = word.slice(0, i);
          if (i < word.length) {{
            setTimeout(step, jitter());
          }} else {{
            typing = false;
            if (cb) cb();
          }}
        }}
        setTimeout(step, jitter());
      }}

      function cycle() {{
        if (input.value || typing) return;
        deleteWord(function() {{
          setTimeout(function() {{
            var next = idx;
            while (next === idx) next = Math.floor(Math.random() * words.length);
            idx = next;
            typeWord(words[idx]);
          }}, reducedMotion ? 0 : 200);
        }});
      }}

      setInterval(cycle, 5000);
    }})();
  </script>
  <script>
    /* Theme toggle */
    (function() {{
      var btn = document.getElementById('theme-toggle');
      if (!btn) return;
      var root = document.documentElement;
      var mq = window.matchMedia('(prefers-color-scheme: dark)');
      function currentMode() {{
        var t = root.getAttribute('data-theme');
        if (t === 'dark' || t === 'light') return t;
        return mq.matches ? 'dark' : 'light';
      }}
      btn.addEventListener('click', function() {{
        var next = currentMode() === 'dark' ? 'light' : 'dark';
        root.setAttribute('data-theme', next);
        root.style.background = next === 'dark' ? '#1C1C20' : '#F4F4F5';
        localStorage.setItem('ds-theme', next);
      }});
    }})();
  </script>
</body>
</html>"#,
        escaped_title = escaped_title,
        footer = if show_footer {
            footer::render()
        } else {
            String::new()
        },
        body_content = body_content,
    )
}

#[must_use]
fn escape_html_text(text: &str) -> String {
    let mut escaped = String::with_capacity(text.len());
    for ch in text.chars() {
        match ch {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&#x27;"),
            _ => escaped.push(ch),
        }
    }
    escaped
}

#[cfg(test)]
mod tests {
    use super::*;

    // r[verify frontend.rendering.html-crate]
    // r[verify frontend.styling.tailwind]
    // r[verify frontend.styling.light-theme]
    // r[verify frontend.styling.accent-color]
    // r[verify frontend.styling.responsive]
    #[test]
    fn document_includes_expected_rendering_and_styling_primitives() {
        let html = document("Home", "<p>Body</p>");
        assert!(html.contains("<html lang=\"en\""));
        assert!(html.contains("https://cdn.tailwindcss.com"));
        assert!(html.contains(ACCENT_COLOR));
        assert!(html.contains("<meta name=\"viewport\""));
        assert!(html.contains("bg-canvas text-ink-900"));
        assert!(html.contains("html, body"));
        assert!(html.contains("background-color: var(--c-canvas);"));
        assert!(html.contains("color: var(--c-ink-900);"));
        // Dark mode infrastructure
        assert!(html.contains("prefers-color-scheme: dark"));
        assert!(html.contains("data-theme"));
        assert!(html.contains("theme-toggle"));
    }
}
