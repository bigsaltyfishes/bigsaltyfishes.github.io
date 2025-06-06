use crate::models::ArticleSearchIndex;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ArticleFiltersProps {
    pub search_index: ArticleSearchIndex,
    pub filter_category: Signal<Option<String>>,
    pub filter_tag: Signal<Option<String>>,
    pub on_filter_change: EventHandler<()>,
}

#[component]
pub fn ArticleFilters(props: ArticleFiltersProps) -> Element {
    let ArticleFiltersProps {
        search_index,
        mut filter_category,
        mut filter_tag,
        on_filter_change,
    } = props;

    let clear_filters_visible = filter_category.read().is_some() || filter_tag.read().is_some();

    rsx! {
        div { class: "filters-section",
            if !search_index.categories.is_empty() || !search_index.tags.is_empty() {
                div {
                    class: "filters",
                    div {
                        class: if clear_filters_visible { "clear-filters-container visible" } else { "clear-filters-container" },
                        button {
                            class: if clear_filters_visible { "clear-filters visible" } else { "clear-filters" },
                            onclick: move |_| {
                                filter_category.set(None);
                                filter_tag.set(None);
                                on_filter_change.call(());
                            },
                            "Clear Filters"
                        }
                    }

                    if !search_index.categories.is_empty() {
                        div { class: "filter-group",
                            span { class: "filter-label", "Categories:" }
                            for category in &search_index.categories {
                                button {
                                    class: if filter_category.read().as_ref() == Some(category) { "filter-pill active" } else { "filter-pill" },
                                    onclick: {
                                        let cat = category.clone();
                                        move |_| {
                                            let current_filter = filter_category.read().clone();
                                            if current_filter.as_ref() == Some(&cat) {
                                                filter_category.set(None);
                                            } else {
                                                filter_category.set(Some(cat.clone()));
                                                filter_tag.set(None);
                                            }
                                            on_filter_change.call(());
                                        }
                                    },
                                    "{category}"
                                }
                            }
                        }
                    }

                    if !search_index.tags.is_empty() {
                        div { class: "filter-group",
                            span { class: "filter-label", "Tags:" }
                            for tag in &search_index.tags {
                                button {
                                    class: if filter_tag.read().as_ref() == Some(tag) { "filter-pill tag active" } else { "filter-pill tag" },
                                    onclick: {
                                        let tag_clone = tag.clone();
                                        move |_| {
                                            let current_filter = filter_tag.read().clone();
                                            if current_filter.as_ref() == Some(&tag_clone) {
                                                filter_tag.set(None);
                                            } else {
                                                filter_tag.set(Some(tag_clone.clone()));
                                                filter_category.set(None);
                                            }
                                            on_filter_change.call(());
                                        }
                                    },
                                    "#{tag}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
