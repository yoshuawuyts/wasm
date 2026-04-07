---
name: html-crate
description: "Generate type-safe HTML in Rust using the `html` crate's builder pattern. Use when: writing HTML templates, building pages, rendering components, creating server-side HTML, using the html crate, generating HTML from Rust code."
---

# Writing HTML with the `html` Crate

Generate type-safe, spec-compliant HTML in Rust using the [`html`](https://docs.rs/html/latest/html/) crate (v0.6). The crate models the full HTML spec via Rust's type system — invalid nesting is caught at compile time.

## When to Use

- Building server-side rendered HTML pages or components
- Generating HTML fragments from data
- Any Rust code that needs to produce HTML output

## Recursion Limit

Deep element nesting requires raising the recursion limit. Add this at the crate root:

```rust
#![recursion_limit = "512"]
```

## Module Layout

Elements are organized by HTML spec category:

| Module               | Elements                                                                                                                         |
| -------------------- | -------------------------------------------------------------------------------------------------------------------------------- |
| `html::content`      | `Section`, `Article`, `Aside`, `Header`, `Footer`, `Navigation`, `Main`, `Heading1`–`Heading6`                                   |
| `html::text_content` | `Division`, `Paragraph`, `UnorderedList`, `OrderedList`, `ListItem`, `BlockQuote`, `PreformattedText`, `Figure`, `FigureCaption` |
| `html::inline_text`  | `Anchor`, `Span`, `Strong`, `Emphasis`, `Code`, `Bold`, `Italic`, `LineBreak`                                                    |
| `html::forms`        | `Form`, `Input`, `Button`, `Label`, `Select`, `TextArea`                                                                         |
| `html::tables`       | `Table`, `TableRow`, `TableCell`, `TableHeader`, `Caption`                                                                       |
| `html::media`        | `Image`, `Audio`, `Video`                                                                                                        |
| `html::embedded`     | `IFrame`, `Object`                                                                                                               |
| `html::root`         | `Body`, `Html`                                                                                                                   |
| `html::metadata`     | `Head`, `Title`, `Meta`, `Link`                                                                                                  |

## Core Pattern: Builder API

Every element follows the same pattern:

```rust
use html::text_content::Paragraph;

let p = Paragraph::builder()
    .text("Hello, world!")
    .class("greeting")
    .build();

// Render to string
let html_string = p.to_string();
// => <p class="greeting">Hello, world!</p>
```

### Builder Methods

**Content:**
- `.text("content")` — add text content
- `.class("css-classes")` — set the `class` attribute (space-separated)
- `.id("my-id")` — set the `id` attribute

**All HTML attributes** are available as builder methods using their Rust name:
- `.href("/path")` — for `<a>`
- `.src("/image.png")` — for `<img>`
- `.type_("submit")` — for `<input>` (trailing `_` for Rust keywords)
- `.role("navigation")` — ARIA role
- `.aria_label("Close")` — ARIA attributes use underscores

## Nesting Elements

### Via Closures (Preferred)

Parent builders expose child element methods that take a closure:

```rust
use html::text_content::OrderedList;

let list = OrderedList::builder()
    .list_item(|li| li.text("First").class("item"))
    .list_item(|li| li.text("Second").class("item"))
    .build();
```

The closure receives the child's builder, and you chain methods on it. No `.build()` call inside the closure — the parent handles it.

Child method names match the element name in snake_case:
- `.division(|div| ...)` — adds a `<div>`
- `.paragraph(|p| ...)` — adds a `<p>`
- `.anchor(|a| ...)` — adds an `<a>`
- `.heading1(|h1| ...)` — adds an `<h1>`
- `.span(|s| ...)` — adds a `<span>`
- `.list_item(|li| ...)` — adds an `<li>`
- `.section(|s| ...)` — adds a `<section>`
- `.unordered_list(|ul| ...)` — adds a `<ul>`

### Via Push (For Dynamic Content)

Build elements separately and push them:

```rust
use html::text_content::{UnorderedList, ListItem};

let mut ul = UnorderedList::builder();
for name in &["Alice", "Bob", "Carol"] {
    let li = ListItem::builder().text(*name).build();
    ul.push(li);
}
let list = ul.build();
```

### Combining Both

```rust
use html::text_content::Division;

let page = Division::builder()
    .class("container")
    .heading1(|h1| h1.text("Title"))
    .paragraph(|p| p.text("Introduction paragraph."))
    .division(|inner| {
        inner
            .class("content")
            .paragraph(|p| p.text("Nested content"))
    })
    .build();
```

## Rendering

All elements implement `Display` — use `.to_string()` to render:

```rust
let html_string = element.to_string();
```

## Common Recipes

### Link with Text

```rust
use html::inline_text::Anchor;

let link = Anchor::builder()
    .href("/about")
    .class("nav-link")
    .text("About Us")
    .build();
```

### Navigation with Links

```rust
use html::content::Navigation;

let nav = Navigation::builder()
    .class("main-nav")
    .anchor(|a| a.href("/").text("Home"))
    .anchor(|a| a.href("/about").text("About"))
    .build();
```

### Card Component

```rust
use html::text_content::Division;

fn card(title: &str, body: &str) -> Division {
    Division::builder()
        .class("card rounded shadow p-4")
        .heading3(|h3| h3.text(title).class("font-bold"))
        .paragraph(|p| p.text(body).class("text-gray-600"))
        .build()
}
```

### List from Iterator

```rust
use html::text_content::{UnorderedList, ListItem};

fn tag_list(tags: &[String]) -> UnorderedList {
    let mut ul = UnorderedList::builder();
    ul.class("flex gap-2");
    for tag in tags {
        let li = ListItem::builder()
            .text(tag.as_str())
            .class("px-2 py-1 bg-gray-100 rounded")
            .build();
        ul.push(li);
    }
    ul.build()
}
```

### Full Page Document

```rust
use html::root::{Html, Body};
use html::metadata::{Head, Title, Meta};

let head = Head::builder()
    .meta(|m| m.charset("utf-8"))
    .title(|t| t.text("My Page"))
    .build();

let body = Body::builder()
    .heading1(|h1| h1.text("Welcome"))
    .paragraph(|p| p.text("Hello, world!"))
    .build();

let doc = Html::builder()
    .lang("en")
    .push(head)
    .push(body)
    .build();

let html = format!("<!DOCTYPE html>\n{doc}");
```

## Type Safety

The compiler enforces valid HTML nesting. These won't compile:

- Putting a `<div>` inside a `<span>` (block inside inline)
- Putting a `<li>` directly inside a `<div>` (must be in `<ul>` or `<ol>`)
- Nesting `<a>` inside `<a>` (interactive inside interactive)

If you get a type error about `From<X> for YChild`, it means that child element isn't valid inside the parent per the HTML spec.

## Gotchas

1. **Recursion limit**: Deep nesting hits Rust's default limit. Always set `#![recursion_limit = "512"]`.
2. **Keyword escaping**: HTML attributes matching Rust keywords get a trailing underscore: `type_()`, `for_()`, `is_()`.
3. **Class is a single string**: Pass all CSS classes space-separated in one `.class("flex gap-4 p-2")` call.
4. **No raw HTML injection**: The crate escapes text content. To include raw HTML (e.g., SVG), you'll need to use raw string rendering outside the builder or work at the `Display`/`Render` level.
5. **Builder lifetimes**: Builders borrow string data as `Cow<'static, str>` — string literals work directly, but dynamic strings need `.to_string()` or `.to_owned()`.
