use bdk::prelude::*;

use crate::components::{button::SecondaryButton, checkbox::CheckBoxWithLabel, input::Input};

#[component]
pub fn DeleteArtworkModal(
    lang: Language,
    artwork_title: String,
    on_close: EventHandler<()>,
    on_delete: EventHandler<()>,
) -> Element {
    let tr: DeleteArtworkModalTranslate = translate(&lang);
    let mut input_value = use_signal(|| "".to_string());
    let mut checked = use_signal(|| false);
    rsx! {
        div { class: "text-white text-sm mt-2.5 flex flex-col gap-9 w-100",
            p { class: "whitespace-pre-wrap", {tr.description} }
            div { class: "gap-5 flex flex-col",
                Input {
                    placeholder: artwork_title.clone(),
                    value: input_value(),
                    on_change: move |value| input_value.set(value),
                }
                CheckBoxWithLabel {
                    label: tr.terms,
                    on_change: move |v| {
                        checked.set(v);
                    },
                }
                div { class: "flex justify-between gap-5",
                    SecondaryButton { class: "flex-1", onclick: move |_| on_close.call(()), {tr.cancel} }
                    SecondaryButton {
                        class: "flex-1",
                        disabled: input_value() != artwork_title || !checked(),
                        onclick: move |_| on_delete.call(()),
                        {tr.delete}
                    }
                }
            }
        }
    }
}

translate! {
    DeleteArtworkModalTranslate;

    title: {
        en: "Delete the Artwork",
        ko: "Artwork 삭제",
    },
    description: {
        en: "Please note that deleted artwork cannot be restored. \n Enter the title of the artwork you want to delete.",
        ko: "삭제된 Artwork는 복원할 수 없습니다. \n 삭제할 Artwork의 제목을 입력하세요.",
    },
    terms: {
        en: "I have read and accept the Terms of Service.",
        ko: "서비스 약관을 읽고 동의합니다.",
    },
    cancel: {
        en: "Cancel",
        ko: "취소",
    },
    delete: {
        en: "Delete",
        ko: "삭제",
    },
}
