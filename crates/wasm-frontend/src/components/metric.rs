//! Metric display component.
//!
//! Caption label, large value, optional delta. Used for dashboard-style
//! key figures.

use html::text_content::Division;

/// Render a metric with label, value, and optional delta.
pub(crate) fn render(label: &str, value: &str, delta: Option<&str>) -> Division {
    let mut d = Division::builder();
    d.division(|inner| {
        inner
            .class("text-[12px] text-ink-500")
            .text(label.to_owned())
    });
    d.division(|inner| {
        let val = inner.class("mt-1 flex items-baseline gap-1.5").span(|s| {
            s.class("text-[20px] font-semibold tracking-tight leading-tight")
                .text(value.to_owned())
        });
        if let Some(delta_text) = delta {
            val.span(|s| {
                s.class("text-[11px] font-medium text-positive")
                    .text(delta_text.to_owned())
            });
        }
        val
    });
    d.build()
}
