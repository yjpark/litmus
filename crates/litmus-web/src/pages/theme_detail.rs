use dioxus::prelude::*;

use crate::components::*;
use crate::scene_renderer;
use crate::state::*;
use crate::themes;
use crate::Route;

static ANSI_NAMES: &[&str] = &[
    "black", "red", "green", "yellow", "blue", "magenta", "cyan", "white",
    "bright black", "bright red", "bright green", "bright yellow",
    "bright blue", "bright magenta", "bright cyan", "bright white",
];

/// Single theme detail page with scene navigation via sidebar's ActiveScene.
#[component]
pub fn ThemeDetail(slug: String) -> Element {
    let all_themes = themes::load_embedded_themes();
    let theme = all_themes.iter().find(|t| theme_slug(&t.name) == slug);
    let mut palette_expanded = use_signal(|| false);
    let mut issues_expanded = use_signal(|| false);
    let filter = use_context::<Signal<FilterState>>();
    let active_scene = use_context::<Signal<ActiveScene>>();

    match theme {
        Some(theme) => {
            let cvd = filter.read().cvd;
            let base_theme = theme.clone();
            let theme = maybe_simulate(&base_theme, cvd);
            let bg = theme.background.to_hex();
            let fg = theme.foreground.to_hex();
            let this_slug = theme_slug(&theme.name);
            let scenes = litmus_model::scenes::all_scenes();
            let scene_idx = active_scene.read().0.unwrap_or(0);
            let tab_idx = scene_idx.min(scenes.len().saturating_sub(1));
            let expanded = *palette_expanded.read();
            let issues_open = *issues_expanded.read();

            let issues = litmus_model::contrast::validate_theme_readability(&theme);
            let issue_count = issues.len();
            let fg_bg_ratio = litmus_model::contrast::contrast_ratio(
                &theme.foreground, &theme.background,
            );
            let readability = litmus_model::contrast::readability_score(&theme) as u8;

            let scene_count = scenes.len();
            let mut compare_sel = use_context::<Signal<CompareSelection>>();
            let detail_slug = this_slug.clone();
            let mut active_scene_write = active_scene;

            // Group issues by scene for the expandable list
            let mut issues_by_scene: Vec<(String, Vec<&litmus_model::contrast::ContrastIssue>)> = Vec::new();
            for issue in &issues {
                if let Some(group) = issues_by_scene.iter_mut().find(|(id, _)| id == &issue.scene_id) {
                    group.1.push(issue);
                } else {
                    issues_by_scene.push((issue.scene_id.clone(), vec![issue]));
                }
            }

            rsx! {
                div {
                    class: "page-theme-detail",
                    tabindex: "0",
                    autofocus: true,
                    onkeydown: move |evt: Event<KeyboardData>| {
                        match evt.key() {
                            Key::ArrowLeft => {
                                if tab_idx > 0 {
                                    active_scene_write.set(ActiveScene(Some(tab_idx - 1)));
                                }
                            }
                            Key::ArrowRight => {
                                if tab_idx + 1 < scene_count {
                                    active_scene_write.set(ActiveScene(Some(tab_idx + 1)));
                                }
                            }
                            Key::Character(ref c) if c == "c" => {
                                let mut sel = compare_sel.write();
                                if let Some(pos) = sel.0.iter().position(|s| s == &detail_slug) {
                                    sel.0.remove(pos);
                                } else if sel.0.len() < MAX_COMPARE {
                                    sel.0.push(detail_slug.clone());
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
                        CompareToggle { slug: this_slug, name: theme.name.clone() }
                    }

                    // Expandable contrast issues
                    if issues_open && issue_count > 0 {
                        div { class: "contrast-issues-list",
                            for (scene_id, scene_issues) in &issues_by_scene {
                                div { class: "contrast-issue-group",
                                    div { class: "contrast-issue-scene mono", "{scene_id}:" }
                                    for issue in scene_issues {
                                        div { class: "contrast-issue-item mono",
                                            span { class: "contrast-issue-text", "\"{issue.text}\"" }
                                            " \u{2014} fg "
                                            span {
                                                class: "color-chip",
                                                style: "background: {issue.fg.to_hex()};",
                                            }
                                            span { " {issue.fg.to_hex()} on bg " }
                                            span {
                                                class: "color-chip",
                                                style: "background: {issue.bg.to_hex()};",
                                            }
                                            span { " {issue.bg.to_hex()} \u{2014} {issue.ratio:.1}:1 (need {issue.threshold:.1}:1)" }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Scene tabs (at top, before palette)
                    div { class: "scene-nav",
                        div { class: "scene-tabs", role: "tablist",
                            for (i, scene) in scenes.iter().enumerate() {
                                button {
                                    class: if i == tab_idx { "scene-tab scene-tab-active" } else { "scene-tab" },
                                    role: "tab",
                                    aria_selected: if i == tab_idx { "true" } else { "false" },
                                    onclick: move |_| active_scene_write.set(ActiveScene(Some(i))),
                                    "{scene.name}"
                                }
                            }
                        }
                        span { class: "mono scene-hint", "\u{2190} \u{2192} navigate \u{00B7} c compare" }
                    }

                    // Active scene
                    if let Some(scene) = scenes.get(tab_idx) {
                        div { role: "tabpanel",
                            scene_renderer::SceneView {
                                theme: theme.clone(),
                                scene: scene.clone(),
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
