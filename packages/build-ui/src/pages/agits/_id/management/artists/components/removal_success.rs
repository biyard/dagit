use bdk::prelude::*;

use crate::pages::agits::_id::management::artists::i18n::RemovalSuccessModalTranslate;

#[component]
pub fn RemovalSuccessModal(
    on_back: EventHandler<()>,
    on_confirm: EventHandler<()>,
    lang: Language,
) -> Element {
    let tr: RemovalSuccessModalTranslate = translate(&lang);

    rsx! {
        div { class: "flex flex-col w-full max-w-md",
            // Modal body
            div { class: "py-2 pb-4",
                p { class: "text-white", {tr.success_popup_description} }
            }
            // Modal footer
            div { class: "flex items-center pt-4 border-border-primary",
                button {
                    class: "flex-1 px-6 py-2 bg-white text-sm text-black hover:bg-gray-200 min-w[120px]",
                    onclick: move |_| on_confirm.call(()),
                    {tr.confirm_btn}
                }
            }
        }
    }
}
