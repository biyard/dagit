#![allow(unused)]
use std::str::FromStr;

use bdk::prelude::by_components::icons::arrows::ChevronLeft;
use bdk::prelude::by_components::icons::{
    arrows::ChevronRight, arrows::ShapeArrowUp, other_devices::Save,
};
use bdk::prelude::dioxus_elements::image;
use bdk::prelude::serde_json::de;
use bdk::prelude::*;
use common::tables::prelude::{
    ArtStyle, Currency, Material, Medium, Rarity, Royalty, Size, Theme, Weight,
};

use crate::components::input::{Input, TextArea};
use crate::components::{button::SecondaryButton, dropdown::DropDown};

use common::tables::artworks::WaysToSell;

use super::components::{
    input::TitleInput,
    tab_header::{Tab, TabHeader},
};
use super::controller::Controller;
use super::i18n::CreateArtworkPageTranslate;

#[component]
pub fn CreateArtworkPage(lang: Language, agit_id: ReadOnlySignal<i64>) -> Element {
    let mut ctrl = Controller::new(lang, agit_id)?;
    let tr: CreateArtworkPageTranslate = translate(&lang);
    let mut title = use_signal(String::default);

    rsx! {
        div { class: "w-full flex flex-col gap-14",
            TitleInput {
                on_go_back: move |_| { ctrl.handle_go_back() },
                placeholder: "Input Title".to_string(),
                value: title(),
                on_change: move |value| {
                    title.set(value);
                },
                disabled: ctrl.active_tab() != Tab::ItemDetails,
            }
            div { class: "flex flex-col gap-10 h-full overflow-y-scroll",
                TabHeader {
                    lang,
                    selected: ctrl.active_tab(),
                    on_click: move |tab| {
                        tracing::debug!("Tab clicked: {:?}", tab);
                        ctrl.set_active_tab(tab);
                    },
                }
                if ctrl.active_tab() == Tab::ItemDetails {
                    ItemDetailTab {
                        lang,
                        on_save: move |result: ItemDetailResult| {
                            ctrl.save_item_detail(title(), result);
                        },
                        on_next: move |_| {
                            tracing::debug!("Next button clicked");
                            ctrl.set_active_tab(Tab::ArtInfo);
                        },
                    }
                } else if ctrl.active_tab() == Tab::ArtInfo {
                    ArtInfo {
                        lang,
                        on_save: Callback::new(move |(image_url, description)| async move {
                            let res = ctrl.create_artwork(image_url, description).await;
                        }),
                        on_back: move |_| {
                            ctrl.set_active_tab(Tab::ItemDetails);
                        },
                    }
                }
            }
        }
    }
}

