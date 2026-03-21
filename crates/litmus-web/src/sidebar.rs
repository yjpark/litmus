use std::collections::HashSet;

use dioxus::prelude::*;

use crate::components::FilterButton;
use crate::state::*;
use crate::themes;
use crate::Route;

/// Persistent left sidebar: navigation, shortlist management, CVD toggle.
#[component]
pub fn Sidebar() -> Element {
    let all_themes = themes::load_embedded_themes();
    let mut shortlist = use_context::<Signal<Shortlist>>();
    let mut cvd_sim = use_context::<Signal<CvdSimulation>>();
    let mut sidebar_open = use_context::<Signal<SidebarOpen>>();

    let cvd = cvd_sim.read().0;
    let sl = shortlist.read().clone();
    let sl_count = sl.0.len();

    // Local state: which shortlisted themes are checked for compare
    let mut checked = use_signal(HashSet::<String>::new);
    // Sync checked set: remove entries no longer in shortlist
    {
        let sl_set: HashSet<String> = sl.0.iter().cloned().collect();
        let current_checked = checked.read().clone();
        let valid: HashSet<String> = current_checked.intersection(&sl_set).cloned().collect();
        if valid.len() != current_checked.len() {
            checked.set(valid);
        }
    }
    let checked_val = checked.read().clone();
    let checked_count = checked_val.len();
    let can_compare = (2..=MAX_COMPARE).contains(&checked_count);

    // Build compare URL from checked slugs (preserve shortlist order)
    let compare_slugs: Vec<String> = sl.0.iter()
        .filter(|s| checked_val.contains(*s))
        .cloned()
        .collect();
    let compare_url = compare_slugs.join(",");

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

            // Nav links
            div { class: "sidebar-section sidebar-nav",
                Link {
                    to: Route::ThemeList {},
                    class: "sidebar-nav-link",
                    onclick: move |_| sidebar_open.set(SidebarOpen(false)),
                    "Browse Themes"
                }
                if can_compare {
                    Link {
                        to: Route::CompareThemes { slugs: compare_url.clone() },
                        class: "sidebar-nav-link",
                        onclick: move |_| sidebar_open.set(SidebarOpen(false)),
                        "Compare ({checked_count})"
                    }
                }
            }

            // Shortlist
            if sl_count > 0 {
                div { class: "sidebar-section sidebar-shortlist",
                    div { class: "sidebar-section-label", "Shortlist ({sl_count})" }
                    div { class: "sidebar-shortlist-items",
                        for slug in &sl.0 {
                            {
                                let name = all_themes.iter()
                                    .find(|t| theme_slug(&t.name) == *slug)
                                    .map(|t| t.name.clone())
                                    .unwrap_or_else(|| slug.clone());
                                let slug_check = slug.clone();
                                let slug_remove = slug.clone();
                                let is_checked = checked_val.contains(slug);
                                rsx! {
                                    div { class: "sidebar-shortlist-item",
                                        label { class: "sidebar-shortlist-check",
                                            input {
                                                r#type: "checkbox",
                                                checked: is_checked,
                                                onchange: move |_| {
                                                    let mut c = checked.write();
                                                    if c.contains(&slug_check) {
                                                        c.remove(&slug_check);
                                                    } else if c.len() < MAX_COMPARE {
                                                        c.insert(slug_check.clone());
                                                    }
                                                },
                                            }
                                            span { class: "sidebar-shortlist-name", "{name}" }
                                        }
                                        button {
                                            class: "sidebar-shortlist-remove",
                                            title: "Remove from shortlist",
                                            onclick: move |_| {
                                                shortlist.write().0.retain(|s| s != &slug_remove);
                                            },
                                            "\u{00d7}"
                                        }
                                    }
                                }
                            }
                        }
                    }

                    div { class: "sidebar-shortlist-actions",
                        if can_compare {
                            Link {
                                to: Route::CompareThemes { slugs: compare_url },
                                class: "sidebar-compare-btn",
                                onclick: move |_| sidebar_open.set(SidebarOpen(false)),
                                "Compare ({checked_count})"
                            }
                        } else if checked_count > MAX_COMPARE {
                            span { class: "sidebar-shortlist-hint", "Max {MAX_COMPARE} for compare" }
                        }
                        button {
                            class: "sidebar-clear-btn",
                            onclick: move |_| {
                                shortlist.write().0.clear();
                            },
                            "Clear"
                        }
                    }
                }
            }

            // CVD (pinned to bottom)
            div { class: "sidebar-section sidebar-cvd",
                span { class: "sidebar-section-label",
                    title: "Simulate color vision deficiency",
                    "CVD"
                }
                FilterButton {
                    label: "Normal",
                    active: cvd.is_none(),
                    onclick: move |_| cvd_sim.set(CvdSimulation(None)),
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
                                onclick: move |_| cvd_sim.set(CvdSimulation(Some(ct))),
                                "{label}"
                            }
                        }
                    }
                }
            }
        }
    }
}
