use dioxus::prelude::*;

use crate::components::FilterButton;
use crate::family;
use crate::state::*;
use crate::themes;
use crate::Route;

/// Persistent left sidebar with all navigation, filtering, scene selection,
/// compare management, and app theme switching.
#[component]
pub fn Sidebar() -> Element {
    let all_themes = themes::load_embedded_themes();
    let scenes = litmus_model::scenes::all_scenes();
    let mut filter = use_context::<Signal<FilterState>>();
    let mut compare_sel = use_context::<Signal<CompareSelection>>();
    let mut sidebar_open = use_context::<Signal<SidebarOpen>>();
    let mut app_theme = use_context::<Signal<AppThemeSlug>>();

    let filter_val = filter.read().clone();
    let query = filter_val.query.clone();
    let variant = filter_val.variant;
    let contrast_on = filter_val.good_contrast;
    let cvd = filter_val.cvd;

    let filtered: Vec<litmus_model::Theme> = all_themes
        .iter()
        .filter(|t| theme_passes_filter(t, &filter_val))
        .cloned()
        .collect();
    let families = family::group_by_family(&filtered);
    let total = all_themes.len();
    let shown = filtered.len();

    let sel = compare_sel.read().clone();
    let sel_count = sel.0.len();
    let can_compare = sel_count >= 2;

    let current_app_theme = app_theme.read().0.clone();

    rsx! {
        aside {
            class: "sidebar",
            role: "navigation",
            aria_label: "Main navigation",

            // Header
            div { class: "sidebar-header",
                Link { to: Route::ThemeList {}, class: "sidebar-logo",
                    span { class: "sidebar-logo-text", "litmus" }
                }
                span { class: "sidebar-subtitle", "terminal color theme previewer" }
            }

            // Search
            div { class: "sidebar-section",
                label { class: "sr-only", r#for: "sidebar-search", "Search themes" }
                input {
                    id: "sidebar-search",
                    class: "sidebar-search",
                    r#type: "text",
                    placeholder: "Search themes...",
                    value: "{query}",
                    oninput: move |evt: Event<FormData>| {
                        filter.write().query = evt.value();
                    },
                }
            }

            // Filters
            div { class: "sidebar-section sidebar-filters",
                FilterButton {
                    label: "All",
                    active: variant == VariantFilter::All,
                    onclick: move |_| filter.write().variant = VariantFilter::All,
                }
                FilterButton {
                    label: "Dark",
                    active: variant == VariantFilter::Dark,
                    onclick: move |_| filter.write().variant = VariantFilter::Dark,
                }
                FilterButton {
                    label: "Light",
                    active: variant == VariantFilter::Light,
                    onclick: move |_| filter.write().variant = VariantFilter::Light,
                }
                span { class: "sidebar-divider", "|" }
                FilterButton {
                    label: "Good contrast",
                    active: contrast_on,
                    onclick: move |_| {
                        let v = filter.read().good_contrast;
                        filter.write().good_contrast = !v;
                    },
                }

                if shown < total {
                    span { class: "sidebar-count", "{shown}/{total}" }
                }
            }

            // CVD
            div { class: "sidebar-section sidebar-cvd",
                span { class: "sidebar-section-label",
                    title: "Simulate color vision deficiency",
                    "CVD"
                }
                FilterButton {
                    label: "Normal",
                    active: cvd.is_none(),
                    onclick: move |_| filter.write().cvd = None,
                }
                for cvd_type in litmus_model::cvd::CvdType::all() {
                    {
                        let ct = *cvd_type;
                        let label = ct.label();
                        let desc = ct.description();
                        rsx! {
                            button {
                                class: if cvd == Some(ct) { "filter-btn filter-btn-active" } else { "filter-btn" },
                                aria_pressed: if cvd == Some(ct) { "true" } else { "false" },
                                title: "{desc}",
                                onclick: move |_| filter.write().cvd = Some(ct),
                                "{label}"
                            }
                        }
                    }
                }
            }

            // Theme list (scrollable)
            div { class: "sidebar-theme-list",
                for fam in &families {
                    div { class: "sidebar-family",
                        div { class: "sidebar-family-name", "{fam.name}" }
                        for theme in &fam.themes {
                            {
                                let slug = theme_slug(&theme.name);
                                let bg = theme.background.to_hex();
                                rsx! {
                                    Link {
                                        to: Route::ThemeDetail { slug: slug.clone() },
                                        class: "sidebar-theme-item",
                                        onclick: move |_| sidebar_open.set(SidebarOpen(false)),
                                        div {
                                            class: "sidebar-theme-dot",
                                            style: "background: {bg};",
                                        }
                                        span { class: "sidebar-theme-name", "{theme.name}" }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Scenes
            div { class: "sidebar-section",
                div { class: "sidebar-section-label", "Scenes" }
                div { class: "sidebar-scene-chips",
                    for scene in &scenes {
                        Link {
                            to: Route::SceneAcrossThemes { scene_id: scene.id.clone() },
                            class: "sidebar-scene-chip",
                            onclick: move |_| sidebar_open.set(SidebarOpen(false)),
                            "{scene.name}"
                        }
                    }
                }
            }

            // Compare
            if sel_count > 0 {
                div { class: "sidebar-section sidebar-compare",
                    div { class: "sidebar-section-label", "Compare ({sel_count})" }
                    div { class: "sidebar-compare-chips",
                        for slug in &sel.0 {
                            {
                                let name = all_themes.iter()
                                    .find(|t| theme_slug(&t.name) == *slug)
                                    .map(|t| t.name.clone())
                                    .unwrap_or_else(|| slug.clone());
                                let slug_remove = slug.clone();
                                rsx! {
                                    span { class: "compare-chip",
                                        "{name}"
                                        button {
                                            class: "compare-chip-remove",
                                            onclick: move |_| {
                                                compare_sel.write().0.retain(|s| s != &slug_remove);
                                            },
                                            "x"
                                        }
                                    }
                                }
                            }
                        }
                    }
                    div { class: "sidebar-compare-actions",
                        if can_compare {
                            Link {
                                to: Route::CompareThemes {
                                    slugs: sel.0.join(","),
                                },
                                class: "sidebar-compare-btn",
                                onclick: move |_| sidebar_open.set(SidebarOpen(false)),
                                "Compare"
                            }
                        }
                        button {
                            class: "sidebar-clear-btn",
                            onclick: move |_| {
                                compare_sel.write().0.clear();
                            },
                            "Clear"
                        }
                    }
                }
            }

            // App theme selector
            div { class: "sidebar-section sidebar-app-theme",
                div { class: "sidebar-section-label", "App Theme" }
                select {
                    class: "sidebar-app-theme-select",
                    value: if let Some(ref s) = current_app_theme { s.as_str() } else { "__default__" },
                    onchange: move |evt: Event<FormData>| {
                        let val = evt.value();
                        if val == "__default__" {
                            app_theme.set(AppThemeSlug(None));
                        } else {
                            app_theme.set(AppThemeSlug(Some(val)));
                        }
                    },
                    option { value: "__default__", "Default (Tokyo Night)" }
                    for t in &all_themes {
                        option { value: "{theme_slug(&t.name)}", "{t.name}" }
                    }
                }
            }
        }
    }
}
