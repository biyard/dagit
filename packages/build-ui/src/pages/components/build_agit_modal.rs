use bdk::prelude::*;

use crate::{
    components::{button::SecondaryButton, input::Input},
    pages::i18n::BuildAgitModalTranslate,
};

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct BuildAgitResult {
    pub name: String,
    // pub agree_term_at: Option<i64>,
    // pub agree_receive_ads_at: Option<i64>,
}

//FIXME: add validation
#[component]
pub fn BuildAgitModal(on_button_click: EventHandler<BuildAgitResult>, lang: Language) -> Element {
    let tr: BuildAgitModalTranslate = translate(&lang);
    let mut name = use_signal(String::new);
    // let mut agree_term = use_signal(|| None::<i64>);
    // let mut agree_receive_ads = use_signal(|| None::<i64>);

    let disabled = name().is_empty(); // || agree_term().is_none();
    let base_url = crate::config::get().base_url;
    let url = format!("{base_url}/{}", name().replace(" ", "-").to_lowercase());
    rsx! {
        div { class: "flex flex-col gap-9",
            h2 { class: "text-sm font-bold", {tr.description} }
            div { class: "flex flex-col gap-5",
                Input {
                    label: tr.name_label,
                    placeholder: tr.name_placeholder,
                    value: name(),
                    on_change: move |value| {
                        name.set(value);
                    },
                }
                Input {
                    label: tr.short_label,
                    value: url,
                    on_change: move |_| {},
                    disabled: true,
                }
                // CheckBoxWithLabel {
                //     label: tr.terms,
                //     // checked: agree_term().is_some(),
                //     on_change: move |checked| {
                //         if checked {
                //             agree_term.set(Some(chrono::Utc::now().timestamp()));
                //         } else {
                //             agree_term.set(None);
                //         }
                //     },
                // }
                // CheckBoxWithLabel {
                //     label: tr.ad_terms,
                //     // checked: agree_receive_ads().is_some(),
                //     on_change: move |checked| {
                //         if checked {
                //             agree_receive_ads.set(Some(chrono::Utc::now().timestamp()));
                //         } else {
                //             agree_receive_ads.set(None);
                //         }
                //     },
                // }
                SecondaryButton {
                    disabled,
                    onclick: move |_| {
                        on_button_click(BuildAgitResult { name: name() });
                    },
                    {tr.build_your_agit}
                }
            }
        }
    }
}
