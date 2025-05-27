// use crate::{
//     components::{search_filter_bar::SearchFilterBar, tab_button::TabButton},
//     pages::agits::_id::management::{
//         collections::Controller,
//         collectors::i18n::CollectorsTranslate,
//         components::{ActivityTable, NftTable, OwnedTable},
//     },
//     routes::Route,
// };
use bdk::prelude::*;

// #[derive(Clone, Debug, PartialEq)]
// pub enum AssetTab {
//     Owned,
//     Created,
//     Trade,
//     Activity,
// }
#[component]
#[allow(unused_variables)]
pub fn CollectorDetailPage(
    lang: Language,
    agit_id: ReadOnlySignal<i64>,
    collector_id: i64,
) -> Element {
    // let search_query = use_signal(String::new);
    // let mut view_mode = use_signal(|| "table");
    // let filter = use_signal(|| "All");
    // let tr: CollectorsTranslate = translate(&lang);
    // let ctrl = Controller::new(lang, agit_id)?;
    // let assets = ctrl.artworks();
    // let mut active_tab = use_signal(|| AssetTab::Owned);
    rsx! {}
    // rsx! {
    //     div { class: "w-full min-h-screen bg-background h-full flex text-white justify-center items-center",

    //         div { class: "flex flex-col w-full h-full text-white",

    //             // Header with back button and collector info
    //             div { class: "flex flex-col mb-6",
    //                 div { class: "flex items-center",
    //                     Link {
    //                         to: Route::CollectorPage {
    //                             lang,
    //                             agit_id: agit_id(),
    //                         },
    //                         class: "text-gray-400 hover:text-white ",
    //                         svg {
    //                             xmlns: "http://www.w3.org/2000/svg",
    //                             class: "h-6 w-6",
    //                             fill: "none",
    //                             view_box: "0 0 24 24",
    //                             stroke: "currentColor",
    //                             path {
    //                                 stroke_linecap: "round",
    //                                 stroke_linejoin: "round",
    //                                 stroke_width: "2",
    //                                 d: "M15 19l-7-7 7-7",
    //                             }
    //                         }
    //                     }

    //                     div { class: "flex items-center",

    //                         h1 { class: "text-xl font-bold",
    //                             {format!("{{Collector {}}}", collector_id)}
    //                         }
    //                     }
    //                 }
    //                 div { class: "text-sm text-gray-400 m-2",
    //                     "Last Activity 2mins ago / Joined Feb 01, 2025"
    //                 }
    //             }
    //             div { class: "flex flex-col text-white space-y-4 mb-20",

    //                 // Top: Avatar + Stats
    //                 div { class: "flex items-start",
    //                     // Avatar section
    //                     div { class: "relative mr-8",
    //                         div { class: "w-20 h-20 bg-gray-300 rounded-full" }
    //                         div { class: "absolute -bottom-1 -right-1 text-green-500 mr-2",
    //                             svg {
    //                                 view_box: "0 0 24 24",
    //                                 width: "28",
    //                                 height: "28",
    //                                 fill: "#10b981",
    //                                 class: "ml-1",
    //                                 path { d: "M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" }
    //                             }
    //                         }
    //                     }

    //                     // Stats + Wallets (stacked vertically)
    //                     div { class: "flex-1 grid grid-cols-3 gap-4",

    //                         // Total Volume and created  column
    //                         div {
    //                             // Stat
    //                             div { class: "border-b-2 border-border-primary flex justify-between items-center pb-2  mb-4",
    //                                 p { class: "text-sm text-white", {tr.total_volume} }
    //                                 p { class: "text-sm font-bold", "2,370 ETH" }
    //                             }
    //                             // Wallet
    //                             div { class: "pt-2 border-b-2 border-border-primary flex justify-between items-center pb-2 mt-4",
    //                                 p { class: "text-sm text-white", "Created" }
    //                                 p { class: "text-sm font-bold", "2,370 ETH" }
    //                             }
    //                         }
    //                         // Total owned  and wallet column
    //                         div {
    //                             // Stat
    //                             div { class: "border-b-2 border-border-primary flex justify-between items-center pb-2 mb-4",
    //                                 p { class: "text-sm text-white", {tr.owned} }
    //                                 p { class: "text-sm font-bold", "2,370 ETH" }
    //                             }
    //                             // Wallet
    //                             div { class: "pt-2 border-b-2 border-border-primary flex justify-between items-center pb-2 mt-4",
    //                                 p { class: "text-sm text-white", {tr.wallet_address} }
    //                                 div { class: "flex items-center",
    //                                     p { class: "font-mono text-sm", "0x1234...abcd" }
    //                                     button { class: "ml-2 text-white",
    //                                         svg {
    //                                             xmlns: "http://www.w3.org/2000/svg",
    //                                             class: "h-4 w-4",
    //                                             fill: "none",
    //                                             view_box: "0 0 24 24",
    //                                             stroke: "currentColor",
    //                                             path {
    //                                                 stroke_linecap: "round",
    //                                                 stroke_linejoin: "round",
    //                                                 stroke_width: "2",
    //                                                 d: "M19 9l-7 7-7-7",
    //                                             }
    //                                         }
    //                                     }
    //                                 }
    //                             }
    //                         }
    //                         // Total owned and wallet column
    //                         div {
    //                             // Stat
    //                             div { class: "border-b-2 border-border-primary flex justify-between items-center pb-2 mb-4",
    //                                 p { class: "text-sm text-white", {tr.owned} }
    //                                 p { class: "text-sm font-bold", "2,370 ETH" }
    //                             }
    //                             // Wallet
    //                             div { class: "pt-2 border-b-2 border-border-primary flex justify-between items-center pb-2 mt-4",
    //                                 p { class: "text-sm text-white", {tr.wallet_address} }
    //                                 div { class: "flex items-center",
    //                                     p { class: "font-mono text-sm", "0x1234...abcd" }
    //                                     button { class: "ml-2 text-white",
    //                                         svg {
    //                                             xmlns: "http://www.w3.org/2000/svg",
    //                                             class: "h-4 w-4",
    //                                             fill: "none",
    //                                             view_box: "0 0 24 24",
    //                                             stroke: "currentColor",
    //                                             path {
    //                                                 stroke_linecap: "round",
    //                                                 stroke_linejoin: "round",
    //                                                 stroke_width: "2",
    //                                                 d: "M19 9l-7 7-7-7",
    //                                             }
    //                                         }
    //                                     }
    //                                 }
    //                             }
    //                         }
    //                     }
    //                 }
    //             }

    //             div { class: "mb-10",
    //                 div { class: "flex space-x-8 text-sm font-medium",
    //                     TabButton {
    //                         label: tr.owned,
    //                         is_active: *active_tab.read() == AssetTab::Owned,
    //                         badge_text: Some("585".to_string()),
    //                         onclick: move |_| active_tab.set(AssetTab::Owned),
    //                     }

    //                     TabButton {
    //                         label: tr.created,
    //                         is_active: *active_tab.read() == AssetTab::Created,
    //                         badge_text: Some("585".to_string()),
    //                         onclick: move |_| active_tab.set(AssetTab::Created),
    //                     }

    //                     TabButton {
    //                         label: tr.trade,
    //                         is_active: *active_tab.read() == AssetTab::Trade,
    //                         badge_text: Some("585".to_string()),
    //                         onclick: move |_| active_tab.set(AssetTab::Trade),
    //                     }

    //                     TabButton {
    //                         label: tr.activity,
    //                         is_active: *active_tab.read() == AssetTab::Activity,
    //                         onclick: move |_| active_tab.set(AssetTab::Activity),
    //                     }
    //                 }
    //             }
    //             SearchFilterBar {
    //                 placeholder: tr.search_by_title,
    //                 show_filter_btn: true,
    //                 show_art_btn: true,
    //                 on_search_change: move |search_text| {},
    //                 on_search: move |search_text| {},
    //                 on_view_mode_click: move |_| {
    //                     if *view_mode.read() == "table" {
    //                         view_mode.set("nftImages");
    //                     } else {
    //                         view_mode.set("table");
    //                     }
    //                 },
    //             }

    //             // Assets table
    //             div { class: "overflow-x-auto flex-1",

    //                 {
    //                     match active_tab.with(|tab| tab.clone()) {
    //                         AssetTab::Owned => {
    //                             if *view_mode.read() == "nftImages" {
    //                                 rsx! {
    //                                     NftTable { artworks: assets.clone(), lang }
    //                                 }
    //                             } else {
    //                                 rsx! {
    //                                     OwnedTable { artworks: assets.clone(), lang }
    //                                 }
    //                             }
    //                         }
    //                         AssetTab::Created => {
    //                             if *view_mode.read() == "nftImages" {
    //                                 rsx! {
    //                                     NftTable { artworks: assets.clone(), lang }
    //                                 }
    //                             } else {
    //                                 rsx! {
    //                                     OwnedTable { artworks: assets.clone(), lang }
    //                                 }
    //                             }
    //                         }
    //                         AssetTab::Trade => {
    //                             if *view_mode.read() == "nftImages" {
    //                                 rsx! {
    //                                     NftTable { artworks: assets.clone(), lang }
    //                                 }
    //                             } else {
    //                                 rsx! {
    //                                     OwnedTable { artworks: assets.clone(), lang }
    //                                 }
    //                             }
    //                         }
    //                         AssetTab::Activity => {
    //                             if *view_mode.read() == "nftImages" {
    //                                 rsx! {
    //                                     NftTable { artworks: assets.clone(), lang }
    //                                 }
    //                             } else {
    //                                 rsx! {
    //                                     ActivityTable { activity: assets.clone(), lang }
    //                                 }
    //                             }
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }
}
