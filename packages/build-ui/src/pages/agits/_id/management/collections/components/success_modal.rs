use bdk::prelude::*;

use crate::{
    components::button::SecondaryButton,
    pages::agits::_id::management::collections::i18n::SuccessModalTranslate,
};

#[component]
pub fn SuccessModal(
    collection_name: String,
    on_confirm: EventHandler<()>,
    lang: Language,
) -> Element {
    let tr: SuccessModalTranslate = translate(&lang);

    rsx! {
        div { class: "max-w-md flex flex-col w-full",
            // Modal body
            div { class: "py-2 pb-4",
                p { class: "text-white", {tr.description} }
            }
            // Modal footer
            div { class: "flex  items-center  pt-4 border-border-primary",
                SecondaryButton {
                    onclick: move |_| on_confirm.call(()),
                    disabled: false,
                    class: "flex-1 px-6 py-2 bg-white text-sm text-black hover:bg-gray-200 min-w[120px]",
                    {tr.confirm_btn_text}
                }
            }
        }
    }
}
