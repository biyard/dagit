use bdk::prelude::*;

use crate::{
    components::{
        button::{PrimaryButton, SecondaryButton},
        input::Input,
    },
    pages::agits::_id::management::collections::i18n::CollectionNameInputModalTranslate,
};

#[component]
pub fn CollectionNameInputModal(
    on_back: EventHandler<()>,
    on_add: EventHandler<String>, // Changed to pass the collection name
    lang: Language,
) -> Element {
    let mut collection_name = use_signal(|| String::new());
    let tr: CollectionNameInputModalTranslate = translate(&lang);

    rsx! {
        div { class: "flex flex-col",
            // Modal body
            div { class: "pb-5",
                p { class: "text-sm text-white mb-4", {tr.sub_title} }
                // Collection name input
                div { class: "mb-4",
                    Input {
                        label: tr.collection_name,
                        placeholder: tr.collection_name_placeholder,
                        value: "{collection_name}",
                        on_change: move |v| collection_name.set(v),
                    }
                }
                // Short URL input
                div {
                    Input {
                        url_input: "dagit_url".to_string(),
                        label: tr.short_url,
                        placeholder: "(collection)",
                        value: {
                            let name = collection_name.read();
                            name.to_lowercase().replace(" ", "-")
                        },
                        on_change: move |v| collection_name.set(v),
                        readonly: true,
                    }
                }
            }
            // Modal footer
            div { class: "flex items-center justify-end gap-4 p-6 border-t border-border-primary",
                PrimaryButton {
                    onclick: move |_| {
                        on_back.call(());
                    },
                    disabled: false,
                    {tr.back_btn_text}
                }

                SecondaryButton {
                    onclick: move |_| {
                        on_add.call(collection_name.read().clone());
                    },
                    disabled: collection_name.read().is_empty(),
                    {tr.add_btn_text}
                }
            }
        }
    }
}