#[derive(Debug, Default, Eq, PartialEq, Clone)]
pub struct ItemDetailResult {
    pub display_name: String,
    pub ways_to_sell: WaysToSell,
    pub rarity: Option<Rarity>,
    pub stock: Option<i64>,
    pub price: Currency,
    pub collection: Option<String>,
    pub medium: Medium,
    pub theme: Theme,
    pub year: i64,
    pub royalty: Royalty,
    pub lockup_started_at: Option<i64>,
    pub lockup_ended_at: Option<i64>,
    pub art_style: Vec<ArtStyle>,
    pub material: Vec<Material>,
    pub color: Vec<String>,
    pub size: Size,
    pub weight: Weight,
}
#[component]
fn ItemDetailTab(
    lang: Language,
    on_save: EventHandler<ItemDetailResult>,
    on_next: EventHandler<()>,
) -> Element {
    let tr: CreateArtworkPageTranslate = translate(&lang);

    let mut display_name = use_signal(String::default);

    let mut ways_to_sell = use_signal(|| None::<WaysToSell>);
    let mut rarity = use_signal(|| None::<Rarity>);
    let mut stock = use_signal(String::default);
    let mut currency = use_signal(|| None::<Currency>);

    let mut medium = use_signal(|| None::<Medium>);
    let mut theme = use_signal(|| None::<Theme>);
    let mut year = use_signal(String::default);

    rsx! {
        div { class: "flex flex-col gap-20",
            Accordion { label: tr.artist_info.to_string(),
                Label {
                    label: "Display Name or Email".to_string(),
                    required: true,
                    div { class: "w-110",
                        Input {
                            placeholder: "Input".to_string(),
                            value: display_name(),
                            on_change: move |v| {
                                tracing::debug!("Input changed: {}", v);
                                display_name.set(v);
                            },
                        }
                    }
                }
            }
            Accordion { label: tr.sale_info.to_string(),
                Label { label: tr.ways_to_sell.to_string(), required: true,
                    DropDown {
                        id: tr.ways_to_sell.to_string(),
                        options: WaysToSell::VARIANTS.iter().map(|v| v.translate(&lang).to_string()).collect(),
                        placeholder: "Select",
                        onchange: move |selected: String| {
                            WaysToSell::from_str(&selected)
                                .map(|v| {
                                    ways_to_sell.set(Some(v));
                                })
                                .unwrap_or_else(|_| {
                                    tracing::error!("Failed to parse selected ways to sell: {}", selected);
                                });
                        },
                    }
                }
                Label { label: tr.rarity.to_string(),
                    DropDown {
                        id: tr.rarity.to_string(),
                        options: Rarity::VARIANTS.iter().map(|v| v.translate(&lang).to_string()).collect(),
                        placeholder: "Select",
                        onchange: move |selected: String| {
                            Rarity::from_str(&selected)
                                .map(|v| {
                                    rarity.set(Some(v));
                                })
                                .unwrap_or_else(|_| {
                                    tracing::error!("Failed to parse selected rarity: {}", selected);
                                });
                        },
                    }
                }
                Label { label: tr.stock.to_string(),
                    Input {
                        placeholder: "Input".to_string(),
                        value: stock(),
                        on_change: move |v| {
                            tracing::debug!("Input changed: {}", v);
                            stock.set(v);
                        },
                    }
                }
                Label { label: tr.price.to_string(), required: true,
                    DropDown {
                        id: tr.price.to_string(),
                        options: Currency::iter().map(|v| format!("{:?}", v)).collect(),
                        placeholder: "Select",
                        onchange: move |selected: String| {
                            Currency::from_str(&selected)
                                .map(|v| {
                                    currency.set(Some(v));
                                })
                                .unwrap_or_else(|_| {
                                    tracing::error!("Failed to parse selected currency: {}", selected);
                                });
                        },
                    }
                }
                Label { label: tr.lock_up_period.to_string() }
            }
            Accordion { label: tr.attributes.to_string(),
                Label { label: tr.collection.to_string(),
                    DropDown {
                        id: tr.collection.to_string(),
                        options: vec![],
                        placeholder: "Select",
                        onchange: move |selected: String| {
                            tracing::debug!("Selected Collection: {}", selected);
                        },
                    }
                }
                Label { label: tr.medium.to_string(), required: true,
                    DropDown {
                        id: tr.medium.to_string(),
                        options: Medium::VARIANTS.iter().map(|v| v.translate(&lang).to_string()).collect(),
                        placeholder: "Select",
                        onchange: move |selected: String| {
                            Medium::from_str(&selected)
                                .map(|v| {
                                    medium.set(Some(v));
                                })
                                .unwrap_or_else(|_| {
                                    tracing::error!("Failed to parse selected medium: {}", selected);
                                });
                        },
                    }
                }
                Label { label: tr.theme.to_string(), required: true,
                    DropDown {
                        id: tr.theme.to_string(),
                        options: Theme::VARIANTS.iter().map(|v| v.translate(&lang).to_string()).collect(),
                        placeholder: "Select",
                        onchange: move |selected: String| {
                            Theme::from_str(&selected)
                                .map(|v| {
                                    theme.set(Some(v));
                                })
                                .unwrap_or_else(|_| {
                                    tracing::error!("Failed to parse selected theme: {}", selected);
                                });
                        },
                    }
                }
                Label { label: tr.year.to_string(), required: true,
                    Input {
                        placeholder: "YYYY".to_string(),
                        value: year(),
                        on_change: move |v| {
                            tracing::debug!("year changed: {}", v);
                            year.set(v);
                        },
                    }
                }
            }
            div { class: "flex flex-row justify-end gap-5",
                SecondaryButton {
                    class: "text-white",
                    onclick: move |_| {
                        on_save
                            .call(ItemDetailResult {
                                display_name: display_name(),
                                ways_to_sell: ways_to_sell().unwrap_or_default(),
                                rarity: rarity().clone(),
                                stock: if stock().is_empty() {
                                    None
                                } else {
                                    Some(stock().parse().unwrap_or_default())
                                },
                                price: currency().unwrap_or(Currency::ETH),
                                collection: None,
                                medium: medium().unwrap_or_default(),
                                theme: theme().unwrap_or_default(),
                                year: year().parse().unwrap_or_default(),
                                ..Default::default()
                            });
                    },
                    div { class: "flex flex-row gap-2.5 w-30",
                        Save { class: "[&>path]:stroke-white" }
                        span { class: "flex-1", {tr.save} }
                    }
                }
                SecondaryButton {
                    class: "text-white",
                    onclick: move |_| {
                        on_next.call(());
                    },
                    div { class: "flex flex-row gap-2.5 w-30",
                        span { class: "flex-1", {tr.images} }
                        ChevronRight { class: "[&>path]:stroke-white" }
                    }
                }
            }
        }
    }
}

