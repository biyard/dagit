use bdk::prelude::*;

use crate::components::ServiceLogo;

// use super::controller::{Blockchain, Wallet};
use super::controller::Controller;
use super::i18n::RootPageTranslate;

#[component]
pub fn RootPage(lang: Language) -> Element {
    let tr: RootPageTranslate = translate(&lang);
    let ctrl = Controller::new(lang)?;
    rsx! {
        div { class: "flex-1 flex flex-col items-center justify-center h-full text-center",
            ServiceLogo { class: "mb-10", width: "249.98", height: "55.31" }

            h2 { class: "text-2xl text-white mb-4 max-w-xl", {tr.description} }
            p { class: "font-light font-aleo text-white mb-8", {tr.sub_description} }

            button {
                class: "bg-teal-500 text-black py-3 px-6 text-sm cursor-pointer hover:bg-teal-600",
                onclick: {
                    move |_| {
                        ctrl.open_select_blockchain_modal();
                    }
                },
                {tr.button_text}
            }
        }
    }
}
