//! Interface detail page.

use crate::wit_doc::{FunctionDoc, InterfaceDoc, TypeDoc, TypeKind, WitDocument};
use html::text_content::{Division, ListItem, UnorderedList};
use wasm_meta_registry_client::{KnownPackage, PackageVersion};

use super::package_shell;

/// Render the interface detail page.
#[must_use]
pub(crate) fn render(
    pkg: &KnownPackage,
    version: &str,
    version_detail: Option<&PackageVersion>,
    iface: &InterfaceDoc,
    _doc: &WitDocument,
) -> String {
    let display_name = package_shell::display_name_for(pkg);
    let title = format!("{display_name} — {}", iface.name);

    // Interface content
    let mut outer = Division::builder();

    // Full interface definition code block
    outer.push(render_interface_definition(iface));

    // Grouped type and function sections
    let mut content = Division::builder();
    content.class("space-y-8");
    let resources: Vec<&TypeDoc> = iface
        .types
        .iter()
        .filter(|t| matches!(t.kind, TypeKind::Resource { .. }))
        .collect();
    let records: Vec<&TypeDoc> = iface
        .types
        .iter()
        .filter(|t| matches!(t.kind, TypeKind::Record { .. }))
        .collect();
    let variants: Vec<&TypeDoc> = iface
        .types
        .iter()
        .filter(|t| matches!(t.kind, TypeKind::Variant { .. }))
        .collect();
    let enums: Vec<&TypeDoc> = iface
        .types
        .iter()
        .filter(|t| matches!(t.kind, TypeKind::Enum { .. }))
        .collect();
    let flags: Vec<&TypeDoc> = iface
        .types
        .iter()
        .filter(|t| matches!(t.kind, TypeKind::Flags { .. }))
        .collect();
    let aliases: Vec<&TypeDoc> = iface
        .types
        .iter()
        .filter(|t| matches!(t.kind, TypeKind::Alias(_)))
        .collect();

    if !resources.is_empty() {
        content.push(render_type_section("Resources", &resources));
    }
    if !records.is_empty() {
        content.push(render_type_section("Records", &records));
    }
    if !variants.is_empty() {
        content.push(render_type_section("Variants", &variants));
    }
    if !enums.is_empty() {
        content.push(render_type_section("Enums", &enums));
    }
    if !flags.is_empty() {
        content.push(render_type_section("Flags", &flags));
    }
    if !aliases.is_empty() {
        content.push(render_type_section("Type Aliases", &aliases));
    }
    if !iface.functions.is_empty() {
        content.push(render_function_section(&iface.functions));
    }

    outer.push(content.build());

    let ctx = package_shell::SidebarContext {
        pkg,
        version,
        version_detail,
        importers: &[],
        exporters: &[],
        description_override: Some(iface.docs.as_deref().unwrap_or("")),
    };
    let extra = vec![crate::nav::Crumb {
        label: iface.name.clone(),
        href: None,
    }];
    package_shell::render_page_with_crumbs(&ctx, &title, outer.build(), extra)
}

/// Render a section of types grouped by kind.
fn render_type_section(heading: &str, types: &[&TypeDoc]) -> Division {
    let mut div = Division::builder();
    div.class("pt-6 border-t-2 border-fg first:pt-0 first:border-0");
    div.heading_2(|h2| {
        h2.class("text-sm font-medium text-fg-muted uppercase tracking-wide mb-3 pb-2 border-b-2 border-fg")
            .text(heading.to_owned())
    });

    let mut ul = UnorderedList::builder();
    ul.class("space-y-0.5");
    for ty in types {
        ul.push(render_type_row(ty));
    }
    div.push(ul.build());
    div.build()
}

/// Render a single type row in docs.rs style: linked name + doc excerpt.
fn render_type_row(ty: &TypeDoc) -> ListItem {
    let color_class = kind_color_class(&ty.kind);

    let mut li = ListItem::builder();
    li.class("py-3 flex gap-6");

    // Left: kind-colored name
    li.division(|left| {
        left.class("shrink-0 w-52").anchor(|a| {
            a.href(ty.url.clone())
                .class(format!(
                    "font-mono text-sm font-medium hover:underline {color_class}"
                ))
                .text(ty.name.clone())
        })
    });

    // Right: doc excerpt
    if let Some(docs) = &ty.docs {
        li.division(|right| {
            right
                .class("text-sm leading-relaxed text-fg-secondary line-clamp-2 min-w-0")
                .text(first_sentence(docs))
        });
    }

    li.build()
}

/// Render the freestanding functions section.
fn render_function_section(functions: &[FunctionDoc]) -> Division {
    let mut div = Division::builder();
    div.class("pt-6 border-t-2 border-fg first:pt-0 first:border-0");
    div.heading_2(|h2| {
        h2.class("text-sm font-medium text-fg-muted uppercase tracking-wide mb-3 pb-2 border-b-2 border-fg")
            .text("Functions")
    });

    let mut ul = UnorderedList::builder();
    ul.class("space-y-0.5");
    for func in functions {
        ul.push(render_function_row(func));
    }
    div.push(ul.build());
    div.build()
}

/// Render a single function row: linked name + doc excerpt.
fn render_function_row(func: &FunctionDoc) -> ListItem {
    // Color for functions: use a teal/cyan hue
    let color_class = "text-wit-func";

    let mut li = ListItem::builder();
    li.class("py-3 flex gap-6");

    // Left: function name
    li.division(|left| {
        left.class("shrink-0 w-52").anchor(|a| {
            a.href(func.url.clone())
                .class(format!(
                    "font-mono text-sm font-medium hover:underline {color_class}"
                ))
                .text(func.name.clone())
        })
    });

    // Right: doc excerpt
    if let Some(docs) = &func.docs {
        li.division(|right| {
            right
                .class("text-sm leading-relaxed text-fg-secondary line-clamp-2 min-w-0")
                .text(first_sentence(docs))
        });
    }

    li.build()
}

