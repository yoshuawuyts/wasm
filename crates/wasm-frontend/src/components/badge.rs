//! Badge components.
//!
//! Status badges with categorical color pairs, count badges, and tags.

use html::inline_text::Span;

/// Categorical color variant for badges.
pub(crate) enum BadgeColor {
    Green,
    Cream,
    Pink,
    Blue,
    Muted,
}

/// Render a status badge with a dot indicator.
pub(crate) fn status(label: &str, color: BadgeColor) -> Span {
    let classes = match color {
        BadgeColor::Green => {
            "inline-flex items-center gap-1.5 px-2 h-6 rounded-pill bg-cat-green text-cat-greenInk"
        }
        BadgeColor::Cream => {
            "inline-flex items-center gap-1.5 px-2 h-6 rounded-pill bg-cat-cream text-cat-creamInk"
        }
        BadgeColor::Pink => {
            "inline-flex items-center gap-1.5 px-2 h-6 rounded-pill bg-cat-pink text-cat-pinkInk"
        }
        BadgeColor::Blue => {
            "inline-flex items-center gap-1.5 px-2 h-6 rounded-pill bg-cat-blue text-cat-blueInk"
        }
        BadgeColor::Muted => {
            "inline-flex items-center px-2 h-6 rounded-pill bg-surfaceMuted text-ink-700"
        }
    };
    let dot_class = match color {
        BadgeColor::Green => "h-1.5 w-1.5 rounded-full bg-cat-greenInk",
        BadgeColor::Cream => "h-1.5 w-1.5 rounded-full bg-cat-creamInk",
        BadgeColor::Pink => "h-1.5 w-1.5 rounded-full bg-cat-pinkInk",
        BadgeColor::Blue => "h-1.5 w-1.5 rounded-full bg-cat-blueInk",
        BadgeColor::Muted => {
            return Span::builder()
                .class(classes)
                .text(label.to_owned())
                .build();
        }
    };
    Span::builder()
        .class(classes)
        .span(|s| s.class(dot_class))
        .text(format!(" {label}"))
        .build()
}

/// Render a count badge (small pill with number).
pub(crate) fn count(value: &str) -> Span {
    Span::builder()
        .class("inline-flex items-center px-1.5 min-w-[20px] h-5 rounded-pill bg-ink-700 text-canvas justify-center text-[12px] font-medium")
        .text(value.to_owned())
        .build()
}
