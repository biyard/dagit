use bdk::prelude::*;

use crate::pages::agits::_id::management::artists::i18n::ConfirmRemoveArtistModalTranslate;

#[component]
pub fn ConfirmRemoveArtistModal(
    on_back: EventHandler<()>,
    on_remove: EventHandler<()>,
    lang: Language,
) -> Element {
    let mut terms_accepted = use_signal(|| false);
    let tr: ConfirmRemoveArtistModalTranslate = translate(&lang);

    rsx! {
        div { class: "flex flex-col w-full max-w-md",
            // Modal header

            div { class: "py-6",
                p { class: "text-white", {tr.popup_description} }
                p { class: "text-white pt-5", {tr.popup_description_2} }
            }
            div { class: "pt-0 flex items-start",
                input {
                    id: "announcements",
                    class: "mr-2  h-4 w-4 border border-btn-signin checked:bg-white checked:border-black checked:text-black text-sm",
                    r#type: "checkbox",
                    checked: "{terms_accepted}",
                    oninput: move |evt| terms_accepted.set(evt.value().parse().unwrap_or(false)),
                }
                label { class: "text-sm text-popup-label mb-2", {tr.popup_confirm_text} }
            }

            // Modal footer
            div { class: "flex items-center justify-between gap-4 p-6 border-border-primary",
                button {
                    class: "px-10 py-2 text-l text-popup-text hover:text-white border border-white",
                    onclick: move |_| on_back.call(()),
                    {tr.cancel_btn}
                }
                button {
                    class: "px-10 py-3 text-l bg-white  text-black hover:bg-gray-200",
                    onclick: move |_| {
                        on_remove.call(());
                    },
                    {tr.confirm_btn}
                }
            }
        }
    }
}
