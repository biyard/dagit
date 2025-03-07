use crate::{components::ServiceLogo, routes::Route};
use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_translate::{Language, translate};

mod i18n;
use i18n::NavigationTranslate;

#[derive(PartialEq, Eq)]
enum SelectedSection {
    HomePage,
    Orders,
    Management,
    Community,
    Customers,
    Analytics,
    Design,
    ExtensionTool,
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
    None,
}
fn check_route(route: Route) -> (SelectedSection, SelectedItem) {
    match route {
        Route::HomePage {
            lang: _,
            agit_id: _,
        } => (SelectedSection::HomePage, SelectedItem::None),
        _ => {
            tracing::error!("Unknown route: {}", route);
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
        div { class: "flex flex-col p-10 gap-12.5",
            ServiceLogo {}
            div { class: "flex flex-col gap-5 text-white text-base",
                Section {
                    label: tr.home,
                    selected: selected_section == SelectedSection::HomePage,
                }
                Section {
                    label: tr.orders,
                    selected: selected_section == SelectedSection::Orders,
                    Item { selected: selected_item == SelectedItem::SalesRequest,
                        Link { to: Route::HomePage { lang, agit_id }, {tr.sales_request} }
                    }
                    Item { selected: selected_item == SelectedItem::ShippingLabel,
                        Link { to: Route::HomePage { lang, agit_id }, {tr.shipping_label} }
                    }

                }
                Section {
                    label: tr.management,
                    selected: selected_section == SelectedSection::Management,
                    Item { selected: selected_item == SelectedItem::Artworks,
                        Link { to: Route::HomePage { lang, agit_id }, {tr.artworks} }
                    }
                    Item { selected: selected_item == SelectedItem::Collections,
                        Link { to: Route::HomePage { lang, agit_id }, {tr.collections} }
                    }
                    Item { selected: selected_item == SelectedItem::Artist,
                        Link { to: Route::HomePage { lang, agit_id }, {tr.artists} }
                    }
                }
                Section {
                    label: tr.community,
                    selected: selected_section == SelectedSection::Community,
                    Item { selected: selected_item == SelectedItem::Collectors,
                        Link { to: Route::HomePage { lang, agit_id }, {tr.collectors} }
                    }
                    Item { selected: selected_item == SelectedItem::Dao,
                        Link { to: Route::HomePage { lang, agit_id }, {tr.dao} }
                    }
                    Item { selected: selected_item == SelectedItem::Oracle,
                        Link { to: Route::HomePage { lang, agit_id }, {tr.oracle} }
                    }
                    Item { selected: selected_item == SelectedItem::Faq,
                        Link { to: Route::HomePage { lang, agit_id }, {tr.faq} }
                    }
                }
                Section {
                    label: tr.customers,
                    selected: selected_section == SelectedSection::Customers,
                }
                Section {
                    label: tr.analytics,
                    selected: selected_section == SelectedSection::Analytics,
                    Item { selected: selected_item == SelectedItem::Report,
                        Link { to: Route::HomePage { lang, agit_id }, {tr.report} }
                    }
                    Item { selected: selected_item == SelectedItem::Traffic,
                        Link { to: Route::HomePage { lang, agit_id }, {tr.traffic} }
                    }
                }
                Section {
                    label: tr.design,
                    selected: selected_section == SelectedSection::Design,
                }
                Section {
                    label: tr.extension_tool,
                    selected: selected_section == SelectedSection::ExtensionTool,
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
) -> Element {
    rsx! {
        div {
            class: format!(
                "hover:border-primary cursor-pointer transition-colors duration-200 ease-in-out {}",
                if selected { "border-primary" } else { "" },
            ),
            div {
                class: format!(
                    "flex items-center py-2.5 gap-2 {}",
                    if selected { "text-primary" } else { "" },
                ),
                {label}
            }
            div { class: "ml-5", {children} }
        }
    }
}

#[component]
fn Item(selected: bool, children: Element) -> Element {
    rsx! {
        div { class: format!("{}", if selected { "text-primary" } else { "text-inherit" }),
            {children}
        }
    }
}
