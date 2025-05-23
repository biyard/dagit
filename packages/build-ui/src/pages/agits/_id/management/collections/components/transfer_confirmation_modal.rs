use bdk::prelude::*;

use crate::{
    components::button::{PrimaryButton, SecondaryButton},
    pages::agits::_id::management::collections::i18n::TransferConfirmationModalTranslate,
};
#[component]
pub fn TransferConfirmationModal(
    selected_count: usize,
    on_back: EventHandler<()>,
    on_continue: EventHandler<String>, // Changed to pass the collection name
    lang: Language,
) -> Element {
    let tr: TransferConfirmationModalTranslate = translate(&lang);

    rsx! {
        div { class: "flex flex-col w-full max-w-md",
            div { class: "",
                p { class: "text-white", "The {selected_count} {tr.description}" }
            }
            // Modal footer
            div { class: "flex items-center justify-between gap-4 pt-6 pb-4 border-border-primary",
                PrimaryButton {
                    onclick: move |_| {
                        on_back.call(());
                    },
                    disabled: false,
                    {tr.back_btn_text}
                }
                SecondaryButton {
                    onclick: move |_| {
                        on_continue.call(String::new());
                    },
                    disabled: false,
                    class: "text-black bg-white",
                    {tr.continue_btn_text}
                }
            }
                // button {
        //     class: "px-10 py-3 text-l bg-white  text-black hover:bg-gray-200",
        //     onclick: move |_| {
        //         on_continue.call(String::new());
        //     },
        //     {tr.continue_btn_text}
        // }
        }
    }
}
