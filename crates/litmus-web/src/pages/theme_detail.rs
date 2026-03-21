use dioxus::prelude::*;

use crate::components::*;
use crate::scene_renderer::{self, SpanIssueDetail};
use crate::state::*;
use crate::themes;
use crate::Route;

static ANSI_NAMES: &[&str] = &[
    "black", "red", "green", "yellow", "blue", "magenta", "cyan", "white",
    "bright black", "bright red", "bright green", "bright yellow",
    "bright blue", "bright magenta", "bright cyan", "bright white",
];

/// Single theme detail page — all scenes rendered vertically with minimap.
#[component]
pub fn ThemeDetail(slug: String) -> Element {
    let all_themes = themes::load_embedded_themes();
    let theme = all_themes.iter().find(|t| theme_slug(&t.name) == slug);
    let mut palette_expanded = use_signal(|| false);
    let mut issues_expanded = use_signal(|| false);
    let cvd_sim = use_context::<Signal<CvdSimulation>>();

    match theme {
        Some(theme) => {
            let cvd = cvd_sim.read().0;
            let base_theme = theme.clone();
            let theme = maybe_simulate(&base_theme, cvd);
            let bg = theme.background.to_hex();
            let fg = theme.foreground.to_hex();
            let this_slug = theme_slug(&theme.name);
            let scenes = litmus_model::scenes::all_scenes();
            let expanded = *palette_expanded.read();
            let issues_open = *issues_expanded.read();

            let issues = litmus_model::contrast::validate_theme_readability(&theme);
            let issue_count = issues.len();
            let fg_bg_ratio = litmus_model::contrast::contrast_ratio(
                &theme.foreground, &theme.background,
            );
            let readability = litmus_model::contrast::readability_score(&theme) as u8;

            let mut shortlist = use_context::<Signal<Shortlist>>();
            let app_theme = use_context::<Signal<AppThemeSlug>>();
            let is_current_theme = app_theme.read().0.as_deref() == Some(this_slug.as_str());
            let detail_slug = this_slug.clone();

            // Count issues per scene
            let mut issues_per_scene: std::collections::HashMap<&str, Vec<(usize, usize, SpanIssueDetail)>> = std::collections::HashMap::new();
            for issue in &issues {
                issues_per_scene.entry(issue.scene_id.as_str()).or_default().push(
                    (issue.line, issue.span, SpanIssueDetail {
                        ratio: issue.ratio,
                        threshold: issue.threshold,
                        level: issue.level.to_string(),
                        fg_hex: issue.fg.to_hex(),
                        bg_hex: issue.bg.to_hex(),
                    })
                );
            }

            // Group issues by (scene_id, line_idx) for the expanded issues list
            #[allow(clippy::type_complexity)]
            let mut issues_by_scene_line: Vec<(String, String, Vec<(usize, Vec<(usize, SpanIssueDetail)>)>)> = Vec::new();
            for issue in &issues {
                let scene_name = scenes.iter()
                    .find(|s| s.id == issue.scene_id)
                    .map(|s| s.name.clone())
                    .unwrap_or_else(|| issue.scene_id.clone());
                let detail = SpanIssueDetail {
                    ratio: issue.ratio,
                    threshold: issue.threshold,
                    level: issue.level.to_string(),
                    fg_hex: issue.fg.to_hex(),
                    bg_hex: issue.bg.to_hex(),
                };

                if let Some(scene_group) = issues_by_scene_line.iter_mut().find(|(id, _, _)| id == &issue.scene_id) {
                    if let Some(line_group) = scene_group.2.iter_mut().find(|(li, _)| *li == issue.line) {
                        line_group.1.push((issue.span, detail));
                    } else {
                        scene_group.2.push((issue.line, vec![(issue.span, detail)]));
                    }
                } else {
                    issues_by_scene_line.push((
                        issue.scene_id.clone(),
                        scene_name,
                        vec![(issue.line, vec![(issue.span, detail)])],
                    ));
                }
            }

            rsx! {
                div {
                    class: "page-theme-detail",
                    tabindex: "0",
                    autofocus: true,
                    onkeydown: move |evt: Event<KeyboardData>| {
                        match evt.key() {
                            Key::Character(ref c) if c == "c" => {
                                if !is_current_theme {
                                    let mut sel = shortlist.write();
                                    if let Some(pos) = sel.0.iter().position(|s| s == &detail_slug) {
                                        sel.0.remove(pos);
                                    } else if sel.0.len() < MAX_SHORTLIST {
                                        sel.0.push(detail_slug.clone());
                                    }
                                }
                            }
                            _ => {}
                        }
                    },

                    // Theme header with inline metadata
                    div { class: "detail-header",
                        h2 { class: "page-title", "{theme.name}" }
                        span { class: "mono detail-ratio",
                            if fg_bg_ratio >= litmus_model::contrast::WCAG_AA_NORMAL {
                                span { class: "text-success", "{fg_bg_ratio:.1}:1" }
                            } else {
                                span { class: "text-error", "{fg_bg_ratio:.1}:1" }
                            }
                        }
                        span { class: "detail-readability mono", "readability: {readability}%" }
                        if issue_count > 0 {
                            button {
                                class: "detail-issues-toggle text-error",
                                onclick: move |_| issues_expanded.set(!issues_open),
                                if issues_open {
                                    "{issue_count} contrast issue(s) \u{25BC}"
                                } else {
                                    "{issue_count} contrast issue(s) \u{25B6}"
                                }
                            }
                        }
                        ShortlistCheckbox { slug: this_slug.clone(), name: theme.name.clone() }
                        UseAsAppThemeButton { slug: this_slug }
                    }

                    // Expandable contrast issues — rendered as actual scene lines
                    if issues_open && issue_count > 0 {
                        div { class: "contrast-issues-list",
                            for (scene_id, scene_name, line_groups) in &issues_by_scene_line {
                                {
                                    let scene_obj = scenes.iter().find(|s| s.id == *scene_id);
                                    let scroll_id = scene_id.clone();
                                    rsx! {
                                        div { class: "contrast-issue-group",
                                            button {
                                                class: "contrast-issue-scene mono",
                                                onclick: move |_| {
                                                    let js = format!(
                                                        "document.getElementById('scene-{}').scrollIntoView({{behavior:'smooth',block:'start'}})",
                                                        scroll_id
                                                    );
                                                    dioxus::document::eval(&js);
                                                },
                                                "{scene_name} \u{2192}"
                                            }
                                            for (line_idx, span_details) in line_groups.iter() {
                                                {
                                                    let issue_details_for_line: Vec<(usize, usize, SpanIssueDetail)> = span_details.iter()
                                                        .map(|(si, d)| (*line_idx, *si, d.clone()))
                                                        .collect();
                                                    let line = scene_obj
                                                        .and_then(|s| s.lines.get(*line_idx))
                                                        .cloned();
                                                    rsx! {
                                                        if let Some(line) = line {
                                                            div { class: "contrast-issue-line",
                                                                pre {
                                                                    style: "background-color: {bg}; color: {fg};",
                                                                    scene_renderer::LineView {
                                                                        theme: theme.clone(),
                                                                        line: line,
                                                                        line_idx: *line_idx,
                                                                        issue_details: issue_details_for_line,
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // All scenes rendered vertically
                    for scene in &scenes {
                        {
                            let scene_issues = issues_per_scene
                                .get(scene.id.as_str())
                                .cloned()
                                .unwrap_or_default();
                            let issue_count = scene_issues.len();
                            rsx! {
                                div {
                                    class: "detail-scene-section",
                                    id: "scene-{scene.id}",
                                    h3 { class: "detail-scene-heading",
                                        "{scene.name}"
                                        if issue_count > 0 {
                                            span { class: "scene-tab-badge", "{issue_count}" }
                                        }
                                    }
                                    scene_renderer::SceneView {
                                        theme: theme.clone(),
                                        scene: scene.clone(),
                                        issue_details: scene_issues,
                                    }
                                }
                            }
                        }
                    }

                    // Compact color palette (expandable)
                    div {
                        class: "color-palette",
                        style: "background: {bg}; color: {fg};",

                        div {
                            class: "palette-compact",
                            onclick: move |_| palette_expanded.set(!expanded),

                            ColorSwatch { label: "bg", color: theme.background.to_hex() }
                            ColorSwatch { label: "fg", color: theme.foreground.to_hex() }
                            ColorSwatch { label: "cur", color: theme.cursor.to_hex() }

                            span { class: "palette-divider", "|" }

                            div { class: "swatch-row",
                                for color in theme.ansi.as_array().iter() {
                                    div {
                                        class: "swatch",
                                        style: "background: {color.to_hex()};",
                                        title: "{color.to_hex()}",
                                    }
                                }
                            }

                            span { class: "mono palette-toggle",
                                if expanded { "collapse" } else { "expand" }
                            }
                        }

                        if expanded {
                            div { class: "palette-expanded",
                                div { class: "special-colors",
                                    ColorSwatch { label: "bg", color: theme.background.to_hex() }
                                    ColorSwatch { label: "fg", color: theme.foreground.to_hex() }
                                    ColorSwatch { label: "cursor", color: theme.cursor.to_hex() }
                                    ColorSwatch { label: "sel bg", color: theme.selection_background.to_hex() }
                                    ColorSwatch { label: "sel fg", color: theme.selection_foreground.to_hex() }
                                }

                                div { class: "ansi-grid",
                                    for (i, color) in theme.ansi.as_array().iter().enumerate() {
                                        div { class: "ansi-cell",
                                            div {
                                                class: "swatch-lg mono",
                                                style: "background: {color.to_hex()}; color: {fg};",
                                                title: "{color.to_hex()}",
                                                "{i}"
                                            }
                                            div { class: "mono ansi-name", "{ANSI_NAMES[i]}" }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    ExportButtons { theme: theme.clone() }
                }

                SceneMinimap { scenes: scenes.clone() }
            }
        }
        None => {
            rsx! {
                div {
                    h2 { "Theme not found" }
                    p { "No theme matches \"{slug}\"." }
                    Link { to: Route::ThemeList {}, class: "accent-link", "Back to all themes" }
                }
            }
        }
    }
}
