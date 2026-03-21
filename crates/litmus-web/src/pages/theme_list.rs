use dioxus::prelude::*;

use crate::components::{ShortlistToggle, UseAsAppThemeButton};
use crate::scene_renderer;
use crate::state::*;
use crate::themes;
use crate::Route;

/// Home page: theme card grid with inline filters.
#[component]
pub fn ThemeList() -> Element {
    let all_themes = themes::load_embedded_themes();
    let mut filter = use_signal(FilterState::default);
    let cvd_sim = use_context::<Signal<CvdSimulation>>();
    let cvd = cvd_sim.read().0;

    let filter_val = filter.read().clone();

    let mut filtered: Vec<litmus_model::Theme> = all_themes
        .iter()
        .filter(|t| theme_passes_filter(t, &filter_val))
        .map(|t| maybe_simulate(t, cvd))
        .collect();
    filtered.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

    let total = all_themes.len();
    let shown = filtered.len();

    // Count badges for variant filter
    let count_all = all_themes.len();
    let count_dark = all_themes.iter().filter(|t| !is_light_theme(t)).count();
    let count_light = all_themes.iter().filter(|t| is_light_theme(t)).count();

    let query = filter_val.query.clone();
    let variant = filter_val.variant;
    let min_read = filter_val.min_readability;

    rsx! {
        div { class: "page-theme-list",
            div { class: "page-header",
                h2 { class: "page-title", "Browse Themes" }
                if shown < total {
                    span { class: "filter-count", "{shown}/{total}" }
                }
            }

            // Inline filter bar
            div { class: "filter-bar",
                input {
                    class: "filter-bar-search",
                    r#type: "text",
                    placeholder: "Search themes...",
                    value: "{query}",
                    oninput: move |evt: Event<FormData>| {
                        filter.write().query = evt.value();
                    },
                }

                button {
                    class: if variant == VariantFilter::All { "filter-btn filter-btn-active" } else { "filter-btn" },
                    aria_pressed: if variant == VariantFilter::All { "true" } else { "false" },
                    onclick: move |_| filter.write().variant = VariantFilter::All,
                    "All "
                    span { class: "filter-badge", "({count_all})" }
                }
                button {
                    class: if variant == VariantFilter::Dark { "filter-btn filter-btn-active" } else { "filter-btn" },
                    aria_pressed: if variant == VariantFilter::Dark { "true" } else { "false" },
                    onclick: move |_| filter.write().variant = VariantFilter::Dark,
                    "Dark "
                    span { class: "filter-badge", "({count_dark})" }
                }
                button {
                    class: if variant == VariantFilter::Light { "filter-btn filter-btn-active" } else { "filter-btn" },
                    aria_pressed: if variant == VariantFilter::Light { "true" } else { "false" },
                    onclick: move |_| filter.write().variant = VariantFilter::Light,
                    "Light "
                    span { class: "filter-badge", "({count_light})" }
                }

                select {
                    class: "filter-bar-readability",
                    value: match min_read {
                        None => "any",
                        Some(v) => match v {
                            80 => "80",
                            90 => "90",
                            95 => "95",
                            _ => "any",
                        },
                    },
                    onchange: move |evt: Event<FormData>| {
                        let val = evt.value();
                        filter.write().min_readability = match val.as_str() {
                            "80" => Some(80),
                            "90" => Some(90),
                            "95" => Some(95),
                            _ => None,
                        };
                    },
                    option { value: "any", "Readability: Any" }
                    option { value: "80", "Readability: 80%+" }
                    option { value: "90", "Readability: 90%+" }
                    option { value: "95", "Readability: 95%+" }
                }
            }

            // Flat grid — no family groups
            div { class: "theme-grid",
                for theme in &filtered {
                    ThemeCard { theme: theme.clone() }
                }
            }

            if filtered.is_empty() {
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
                    div { class: "theme-card-actions",
                        ShortlistToggle { slug: slug.clone(), name: theme.name.clone() }
                        UseAsAppThemeButton { slug }
                    }
                }
            }
        }
    }
}
