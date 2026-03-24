use dioxus::prelude::*;

use crate::components::*;
use crate::scene_renderer;
use crate::state::*;
use crate::themes;
use crate::Route;

/// Scene-centric view: one scene rendered across all themes in a grid.
#[component]
pub fn SceneAcrossThemes(scene_id: String) -> Element {
    let active_provider = use_context::<Signal<ActiveProvider>>();
    let all_themes = themes::themes_for_provider(&active_provider.read().0);
    let scenes = litmus_model::scenes::all_scenes();
    let scene = scenes.iter().find(|s| s.id == scene_id);
    let cvd_sim = use_context::<Signal<CvdSimulation>>();
    let cvd = cvd_sim.read().0;

    match scene {
        Some(scene) => {
            rsx! {
                div { class: "page-scene-across",
                    h2 { class: "page-title", "{scene.name}" }
                    p { class: "page-subtitle", "{scene.description}" }

                    // Scene selector tabs
                    div { class: "scene-tabs scene-tabs-margin",
                        for s in &scenes {
                            Link {
                                to: Route::SceneAcrossThemes { scene_id: s.id.clone() },
                                class: if s.id == scene.id { "scene-tab scene-tab-active" } else { "scene-tab" },
                                "{s.name}"
                            }
                        }
                    }

                    div { class: "scene-grid",
                        for theme in &all_themes {
                            {
                                let sim_theme = maybe_simulate(theme, cvd);
                                rsx! {
                                    div { class: "scene-grid-card",
                                        div { class: "scene-grid-card-header",
                                            Link {
                                                to: Route::ThemeDetail {
                                                    slug: theme_slug(&theme.name),
                                                },
                                                class: "accent-link scene-grid-theme-name",
                                                "{theme.name}"
                                            }
                                            ShortlistToggle {
                                                slug: theme_slug(&theme.name),
                                                name: theme.name.clone(),
                                            }
                                        }
                                        scene_renderer::SceneView {
                                            theme: sim_theme,
                                            scene: scene.clone(),
                                            compact: true,
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        None => {
            rsx! {
                div {
                    h2 { "Scene not found" }
                    p { "No scene matches \"{scene_id}\"." }
                    Link { to: Route::ThemeList {}, class: "accent-link", "Back to all themes" }
                }
            }
        }
    }
}
