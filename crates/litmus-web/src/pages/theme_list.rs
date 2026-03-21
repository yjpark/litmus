use dioxus::prelude::*;

use crate::components::CompareToggle;
use crate::family;
use crate::scene_renderer;
use crate::state::*;
use crate::themes;
use crate::Route;

/// Home page: theme card grid. Filters are owned by the sidebar.
#[component]
pub fn ThemeList() -> Element {
    let all_themes = themes::load_embedded_themes();
    let filter = use_context::<Signal<FilterState>>();
    let filter_val = filter.read().clone();
    let cvd = filter_val.cvd;

    let filtered: Vec<litmus_model::Theme> = all_themes
        .iter()
        .filter(|t| theme_passes_filter(t, &filter_val))
        .map(|t| maybe_simulate(t, cvd))
        .collect();
    let families = family::group_by_family(&filtered);
    let total = all_themes.len();
    let shown = filtered.len();

    rsx! {
        div { class: "page-theme-list",
            div { class: "page-header",
                h2 { class: "page-title", "Browse Themes" }
                if shown < total {
                    span { class: "filter-count", "{shown}/{total}" }
                }
            }

            for fam in &families {
                div { class: "family-group",
                    h3 { class: "family-name", "{fam.name}" }
                    div { class: "theme-grid",
                        for theme in &fam.themes {
                            ThemeCard { theme: theme.clone() }
                        }
                    }
                }
            }

            if families.is_empty() {
                p { class: "empty-state", "No themes match the current filters." }
            }
        }
    }
}

#[component]
fn ThemeCard(theme: litmus_model::Theme) -> Element {
    let bg = theme.background.to_hex();
    let fg = theme.foreground.to_hex();
    let slug = theme_slug(&theme.name);
    let is_light = litmus_model::contrast::relative_luminance(&theme.background) > 0.5;
    let variant = if is_light { "light" } else { "dark" };
    let fg_bg_ratio = litmus_model::contrast::contrast_ratio(&theme.foreground, &theme.background);
    let readability = litmus_model::contrast::readability_score(&theme) as u8;
    let preview_scene = litmus_model::scenes::shell_prompt_scene();

    rsx! {
        Link {
            to: Route::ThemeDetail { slug: slug.clone() },
            class: "theme-card-link",

            div {
                class: "theme-card",
                style: "background: {bg}; color: {fg};",

                div { class: "theme-card-header",
                    span { class: "theme-card-name", "{theme.name}" }
                    span { class: "theme-card-meta", "{variant} {fg_bg_ratio:.1}:1 readability: {readability}%" }
                }

                scene_renderer::ScenePreview {
                    theme: theme.clone(),
                    scene: preview_scene,
                    max_lines: 5,
                }

                div { class: "theme-card-footer",
                    div { class: "swatch-row",
                        for color in theme.ansi.as_array().iter() {
                            div {
                                class: "swatch",
                                style: "background: {color.to_hex()};",
                            }
                        }
                    }
                    CompareToggle { slug, name: theme.name.clone() }
                }
            }
        }
    }
}