#[component]
pub fn ArtInfo(
    lang: Language,
    on_save: EventHandler<(String, Option<String>)>,
    on_back: EventHandler<()>,
) -> Element {
    let tr: CreateArtworkPageTranslate = translate(&Language::default());
    let mut image_url = use_signal(String::default);
    let mut description = use_signal(|| None::<String>);
    rsx! {
        div { class: "flex flex-col gap-20",
            Accordion { label: tr.art_info.to_string() }
            Accordion { label: tr.description.to_string(),
                textarea {
                    class: "text-[15px]/[23px] border border-neutral-80 px-4 py-3 outline-none text-white hover:border-primary focus:border-primary aria-invalid:border-pink placeholder-neutral-80 disabled:!border-neutral-80",
                    placeholder: tr.description_placeholder.to_string(),
                    value: description(),

                    oninput: move |e| {
                        tracing::debug!("Description input changed: {}", e.value());
                        description.set(Some(e.value().clone()));
                    },
                }
            }
            div { class: "flex flex-row justify-end gap-5",
                SecondaryButton {
                    class: "text-white",
                    onclick: move |_| {
                        on_back.call(());
                    },
                    div { class: "flex flex-row gap-2.5 w-30",
                        ChevronLeft { class: "[&>path]:stroke-white" }
                        span { class: "flex-1", {tr.back} }


                    }
                }
                SecondaryButton {
                    class: "text-white",
                    onclick: move |_| {
                        on_save.call((image_url(), description()));
                    },
                    div { class: "flex flex-row gap-2.5 w-30",
                        Save { class: "[&>path]:stroke-white" }
                        span { class: "flex-1", {tr.save} }
                    }
                }
            }
        }
    }
}
#[component]
pub fn Accordion(
    label: String,
    #[props(default = VNode::empty())] children: Element,
    #[props(default = true)] default_open: bool,
) -> Element {
    let mut expand = use_signal(|| default_open);

    rsx! {
        div {
            class: "flex group flex-col pb-2.5 text-white",
            aria_expanded: !expand(),
            div {
                class: "flex flex-row border-b border-neutral-80",
                onclick: move |_| {
                    expand.toggle();
                },
                p { class: "font-bold text-xl", {label} }
                div { class: "group-aria-expanded:rotate-180 transition-transform duration-300 ease-in-out",
                    ShapeArrowUp { class: "size-8 [&>path]:stroke-white [&>path]:fill-white" }
                }
            }
            div { class: "flex flex-col group-aria-expanded:hidden py-5 gap-2.5", {children} }
        }
    }
}

#[component]
fn Label(label: String, #[props(default = false)] required: bool, children: Element) -> Element {
    rsx! {
        div { class: "flex flex-row items-center gap-6",
            p { class: "w-55 shrink-0",
                if required {
                    span { class: "text-red-500", "*" }
                }
                {label}
            }
            div { class: "w-55 shrink-0", {children} }

        }
    }
}
