use crate::{
    components::{input::BottomBorderInput, tab_button::TabButton},
    pages::agits::_id::management::artworks::{
        controller::Controller,
        create::{ItemDetailsTab, components::ArtInfoTab, i18n::CreateArtworkPageTranslate},
    },
};
use bdk::prelude::{by_components::icons::security, *};

#[derive(Clone, Debug, PartialEq)]
pub enum ActiveTab {
    ItemDetails,
    ArtInfo,
}
#[component]
pub fn CreateArtworkPage(lang: Language, agit_id: ReadOnlySignal<i64>) -> Element {
    let ctrl = Controller::new(lang, agit_id)?;
    let mut active_tab = use_signal(|| ActiveTab::ItemDetails);
    let _tr: CreateArtworkPageTranslate = translate(&lang);

    rsx! {
        div { class: "w-full min-h-screen bg-background h-full flex text-white items-center",
            div { class: "flex flex-col w-full h-full  p-6",
                div { class: "flex items-center mb-6",
                    div { onclick: move |_| ctrl.open_artwork_page(),
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            class: "h-6 w-6",
                            fill: "none",
                            view_box: "0 0 24 24",
                            stroke: "currentColor",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                d: "M15 19l-7-7 7-7",
                            }
                        }
                    }
                    BottomBorderInput {
                        label: "Artwork name".to_string(),
                        placeholder: "Input Title".to_string(),
                        value: "".to_string(),
                        on_change: move |_value| {},
                        disabled: false,
                        invalid: false,
                        invalid_message: "".to_string(),
                    }
                    div {
                        security::Lock2 {
                            class: "ml-2 [&>path]:stroke-white [&>circle]:stroke-white",
                            height: 20,
                            width: 20,
                        }
                    }
                }
                div { class: "mt-4 mb-8",
                    div { class: "flex space-x-8 text-sm font-medium",
                        TabButton {
                            label: "Item Details".to_string(),
                            is_active: *active_tab.read() == ActiveTab::ItemDetails,
                            onclick: move |_| active_tab.set(ActiveTab::ItemDetails),
                        }
                        TabButton {
                            label: "Artwork Info".to_string(),
                            is_active: *active_tab.read() == ActiveTab::ArtInfo,
                            onclick: move |_| active_tab.set(ActiveTab::ArtInfo),
                        }
                    }
                }


                if *active_tab.read() == ActiveTab::ItemDetails {
                    ItemDetailsTab { lang: lang.clone(), agit_id: agit_id.clone() }
                } else if *active_tab.read() == ActiveTab::ArtInfo {
                    ArtInfoTab { lang: lang.clone(), agit_id: agit_id.clone() }
                }
            }
        }
    }
}
