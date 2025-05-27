use crate::components::{button::SecondaryButton, toggle_switch::ToggleSwitch};
use bdk::prelude::*;
use by_components::icons::arrows::ChevronDown;

#[component]
pub fn EditRowModal(
    lang: Language,
    on_close: EventHandler<()>,
    on_save: EventHandler<(Vec<SalesMenuType>, Vec<AttributesMenuType>)>,
) -> Element {
    let tr: EditRowModalTranslate = translate(&lang);
    let mut selected_sale_menu: Signal<Vec<SalesMenuType>> = use_signal(|| vec![]);
    let mut selected_attributes_menu: Signal<Vec<AttributesMenuType>> = use_signal(|| vec![]);

    let sum = selected_sale_menu().len();
    let disabled = sum > 8;
    let mut handle_selected_sale_menu = move |menu: &SalesMenuType| {
        if selected_sale_menu().contains(&menu) {
            selected_sale_menu.write().retain(|m| m != menu);
        } else {
            selected_sale_menu.write().push(*menu);
        }
    };

    let mut handle_selected_attributes_menu = move |menu: &AttributesMenuType| {
        if selected_attributes_menu().contains(&menu) {
            selected_attributes_menu.write().retain(|m| m != menu);
        } else {
            selected_attributes_menu.write().push(*menu);
        }
    };

    let mut expand = use_signal(|| true);
    rsx! {
        div { class: "text-white text-sm mt-2.5 flex flex-col gap-9 w-140",
            div { class: "flex flex-col",
                p { {tr.limit} }
                p { {tr.description} }
            }
            div { class: "flex flex-col",
                Accordion {
                    label: "Sale",
                    expand: expand(),
                    on_expand: move |_| {
                        expand.toggle();
                    },
                    for menu in SalesMenuType::VARIANTS.iter() {
                        Menu {
                            title: menu.translate(&lang),
                            description: menu.description(),
                            selected: selected_sale_menu().contains(menu),
                            on_change: move |_| {
                                handle_selected_sale_menu(menu);
                            },
                        }
                    }
                }

                Accordion {
                    label: "Attributes",
                    expand: !expand(),
                    on_expand: move |_| {
                        expand.toggle();
                    },
                    for menu in AttributesMenuType::VARIANTS.iter() {
                        Menu {
                            title: menu.translate(&lang),
                            description: menu.description(),
                            selected: selected_attributes_menu().contains(menu),
                            on_change: move |_| {
                                handle_selected_attributes_menu(menu);
                            },
                        }
                    }
                }
            }
            if !disabled {
                p { class: "text-sm", {tr.selected.replace("{}", &sum.to_string())} }
            } else {
                p { class: "text-sm text-error", {tr.selected_limit} }
            }
            div { class: "w-full flex flex-row gap-5",
                SecondaryButton {
                    class: "flex-1",
                    onclick: move |_| {
                        on_close.call(());
                    },
                    {tr.cancel}
                }
                SecondaryButton {
                    class: "flex-1",
                    disabled,
                    onclick: move |_| {
                        on_save.call((selected_sale_menu().clone(), selected_attributes_menu().clone()));
                    },
                    {tr.save}
                }
            }
        }
    }
}

#[component]
fn Accordion(
    expand: bool,
    on_expand: EventHandler<()>,
    label: String,
    children: Element,
) -> Element {
    rsx! {
        div { class: "flex group flex-col", aria_expanded: !expand,
            div {
                class: "flex flex-row py-3.5 border-b border-transparent group-aria-expanded:border-neutral-80 mb-5",
                onclick: move |_| {
                    on_expand.call(());
                },
                p { class: "text-sm text-neutral-70 w-full flex justify-between items-center",
                    {label}
                }
                div { class: "group-aria-expanded:rotate-180 transition-transform duration-300 ease-in-out",
                    ChevronDown {
                        width: "18px",
                        height: "18px",
                        class: "[&>path]:stroke-white",
                    }
                }
            }
            div { class: "flex flex-col group-aria-expanded:hidden [&>div]:border-b [&>div]:border-neutral-80 max-h-125 overflow-y-scroll",

                {children}
            }
        


        }
    }
}

