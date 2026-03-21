use dioxus::prelude::*;

use crate::components::*;
use crate::scene_renderer;
use crate::state::*;
use crate::themes;
use crate::Route;

/// Multi-theme comparison (2-4 themes side by side).
#[component]
pub fn CompareThemes(slugs: String) -> Element {
    let all_themes = themes::load_embedded_themes();
    let scenes = litmus_model::scenes::all_scenes();
    let slug_list: Vec<&str> = slugs.split(',').filter(|s| !s.is_empty()).collect();
    let filter = use_context::<Signal<FilterState>>();
    let cvd = filter.read().cvd;

    let compare_themes: Vec<litmus_model::Theme> = slug_list
        .iter()
        .filter_map(|slug| all_themes.iter().find(|t| theme_slug(&t.name) == *slug).cloned())
        .map(|t| maybe_simulate(&t, cvd))
        .collect();

    if compare_themes.is_empty() {
        return rsx! {
            div {
                h2 { "No themes found" }
                p { "Could not find any matching themes." }
                Link { to: Route::ThemeList {}, class: "accent-link", "Back to all themes" }
            }
        };
    }

    let n = compare_themes.len();
    let title = compare_themes
        .iter()
        .map(|t| t.name.as_str())
        .collect::<Vec<_>>()
        .join(" vs ");
    let grid_cols = format!("repeat({n}, 1fr)");

    rsx! {
        div { class: "page-compare",
            h2 { class: "page-title", "{title}" }

            CompareSelector {
                all_themes: all_themes.clone(),
                current_slugs: slug_list.iter().map(|s| s.to_string()).collect(),
            }

            if compare_themes.len() >= 2 {
                ColorDiffTable { themes: compare_themes.clone() }
            }

            for scene in &scenes {
                div { class: "compare-scene-group",
                    h3 { class: "compare-scene-name", "{scene.name}" }

                    div {
                        class: "compare-grid",
                        style: "--compare-cols: {grid_cols};",

                        for theme in &compare_themes {
                            div { class: "compare-grid-item",
                                div { class: "compare-grid-item-header",
                                    span { class: "compare-grid-item-name", "{theme.name}" }
                                    Link {
                                        to: Route::ThemeDetail {
                                            slug: theme_slug(&theme.name),
                                        },
                                        class: "text-success choose-link",
                                        "Choose"
                                    }
                                }
                                scene_renderer::SceneView {
                                    theme: theme.clone(),
                                    scene: scene.clone(),
                                    compact: n > 2,
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Dropdowns for selecting comparison themes (supports 2-4).
#[component]
fn CompareSelector(
    all_themes: Vec<litmus_model::Theme>,
    current_slugs: Vec<String>,
) -> Element {
    let nav = use_navigator();

    rsx! {
        div { class: "compare-selector",
            for (idx, slug) in current_slugs.iter().enumerate() {
                {
                    let all = all_themes.clone();
                    let slugs = current_slugs.clone();
                    let current_val = slug.clone();
                    rsx! {
                        if idx > 0 {
                            span { class: "compare-vs", "vs" }
                        }
                        select {
                            class: "compare-select",
                            value: "{current_val}",
                            onchange: move |evt: Event<FormData>| {
                                let mut new_slugs = slugs.clone();
                                new_slugs[idx] = evt.value();
                                nav.push(Route::CompareThemes {
                                    slugs: new_slugs.join(","),
                                });
                            },
                            for t in &all {
                                option {
                                    value: "{theme_slug(&t.name)}",
                                    "{t.name}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
