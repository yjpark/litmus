use dioxus::prelude::*;

use crate::components::ColorSwatch;
use crate::fixtures;
use crate::scene_renderer;
use crate::screenshot_view::scene_id_to_fixture_id;
use crate::state::*;
use crate::term_renderer;
use crate::themes;
use crate::Route;

static ANSI_NAMES: &[&str] = &[
    "black", "red", "green", "yellow", "blue", "magenta", "cyan", "white",
    "bright black", "bright red", "bright green", "bright yellow",
    "bright blue", "bright magenta", "bright cyan", "bright white",
];

/// Multi-theme comparison (2-4 themes side by side).
#[component]
pub fn CompareThemes(slugs: String) -> Element {
    let active_provider = use_context::<Signal<ActiveProvider>>();
    let all_themes = themes::themes_for_provider(&active_provider.read().0);
    let scenes = litmus_model::scenes::all_scenes();
    let cvd_sim = use_context::<Signal<CvdSimulation>>();
    let cvd = cvd_sim.read().0;

    let slug_list: Vec<&str> = slugs.split(',').filter(|s| !s.is_empty()).collect();

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
    let grid_cols = format!("repeat({n}, 1fr)");

    rsx! {
        div { class: "page-compare",
            style: "--compare-cols: {grid_cols};",

            // Sticky column headers with theme names
            div { class: "compare-column-headers",
                for theme in &compare_themes {
                    div { class: "compare-column-header",
                        Link {
                            to: Route::ThemeDetail {
                                slug: theme_slug(&theme.name),
                            },
                            class: "compare-column-header-link",
                            "{theme.name}"
                        }
                    }
                }
            }

            for scene in &scenes {
                div { class: "compare-scene-group",
                    id: "scene-{scene.id}",
                    h3 { class: "compare-scene-name", "{scene.name}" }

                    div { class: "compare-grid",
                        for theme in &compare_themes {
                            div { class: "compare-grid-item",
                                {
                                    let fixture_output = scene_id_to_fixture_id(&scene.id)
                                        .and_then(fixtures::fixture_by_id);
                                    if let Some(output) = fixture_output {
                                        rsx! {
                                            term_renderer::TermOutputView {
                                                theme: theme.clone(),
                                                output: output.clone(),
                                                compact: n > 2,
                                            }
                                        }
                                    } else {
                                        rsx! {
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

            // Color palettes at the end
            h3 { class: "compare-scene-name", "Color Palette" }
            div { class: "compare-grid",
                for theme in &compare_themes {
                    {
                        let bg = theme.background.to_hex();
                        let fg = theme.foreground.to_hex();
                        rsx! {
                            div { class: "compare-grid-item",
                                div {
                                    class: "color-palette",
                                    style: "background: {bg}; color: {fg};",
                                    div { class: "palette-expanded",
                                        div { class: "special-colors",
                                            ColorSwatch { label: "bg".to_string(), color: theme.background.to_hex() }
                                            ColorSwatch { label: "fg".to_string(), color: theme.foreground.to_hex() }
                                            ColorSwatch { label: "cursor".to_string(), color: theme.cursor.to_hex() }
                                            ColorSwatch { label: "sel bg".to_string(), color: theme.selection_background.to_hex() }
                                            ColorSwatch { label: "sel fg".to_string(), color: theme.selection_foreground.to_hex() }
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
                        }
                    }
                }
            }
        }
    }
}
