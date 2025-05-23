use crate::{components::ServiceLogo, routes::Route};
use bdk::prelude::*;
use by_components::icons::arrows::BendArrowRight;
mod i18n;
use i18n::NavigationTranslate;

#[derive(PartialEq, Eq)]
enum SelectedSection {
    HomePage,
    Orders,
    Management,
    Analytics,
    Design,
    ExtensionTool,
    Hub,
}
#[derive(PartialEq, Eq)]
enum SelectedItem {
    SalesRequest,
    ShippingLabel,
    Artworks,
    Collections,
    Artist,
    Collectors,
    Dao,
    Oracle,
    Faq,
    Report,
    Traffic,
    Design,
    ExtensionTool,
    None,
}
fn check_route(route: Route) -> (SelectedSection, SelectedItem) {
    match route {
        Route::ArtworkPage { .. } | Route::CreateArtworkPage { .. } => {
            (SelectedSection::Management, SelectedItem::Artworks)
        }
        Route::CollectionPage { .. } | Route::CollectionDetailPage { .. } => {
            (SelectedSection::Management, SelectedItem::Collections)
        }
        Route::ArtistPage { .. }
        | Route::ArtistDetailPage { .. }
        | Route::EditArtistPage { .. }
        | Route::CreateArtistPage { .. } => (SelectedSection::Management, SelectedItem::Artist),

        Route::CollectorPage { .. } | Route::CollectorDetailPage { .. } => {
            (SelectedSection::Management, SelectedItem::Collectors)
        }
        Route::DaoPage { .. } => (SelectedSection::Hub, SelectedItem::Dao),
        Route::OraclePage { .. } => (SelectedSection::Hub, SelectedItem::Oracle),
        Route::FaqPage { .. } => (SelectedSection::Hub, SelectedItem::Faq),
        Route::ReportPage { .. } => (SelectedSection::Analytics, SelectedItem::Report),
        Route::TrafficPage { .. } => (SelectedSection::Analytics, SelectedItem::Traffic),
        Route::DesignPage { .. } => (SelectedSection::Design, SelectedItem::Design),
        Route::ExtensionToolPage { .. } => {
            (SelectedSection::ExtensionTool, SelectedItem::ExtensionTool)
        }
        Route::HomePage { .. } => (SelectedSection::HomePage, SelectedItem::None),

        Route::SalesRequestPage { .. } => (SelectedSection::Orders, SelectedItem::SalesRequest),
        Route::ShippingLabelPage { .. } => (SelectedSection::Orders, SelectedItem::ShippingLabel),

        Route::RootPage { .. } | Route::NotFoundPage { .. } => {
            (SelectedSection::HomePage, SelectedItem::None)
        }
    }
}
#[component]
pub fn Navigation(lang: Language, agit_id: i64) -> Element {
    let tr: NavigationTranslate = translate(&lang);
    let route = use_route::<Route>();
    let (selected_section, selected_item) = check_route(route);
    rsx! {
        div { class: "flex flex-col p-10 gap-12.5 align-start",
            ServiceLogo { width: "110", height: "24", class: "fill-white" }
            div { class: "flex flex-col gap-5 text-white text-base",

                Section {
                    label: tr.home,
                    selected: selected_section == SelectedSection::HomePage,
                    to: Route::HomePage { lang, agit_id },
                }
                Section {
                    label: tr.orders,
                    selected: selected_section == SelectedSection::Orders,
                    Item { selected: selected_item == SelectedItem::SalesRequest,
                        Link {
                            to: Route::SalesRequestPage {
                                lang,
                                agit_id,
                            },
                            {tr.sales_request}
                        }
                    }
                    Item { selected: selected_item == SelectedItem::ShippingLabel,
                        Link {
                            to: Route::ShippingLabelPage {
                                lang,
                                agit_id,
                            },
                            {tr.shipping_label}
                        }
                    }
                }
                Section {
                    label: tr.management,
                    selected: selected_section == SelectedSection::Management,
                    Item { selected: selected_item == SelectedItem::Artworks,
                        Link {
                            to: Route::ArtworkPage {
                                lang,
                                agit_id,
                            },
                            {tr.artworks}
                        }
                    }
                    Item { selected: selected_item == SelectedItem::Collections,
                        Link {
                            to: Route::CollectionPage {
                                lang,
                                agit_id,
                            },
                            {tr.collections}
                        }
                    }
                    Item { selected: selected_item == SelectedItem::Artist,
                        Link { to: Route::ArtistPage { lang, agit_id }, {tr.artists} }
                    }
                    Item { selected: selected_item == SelectedItem::Collectors,
                        Link {
                            to: Route::CollectorPage {
                                lang,
                                agit_id,
                            },
                            {tr.collectors}
                        }
                    }
                }
                Section {
                    label: tr.hub,
                    selected: selected_section == SelectedSection::Hub,
                    Item { selected: selected_item == SelectedItem::Dao,
                        Link { to: Route::DaoPage { lang, agit_id }, {tr.dao} }
                    }
                    Item { selected: selected_item == SelectedItem::Oracle,
                        Link { to: Route::OraclePage { lang, agit_id }, {tr.oracle} }
                    }
                    Item { selected: selected_item == SelectedItem::Faq,
                        Link { to: Route::FaqPage { lang, agit_id }, {tr.faq} }
                    }
                }
                Section {
                    label: tr.analytics,
                    selected: selected_section == SelectedSection::Analytics,
                    Item { selected: selected_item == SelectedItem::Report,
                        Link { to: Route::ReportPage { lang, agit_id }, {tr.report} }
                    }
                    Item { selected: selected_item == SelectedItem::Traffic,
                        Link {
                            to: Route::TrafficPage {
                                lang,
                                agit_id,
                            },
                            {tr.traffic}
                        }
                    }
                }
                Section {
                    label: tr.design,
                    selected: selected_section == SelectedSection::Design,
                    to: Route::DesignPage { lang, agit_id },
                }
                Section {
                    label: tr.extension_tool,
                    selected: selected_section == SelectedSection::ExtensionTool,
                    to: Route::ExtensionToolPage {
                        lang,
                        agit_id,
                    },
                }
            }
        }
    }
}

#[component]
fn Section(
    label: String,
    #[props(default = VNode::empty())] children: Element,
    selected: bool,
    #[props(default = None)] to: Option<Route>,
) -> Element {
    let content = rsx! {
        div { class: "gap-1",
            div {
                "aria-selected": selected,
                class: "flex items-center hover:text-primary cursor-pointer transition-colors duration-200 ease-in-out border-l border-transparent aria-selected:border-primary aria-selected:text-primary py-2 px-3 ml-[1px] whitespace-nowrap",
                {label}
            }
            div { class: "flex flex-col gap-1", {children} }
        }
    };

    match to {
        Some(route) => rsx! {
            Link { to: route, {content} }
        },
        None => content,
    }
}

#[component]
fn Item(selected: bool, children: Element) -> Element {
    rsx! {
        div { class: "flex items-center gap-1",
            BendArrowRight {
                class: format!(
                    "[&>path]:{}",
                    if selected { "stroke-primary" } else { "stroke-transparent" },
                ),
                width: "18px",
                height: "18px",
            }
            div {
                class: format!(
                    "hover:text-primary cursor-pointer transition-colors duration-200 ease-in-out {}",
                    if selected { "text-primary" } else { "text-inherit" },
                ),
                {children}
            }
        }
    }
}
