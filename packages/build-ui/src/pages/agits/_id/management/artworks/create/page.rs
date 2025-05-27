#![allow(unused)]
use bdk::prelude::by_components::icons::arrows::ShapeArrowUp;
use bdk::prelude::*;
use common::tables::prelude::{Currency, Medium, Rarity, Theme};

use crate::components::input::Input;
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
                            tracing::debug!("Item detail saved: {:?}", result);
                        },
                        on_next: move |_| {
                            tracing::debug!("Next button clicked");
                            ctrl.set_active_tab(Tab::ArtInfo);
                        },
                    }
                }
            
            }
        
        }
    }
}

#[derive(Debug, Default)]
pub struct ItemDetailResult {
    pub display_name: String,
    pub stock: String,
    pub year: String,
}
#[component]
fn ItemDetailTab(
    lang: Language,
    on_save: EventHandler<ItemDetailResult>,
    on_next: EventHandler<()>,
) -> Element {
    let tr: CreateArtworkPageTranslate = translate(&lang);

    let mut display_name = use_signal(String::default);
    let mut stock = use_signal(String::default);
    let mut year = use_signal(String::default);

    rsx! {
        div { class: "flex flex-col gap-20",
            Accordion { label: "Artist Info".to_string(), default_open: true,
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
            Accordion { label: tr.sale_info.to_string(), default_open: true,
                Label { label: tr.ways_to_sell.to_string(), required: true,
                    DropDown {
                        id: tr.ways_to_sell.to_string(),
                        options: WaysToSell::VARIANTS.iter().map(|v| v.translate(&lang).to_string()).collect(),
                        placeholder: "Select",
                        onchange: move |selected| {
                            tracing::debug!("Selected way to sell: {}", selected);
                        },
                    }
                }
                Label { label: tr.rarity.to_string(),
                    DropDown {
                        id: tr.rarity.to_string(),
                        options: Rarity::VARIANTS.iter().map(|v| v.translate(&lang).to_string()).collect(),
                        placeholder: "Select",
                        onchange: move |selected| {
                            tracing::debug!("Selected rarity: {}", selected);
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
                        onchange: move |selected| {
                            tracing::debug!("Selected price: {}", selected);
                        },
                    }
                }
                Label { label: tr.lock_up_period.to_string() }
            }
            Accordion { label: tr.attributes.to_string(), default_open: true,
                Label { label: tr.collection.to_string(),
                    DropDown {
                        id: tr.collection.to_string(),
                        options: vec![],
                        placeholder: "Select",
                        onchange: move |selected| {
                            tracing::debug!("Selected Collection: {}", selected);
                        },
                    }
                }
                Label { label: tr.medium.to_string(), required: true,
                    DropDown {
                        id: tr.medium.to_string(),
                        options: Medium::VARIANTS.iter().map(|v| v.translate(&lang).to_string()).collect(),
                        placeholder: "Select",
                        onchange: move |selected| {
                            tracing::debug!("Selected Medium: {}", selected);
                        },
                    }
                }
                Label { label: tr.theme.to_string(), required: true,
                    DropDown {
                        id: tr.theme.to_string(),
                        options: Theme::VARIANTS.iter().map(|v| v.translate(&lang).to_string()).collect(),
                        placeholder: "Select",
                        onchange: move |selected| {
                            tracing::debug!("Selected theme: {}", selected);
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
                        on_save.call(ItemDetailResult::default());
                    },
                    {tr.save}
                }
                SecondaryButton {
                    class: "text-white",
                    onclick: move |_| {
                        on_next.call(());
                    },
                    {tr.images}
                }
            }
        }
    }
}
#[component]
pub fn Accordion(
    label: String,
    #[props(default = VNode::empty())] children: Element,
    #[props(default = false)] default_open: bool,
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
