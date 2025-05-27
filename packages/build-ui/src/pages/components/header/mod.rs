use bdk::prelude::*;
mod i18n;

use super::super::controller::Controller;

use crate::{components::ServiceLogo, routes::Route};
use i18n::HeaderTranslate;
#[component]
pub fn Header(lang: Language) -> Element {
    let tr: HeaderTranslate = translate(&lang);

    let ctrl = Controller::new(lang)?;

    rsx! {
        nav { class: "w-full flex justify-between items-center bg-transparent py-4 text-white text-base px-1 z-100",
            ServiceLogo { width: "110", height: "24", class: "fill-white" }
            div { class: "flex gap-x-10 ",
                Link { to: Route::RootPage { lang }, {tr.solution} }
                Link { to: Route::RootPage { lang }, {tr.pricing} }
                Link { to: Route::RootPage { lang }, {tr.faq} }
            }
            if let Some((user_info, agit)) = ctrl.get_user_info() {
                div { class: "flex flex-row items-center",
                    div { class: "px-7.5 py-2.5 flex justify-center items-center gap-x-2.5",
                        div { class: "rounded-full bg-white size-10 shrink-0 overflow-hidden",
                            if let Some(profile_url) = user_info.profile_url {
                                img { src: profile_url }
                            }
                        }
                        span { {user_info.nickname} }
                    }
                    div {
                        Link {
                            class: "bg-transparent border border-white inline-block px-4.5 py-3 text-base text-white",
                            to: Route::HomePage {
                                lang,
                                agit_id: agit.id,
                            },
                            {tr.my_agit}
                        }
                    }
                }
            } else {
                span {
                    onclick: move |_| {
                        ctrl.open_select_blockchain_modal();
                    },
                    {tr.login}
                }
            }
        }
    }
}
