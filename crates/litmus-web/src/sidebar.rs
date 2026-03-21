use dioxus::prelude::*;

use crate::components::FilterButton;
use crate::family;
use crate::state::*;
use crate::themes;
use crate::Route;

/// Persistent left sidebar with all navigation, filtering,
/// compare management, and app theme switching.
#[component]
pub fn Sidebar() -> Element {
    let all_themes = themes::load_embedded_themes();
    let mut filter = use_context::<Signal<FilterState>>();
    let mut compare_sel = use_context::<Signal<CompareSelection>>();
    let mut sidebar_open = use_context::<Signal<SidebarOpen>>();
    let mut app_theme = use_context::<Signal<AppThemeSlug>>();
    let navigator = use_navigator();

    let filter_val = filter.read().clone();
    let query = filter_val.query.clone();
    let variant = filter_val.variant;
    let min_read = filter_val.min_readability;
    let cvd = filter_val.cvd;

    let filtered: Vec<litmus_model::Theme> = all_themes
        .iter()
        .filter(|t| theme_passes_filter(t, &filter_val))
        .cloned()
        .collect();
    let families = family::group_by_family(&filtered);
    let shown = filtered.len();

    // Count badges for variant filter
    let count_all = all_themes.len();
    let count_dark = all_themes.iter().filter(|t| !is_light_theme(t)).count();
    let count_light = all_themes.iter().filter(|t| is_light_theme(t)).count();

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
                        navigator.push(Route::ThemeList {});
                    },
                }
            }

            // Variant filter with count badges
            div { class: "sidebar-section sidebar-filters",
                button {
                    class: if variant == VariantFilter::All { "filter-btn filter-btn-active" } else { "filter-btn" },
                    aria_pressed: if variant == VariantFilter::All { "true" } else { "false" },
                    onclick: move |_| {
                        filter.write().variant = VariantFilter::All;
                        navigator.push(Route::ThemeList {});
                    },
                    "All "
                    span { class: "filter-badge", "({count_all})" }
                }
                button {
                    class: if variant == VariantFilter::Dark { "filter-btn filter-btn-active" } else { "filter-btn" },
                    aria_pressed: if variant == VariantFilter::Dark { "true" } else { "false" },
                    onclick: move |_| {
                        filter.write().variant = VariantFilter::Dark;
                        navigator.push(Route::ThemeList {});
                    },
                    "Dark "
                    span { class: "filter-badge", "({count_dark})" }
                }
                button {
                    class: if variant == VariantFilter::Light { "filter-btn filter-btn-active" } else { "filter-btn" },
                    aria_pressed: if variant == VariantFilter::Light { "true" } else { "false" },
                    onclick: move |_| {
                        filter.write().variant = VariantFilter::Light;
                        navigator.push(Route::ThemeList {});
                    },
                    "Light "
                    span { class: "filter-badge", "({count_light})" }
                }

                if shown < count_all {
                    span { class: "sidebar-count", "{shown}/{count_all}" }
                }
            }

            // Readability filter
            div { class: "sidebar-section sidebar-readability",
                span { class: "sidebar-section-label", "Min readability" }
                select {
                    class: "sidebar-readability-select",
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
                        navigator.push(Route::ThemeList {});
                    },
                    option { value: "any", "Any" }
                    option { value: "80", "80%" }
                    option { value: "90", "90%" }
                    option { value: "95", "95%" }
                }
            }

            // Theme list (scrollable) — with bg/fg colors and readability badge
            div { class: "sidebar-theme-list",
                for fam in &families {
                    div { class: "sidebar-family",
                        div { class: "sidebar-family-name", "{fam.name}" }
                        for theme in &fam.themes {
                            {
                                let slug = theme_slug(&theme.name);
                                let bg = theme.background.to_hex();
                                let fg = theme.foreground.to_hex();
                                let score = litmus_model::contrast::readability_score(theme) as u8;
                                rsx! {
                                    Link {
                                        to: Route::ThemeDetail { slug: slug.clone() },
                                        class: "sidebar-theme-item",
                                        style: "background: {bg}; color: {fg};",
                                        onclick: move |_| sidebar_open.set(SidebarOpen(false)),
                                        span { class: "sidebar-theme-name", "{theme.name}" }
                                        span { class: "sidebar-readability-badge", "{score}%" }
                                    }
                                }
                            }
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

            // CVD (moved to bottom, above app theme)
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

            // App theme selector (pinned to bottom)
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
