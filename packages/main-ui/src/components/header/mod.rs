use by_components::icons;
use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_translate::Language;

use crate::components::icons::dagit_logo::ServiceLogo;
use crate::routes::Route;
#[component]
pub fn Header(lang: Language) -> Element {
    rsx! {
        header { class: "z-100 top-0 fixed bg-gradient-to-t from-white/0 via-white/70 to-white/100 w-full flex text-black font-medium text-base justify-center",
            div { class: "w-full max-w-[1440px] flex justify-between items-center gap-10 h-10 my-6.5",
                div { class: "flex justify-center items-center max-w-[1000px] h-full",
                    Link {
                        class: "flex flex-row items-center justify-around gap-1 h-full",
                        to: Route::MainPage {
                            lang: lang.clone(),
                        },
                        ServiceLogo {}
                    }
                    Search {
                        placeholder: "Search by artist, gallery, meta tag...",
                        onchange: |value| {
                            tracing::debug!("value {:?}", value);
                        },
                    }
                    //TODO: Add more menus
                    div { class: "flex gap-10 whitespace-nowrap",
                        Link {
                            to: Route::MainPage {
                                lang: lang.clone(),
                            },
                            "Agits"
                        }
                        Link {
                            //TODO: Change Target
                            to: Route::MainPage {
                                lang: lang.clone(),
                            },
                            "Arts"
                        }
                        Link {
                            //TODO: Change Target
                            to: Route::CollectionsPage{
                                lang: lang.clone(),
                            },
                            "Fair & Shows"
                        }
                        Link {
                            class: "text-[#18b583] font-bold",
                            //TODO: Change Target
                            to: Route::MainPage {
                                lang: lang.clone(),
                            },
                            "Platform"
                        }
                    }
                }
                Link {
                    class: "text-center self-center",
                    to: Route::MainPage {
                        lang: lang.clone(),
                    },
                    "Login"
                }
            }
        }
    }
}

#[component]
pub fn Search(placeholder: String, onchange: EventHandler<String>) -> Element {
    let mut value = use_signal(String::default);

    rsx! {
        div { class: "flex justify-left items-center h-10 rounded-full border border-[#E6E6E6] bg-white px-4 py-2 min-w-[450px]",
            icons::edit::Search {}
            input {
                class: "flex-1 ml-2 text-[#bebebe] text-sm font-medium leading-snug",
                placeholder,
                value: value(),
                r#type: "text",
                onchange: move |evt: FormEvent| {
                    let v = evt.data().value();
                    value.set(v.clone());
                    onchange.call(v);
                },
            }
        }
    }
}