/// Get the CSS color class for a type kind.
///
/// Palette (OKLCH-based, same hue family as the design system):
/// - Records/Variants: blue-violet (hue 260) — structural data types
/// - Enums/Flags: teal (hue 180) — enumerable values
/// - Resources: amber (hue 70) — managed handles
/// - Aliases: default accent — pass-through types
/// - Functions: indigo (hue 240) — callable items
fn kind_color_class(kind: &TypeKind) -> &'static str {
    match kind {
        TypeKind::Record { .. } | TypeKind::Variant { .. } => "text-wit-struct",
        TypeKind::Enum { .. } | TypeKind::Flags { .. } => "text-wit-enum",
        TypeKind::Resource { .. } => "text-wit-resource",
        TypeKind::Alias(_) => "text-accent",
    }
}

/// Extract the first sentence from a doc comment.
fn first_sentence(text: &str) -> String {
    text.split_once(". ")
        .map_or_else(|| text.to_owned(), |(first, _)| format!("{first}."))
}

/// Render the full interface definition as a WIT code block.
fn render_interface_definition(iface: &InterfaceDoc) -> Division {
    let pre_class = "border-2 border-fg px-4 py-3 text-sm font-mono text-fg overflow-x-auto";

    let mut lines = Vec::new();

    // Types
    for ty in &iface.types {
        match &ty.kind {
            TypeKind::Record { fields } => {
                lines.push(format!("  record {} {{", ty.name));
                for f in fields {
                    lines.push(format!("    {}: {},", f.name, format_type_ref_short(&f.ty)));
                }
                lines.push("  }".to_owned());
            }
            TypeKind::Variant { cases } => {
                lines.push(format!("  variant {} {{", ty.name));
                for c in cases {
                    if let Some(t) = &c.ty {
                        lines.push(format!("    {}({}),", c.name, format_type_ref_short(t)));
                    } else {
                        lines.push(format!("    {},", c.name));
                    }
                }
                lines.push("  }".to_owned());
            }
            TypeKind::Enum { cases } => {
                lines.push(format!("  enum {} {{", ty.name));
                for c in cases {
                    lines.push(format!("    {},", c.name));
                }
                lines.push("  }".to_owned());
            }
            TypeKind::Flags { flags } => {
                lines.push(format!("  flags {} {{", ty.name));
                for f in flags {
                    lines.push(format!("    {},", f.name));
                }
                lines.push("  }".to_owned());
            }
            TypeKind::Resource { .. } => {
                lines.push(format!("  resource {};", ty.name));
            }
            TypeKind::Alias(type_ref) => {
                lines.push(format!(
                    "  type {} = {};",
                    ty.name,
                    format_type_ref_short(type_ref)
                ));
            }
        }
        lines.push(String::new());
    }

    // Functions
    for func in &iface.functions {
        let params: Vec<String> = func
            .params
            .iter()
            .filter(|p| p.name != "self")
            .map(|p| format!("{}: {}", p.name, format_type_ref_short(&p.ty)))
            .collect();
        let ret = func
            .result
            .as_ref()
            .map(|r| format!(" -> {}", format_type_ref_short(r)))
            .unwrap_or_default();
        lines.push(format!(
            "  {}: func({}){};",
            func.name,
            params.join(", "),
            ret
        ));
    }

    let body = lines.join("\n");
    let wit_text = format!("interface {} {{\n{}\n}}", iface.name, body);

    Division::builder()
        .class("mb-8")
        .push(
            html::text_content::PreformattedText::builder()
                .class(pre_class)
                .code(|c| c.text(wit_text))
                .build(),
        )
        .build()
}

/// Format a type ref as a short string for code block display.
fn format_type_ref_short(ty: &crate::wit_doc::TypeRef) -> String {
    use crate::wit_doc::TypeRef;
    match ty {
        TypeRef::Primitive { name } | TypeRef::Named { name, .. } => name.clone(),
        TypeRef::List { ty } => format!("list<{}>", format_type_ref_short(ty)),
        TypeRef::Option { ty } => format!("option<{}>", format_type_ref_short(ty)),
        TypeRef::Result { ok, err } => {
            let ok_s = ok
                .as_ref()
                .map_or("_".to_owned(), |t| format_type_ref_short(t));
            let err_s = err
                .as_ref()
                .map_or("_".to_owned(), |t| format_type_ref_short(t));
            format!("result<{ok_s}, {err_s}>")
        }
        TypeRef::Tuple { types } => {
            let inner: Vec<String> = types.iter().map(format_type_ref_short).collect();
            format!("tuple<{}>", inner.join(", "))
        }
        TypeRef::Handle {
            resource_name,
            handle_kind,
            ..
        } => match handle_kind {
            crate::wit_doc::HandleKind::Own => resource_name.clone(),
            crate::wit_doc::HandleKind::Borrow => format!("borrow<{resource_name}>"),
        },
        TypeRef::Future { ty } => {
            let inner = ty
                .as_ref()
                .map_or("_".to_owned(), |t| format_type_ref_short(t));
            format!("future<{inner}>")
        }
        TypeRef::Stream { ty } => {
            let inner = ty
                .as_ref()
                .map_or("_".to_owned(), |t| format_type_ref_short(t));
            format!("stream<{inner}>")
        }
    }
}
