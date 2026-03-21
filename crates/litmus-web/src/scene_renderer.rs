use dioxus::prelude::*;
use litmus_model::scene::{Scene, SceneLine, StyledSpan};
use litmus_model::Theme;

/// Detail about a contrast issue on a specific span, used for tooltips.
#[derive(Clone, PartialEq)]
pub struct SpanIssueDetail {
    pub ratio: f64,
    pub threshold: f64,
    pub level: String,
    pub fg_hex: String,
    pub bg_hex: String,
}

/// Render a complete scene as a terminal-style HTML block.
///
/// `issue_details` contains `(line_idx, span_idx, detail)` tuples marking spans
/// with contrast issues. Those spans get a visual indicator + tooltip via CSS.
#[component]
pub fn SceneView(
    theme: Theme,
    scene: Scene,
    #[props(default = false)] compact: bool,
    #[props(default)] issue_details: Vec<(usize, usize, SpanIssueDetail)>,
) -> Element {
    let bg = theme.background.to_hex();
    let fg = theme.foreground.to_hex();
    let class = if compact { "scene-block scene-compact" } else { "scene-block" };

    rsx! {
        div { class: "{class}",
            if !compact {
                div {
                    style: "margin-bottom: 0.5rem; font-weight: bold; \
                            font-size: 0.85rem; opacity: 0.7;",
                    "{scene.name}"
                }
            }
            pre {
                style: "background-color: {bg}; color: {fg};",
                for (i, line) in scene.lines.iter().enumerate() {
                    LineView { key: "{i}", theme: theme.clone(), line: line.clone(), line_idx: i, issue_details: issue_details.clone() }
                    "\n"
                }
            }
        }
    }
}

/// Render a scene preview: first N lines only, no title, compact styling.
#[component]
pub fn ScenePreview(theme: Theme, scene: Scene, #[props(default = 5)] max_lines: usize) -> Element {
    let bg = theme.background.to_hex();
    let fg = theme.foreground.to_hex();
    let lines_to_show = scene.lines.len().min(max_lines);

    rsx! {
        div { class: "scene-preview",
            pre {
                style: "background-color: {bg}; color: {fg};",
                for (i, line) in scene.lines.iter().take(lines_to_show).enumerate() {
                    LineView { key: "{i}", theme: theme.clone(), line: line.clone() }
                    "\n"
                }
            }
        }
    }
}

/// Render a single scene line.
#[component]
pub fn LineView(theme: Theme, line: SceneLine, #[props(default)] line_idx: usize, #[props(default)] issue_details: Vec<(usize, usize, SpanIssueDetail)>) -> Element {
    if line.spans.is_empty() {
        return rsx! { "" };
    }

    rsx! {
        for (i, span) in line.spans.iter().enumerate() {
            {
                let detail = issue_details.iter()
                    .find(|(l, s, _)| *l == line_idx && *s == i)
                    .map(|(_, _, d)| d.clone());
                rsx! {
                    SpanView { key: "{i}", theme: theme.clone(), span: span.clone(), issue_detail: detail, line_idx: line_idx }
                }
            }
        }
    }
}

/// Render a single styled span as an HTML <span> with inline styles.
#[component]
fn SpanView(theme: Theme, span: StyledSpan, #[props(default)] issue_detail: Option<SpanIssueDetail>, #[props(default)] line_idx: usize) -> Element {
    let mut styles = Vec::new();

    if let Some(ref fg) = span.fg {
        let color = fg.resolve(&theme).to_hex();
        styles.push(format!("color: {color}"));
    }

    if let Some(ref bg) = span.bg {
        let color = bg.resolve(&theme).to_hex();
        styles.push(format!("background-color: {color}"));
    }

    if span.style.bold {
        styles.push("font-weight: bold".into());
    }
    if span.style.italic {
        styles.push("font-style: italic".into());
    }
    if span.style.underline {
        styles.push("text-decoration: underline".into());
    }
    if span.style.dim {
        styles.push("opacity: 0.6".into());
    }

    let style_str = styles.join("; ");
    let has_issue = issue_detail.is_some();
    let class = if has_issue { "contrast-issue-span" } else { "" };

    rsx! {
        span { class: "{class}", style: "{style_str}",
            "{span.text}"
            if let Some(d) = &issue_detail {
                {
                    let level_text = if d.level == "AA" {
                        "for normal text"
                    } else {
                        "for large/bold text"
                    };
                    let tooltip_class = if line_idx < 2 {
                        "contrast-tooltip contrast-tooltip-below"
                    } else {
                        "contrast-tooltip"
                    };
                    rsx! {
                        span { class: "{tooltip_class}",
                            span { class: "contrast-tooltip-rule",
                                "WCAG {d.level}: requires {d.threshold:.0}:1 {level_text}"
                            }
                            br {}
                            span { class: "contrast-tooltip-ratio",
                                "Actual: {d.ratio:.1}:1"
                            }
                            br {}
                            span { class: "contrast-tooltip-colors",
                                span {
                                    class: "color-chip",
                                    style: "background: {d.fg_hex};",
                                }
                                " {d.fg_hex} on "
                                span {
                                    class: "color-chip",
                                    style: "background: {d.bg_hex};",
                                }
                                " {d.bg_hex}"
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Render all scenes for a given theme.
#[component]
pub fn AllScenesView(theme: Theme) -> Element {
    let scenes = litmus_model::scenes::all_scenes();

    rsx! {
        div { class: "scenes-container",
            for scene in scenes {
                SceneView { key: "{scene.id}", theme: theme.clone(), scene: scene }
            }
        }
    }
}
