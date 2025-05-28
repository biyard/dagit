use bdk::prelude::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Translate)]
pub enum Tab {
    #[translate(en = "Item Details", ko = "Item Details")]
    ItemDetails,
    #[translate(en = "Art Info", ko = "Art Info")]
    ArtInfo,
}

#[component]
pub fn TabHeader(lang: Language, selected: Tab, on_click: EventHandler<Tab>) -> Element {
    rsx! {
        div { class: "flex flex-row",
            {
                Tab::VARIANTS
                    .iter()
                    .map(|tab| {
                        let label = tab.translate(&lang);
                        rsx! {
                            div {
                                aria_selected: *tab == selected,
                                class: "min-w-40 text-base font-light text-center text-neutral-70 cursor-pointer border-b border-transparent pb-2.5 hover:bg-gray-200 aria-selected:border-white aria-selected:font-semibold aria-selected:text-white",
                                onclick: move |_| {
                                    on_click.call(*tab);
                                },
                                {label}
                            }
                        }
                    })
            }
        }
    }
}
