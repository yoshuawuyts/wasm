//! Shared sigil definitions.
//!
//! Centralises the color + letter combinations used across the sidebar,
//! item lists, and detail pages so every callsite references one constant.

/// A sigil definition: background color, text color, and letter.
pub(crate) struct Sigil {
    /// CSS background color value (e.g. `"var(--c-cat-green)"`).
    pub bg: &'static str,
    /// CSS text color value (e.g. `"var(--c-cat-green-ink)"`).
    pub color: &'static str,
    /// Single-character label (always rendered uppercase by CSS).
    pub text: &'static str,
}

/// World — green "W".
pub(crate) const WORLD: Sigil = Sigil {
    bg: "var(--c-cat-green)",
    color: "var(--c-cat-green-ink)",
    text: "W",
};

/// Interface — lilac "I".
pub(crate) const IFACE: Sigil = Sigil {
    bg: "var(--c-cat-lilac)",
    color: "var(--c-cat-lilac-ink)",
    text: "I",
};

/// Function — green "F".
pub(crate) const FUNC: Sigil = Sigil {
    bg: "var(--c-cat-green)",
    color: "var(--c-cat-green-ink)",
    text: "F",
};

/// Type (record, variant, alias, etc.) — blue "T".
pub(crate) const TYPE: Sigil = Sigil {
    bg: "var(--c-cat-blue)",
    color: "var(--c-cat-blue-ink)",
    text: "T",
};

/// Resource — peach "R".
pub(crate) const RESOURCE: Sigil = Sigil {
    bg: "var(--c-cat-peach)",
    color: "var(--c-cat-peach-ink)",
    text: "R",
};

/// Record — lilac "R".
pub(crate) const RECORD: Sigil = Sigil {
    bg: "var(--c-cat-lilac)",
    color: "var(--c-cat-lilac-ink)",
    text: "R",
};

/// Variant — lilac "V".
pub(crate) const VARIANT: Sigil = Sigil {
    bg: "var(--c-cat-lilac)",
    color: "var(--c-cat-lilac-ink)",
    text: "V",
};

/// Enum — teal "E".
pub(crate) const ENUM: Sigil = Sigil {
    bg: "var(--c-cat-teal)",
    color: "var(--c-cat-teal-ink)",
    text: "E",
};

/// Flags — teal "F".
pub(crate) const FLAGS: Sigil = Sigil {
    bg: "var(--c-cat-teal)",
    color: "var(--c-cat-teal-ink)",
    text: "F",
};

/// Package / root — slate "·".
#[allow(dead_code)]
pub(crate) const ROOT: Sigil = Sigil {
    bg: "var(--c-cat-slate)",
    color: "var(--c-cat-slate-ink)",
    text: "\u{00b7}",
};
