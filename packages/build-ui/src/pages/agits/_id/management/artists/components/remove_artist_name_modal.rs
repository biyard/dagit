use bdk::prelude::*;

use crate::pages::agits::_id::management::artists::i18n::RemoveArtistNameModalTranslate;

#[component]
pub fn RemoveArtistModal(
    on_back: EventHandler<()>,
    on_remove: EventHandler<()>,
    lang: Language,
) -> Element {
    let mut collection_name = use_signal(|| String::new());
    let tr: RemoveArtistNameModalTranslate = translate(&lang);
    let mut terms_accepted = use_signal(|| false);

    rsx! {

        div { class: "flex flex-col w-full max-w-md",
            div { class: "pb-4 mt-5",
                p { class: "text-sm text-white", {tr.sub_title} }
                p { class: "text-sm text-white mb-4", {tr.enter_artist_name} }
                // Collection name input
                div { class: "mb-4",
                    input {
                        class: "w-full bg-transparent border text-white text-sm rounded-none p-2 focus:outline-none focus:border-primary",
                        placeholder: "Artist name",
                        value: "{collection_name}",
                        oninput: move |evt| collection_name.set(evt.value().clone()),
                    }
                }
                // Short URL input
                div { class: "flex items-start",
                    input {
                        id: "announcements",
                        class: "mr-2  h-4 w-4 border border-btn-signin checked:bg-white checked:border-black checked:text-black text-sm",
                        r#type: "checkbox",
                        checked: "{terms_accepted}",
                        oninput: move |evt| terms_accepted.set(evt.value().parse().unwrap_or(false)),
                    }
                    label { class: "text-sm text-popup-label mb-2", {tr.popup_confirm_text} }
                }
            }


            // Modal footer
            div { class: "flex items-center justify-between gap-4 pt-0 border-border-primary",
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