#[component]
fn Menu(
    title: String,
    description: String,
    selected: bool,
    on_change: EventHandler<bool>,
) -> Element {
    rsx! {
        div {
            class: "flex px-4.5 py-3.5 items-center justify-between",
            onclick: move |_| {
                on_change.call(!selected);
            },
            div { class: "flex flex-col",
                p { class: "text-base text-white", {title} }
                p { class: "text-sm text-white/45", {description} }
            }
            ToggleSwitch { value: selected, on_change: move |_| {} }
        }
    }
}
translate! {
    EditRowModalTranslate;

    title: {
        en: "Edit Top Row Items",
        ko: "상단 행 항목 편집",
    },
    limit: {
        en: "You can select up to 8 items only.",
        ko: "테이블의 상단 행 항목을 편집합니다.",
    },
    description: {
        en: "'Title' and 'Status' cannot be excluded from the items.",
        ko: "'제목'과 '상태'는 항목에서 제외할 수 없습니다.",
    },
    selected: {
        en: "{} items have been selected.",
        ko: "{}개 항목이 선택되었습니다.",
    },
    selected_limit: {
        en: "You cannot select more than 8 items.",
        ko: "8개 이상의 항목을 선택할 수 없습니다.",
    },
    cancel: {
        en: "Cancel",
        ko: "취소",
    }
    save: {
        en: "Save",
        ko: "저장",
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Translate)]
pub enum SalesMenuType {
    #[translate(ko = "Rank", en = "Rank")]
    Rank,
    #[translate(ko = "Owner", en = "Owner")]
    Owner,
    #[translate(ko = "Current Price", en = "Current Price")]
    CurrentPrice,
    #[translate(ko = "Average Price", en = "Average Price")]
    AveragePrice,
    #[translate(ko = "Price Change", en = "Price Change")]
    PriceChange,
    #[translate(ko = "Floor Price", en = "Floor Price")]
    FloorPrice,
    #[translate(ko = "Floor Change", en = "Floor Change")]
    FloorChange,
    #[translate(ko = "Sales Volume", en = "Sales Volume")]
    SalesVolume,
    #[translate(ko = "Volume Change", en = "Volume Change")]
    VolumeChange,
    #[translate(ko = "Ways to Sell", en = "Ways to Sell")]
    WaysToSell,
    #[translate(ko = "Rarity", en = "Rarity")]
    Rarity,
    #[translate(ko = "Stock", en = "Stock")]
    Stock,
    #[translate(ko = "Royalty", en = "Royalty")]
    Royalty,
}

impl SalesMenuType {
    pub fn description(&self) -> String {
        match self {
            SalesMenuType::Rank => {
                "Please input passenger's name or delete this field.".to_string()
            }
            SalesMenuType::Owner => {
                "Please input passenger's name or delete this field.".to_string()
            }
            SalesMenuType::CurrentPrice => {
                "Please input passenger's name or delete this field.".to_string()
            }
            SalesMenuType::AveragePrice => {
                "Please input passenger's name or delete this field.".to_string()
            }
            SalesMenuType::PriceChange => {
                "Please input passenger's name or delete this field.".to_string()
            }
            SalesMenuType::FloorPrice => {
                "Please input passenger's name or delete this field.".to_string()
            }
            SalesMenuType::FloorChange => {
                "Please input passenger's name or delete this field.".to_string()
            }
            SalesMenuType::SalesVolume => {
                "Please input passenger's name or delete this field.".to_string()
            }
            SalesMenuType::VolumeChange => {
                "Please input passenger's name or delete this field.".to_string()
            }
            SalesMenuType::WaysToSell => {
                "Please input passenger's name or delete this field.".to_string()
            }
            SalesMenuType::Rarity => {
                "Please input passenger's name or delete this field.".to_string()
            }
            SalesMenuType::Stock => {
                "Please input passenger's name or delete this field.".to_string()
            }
            SalesMenuType::Royalty => {
                "Please input passenger's name or delete this field.".to_string()
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Translate)]
pub enum AttributesMenuType {
    #[translate(ko = "Collection", en = "Collection")]
    Collection,
    #[translate(ko = "Medium", en = "Medium")]
    Medium,
    #[translate(ko = "Material", en = "Material")]
    Material,
}

impl AttributesMenuType {
    pub fn description(&self) -> String {
        match self {
            AttributesMenuType::Collection => {
                "Please input passenger's name or delete this field.".to_string()
            }
            AttributesMenuType::Medium => {
                "Please input passenger's name or delete this field.".to_string()
            }
            AttributesMenuType::Material => {
                "Please input passenger's name or delete this field.".to_string()
            }
        }
    }
}
