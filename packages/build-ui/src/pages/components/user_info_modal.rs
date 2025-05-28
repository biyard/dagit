use bdk::prelude::*;

use crate::{
    components::{button::SecondaryButton, checkbox::CheckBoxWithLabel, input::InputWithLabel},
    pages::i18n::UserInfoModalTranslate,
};

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct UserInfoResult {
    pub nickname: String,
    pub email: String,
    pub terms_agreed_at: i64,
    pub ads_agreed_at: Option<i64>,
}

//FIXME: add validation
#[component]
pub fn UserInfoModal(
    on_button_click: EventHandler<UserInfoResult>,
    lang: Language,
    nickname: Option<String>,
    email: Option<String>,
) -> Element {
    let tr: UserInfoModalTranslate = translate(&lang);
    let mut nickname = use_signal(|| nickname.unwrap_or_default());
    let mut email = use_signal(|| email.unwrap_or_default());
    let mut agree_term = use_signal(|| None::<i64>);
    let mut agree_receive_ads = use_signal(|| None::<i64>);

    let disabled = nickname().is_empty() || email().is_empty() || agree_term().is_none();

    rsx! {
        div { class: "flex flex-col gap-9",
            h2 { class: "text-sm font-bold", {tr.description} }
            div { class: "flex flex-col gap-5",
                InputWithLabel {
                    label: tr.name_label,
                    placeholder: tr.name_placeholder,
                    value: nickname(),
                    on_change: move |value| {
                        nickname.set(value);
                    },
                }
                InputWithLabel {
                    label: tr.email_label,
                    placeholder: tr.email_placeholder,
                    value: email(),
                    on_change: move |value| {
                        email.set(value);
                    },
                }
                CheckBoxWithLabel {
                    label: tr.terms,
                    // checked: agree_term().is_some(),
                    on_change: move |checked| {
                        if checked {
                            agree_term.set(Some(chrono::Utc::now().timestamp()));
                        } else {
                            agree_term.set(None);
                        }
                    },
                }
                CheckBoxWithLabel {
                    label: tr.ad_terms,
                    // checked: agree_receive_ads().is_some(),
                    on_change: move |checked| {
                        if checked {
                            agree_receive_ads.set(Some(chrono::Utc::now().timestamp()));
                        } else {
                            agree_receive_ads.set(None);
                        }
                    },
                }
                SecondaryButton {
                    disabled,
                    onclick: move |_| {
                        if nickname().is_empty() || email().is_empty() || agree_term().is_none() {
                            return;
                        }
                        on_button_click(UserInfoResult {
                            nickname: nickname(),
                            email: email(),
                            terms_agreed_at: agree_term().unwrap(),
                            ads_agreed_at: agree_receive_ads(),
                        });
                    },
                    {tr.finish_signup}
                }
            }
        }
    }
}
