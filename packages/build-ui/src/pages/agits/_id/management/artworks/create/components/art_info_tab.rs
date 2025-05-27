// use crate::components::button::ButtonWithIcon;
use crate::components::image_upload::FileUpload;
use crate::components::input::TextArea;
use crate::pages::agits::_id::management::artworks::controller::Controller;
use crate::pages::agits::_id::management::artworks::create::i18n::CreateArtworkPageTranslate;
use bdk::prelude::by_components::icons::{arrows, other_devices};
use bdk::prelude::*;

#[component]
pub fn ArtInfoTab(lang: Language, agit_id: ReadOnlySignal<i64>) -> Element {
    let mut ctrl = Controller::new(lang, agit_id)?;
    let tr: CreateArtworkPageTranslate = translate(&lang);
    rsx! {
        div { class: "flex flex-col mt-8",
            div { class: "flex flex-row items-center pb-5 mb-1",
                span { class: "text-red-500 mr-1", "*" }
                label { class: "text-sm ", "Detail Images" }
            }
            FileUpload { onclick: move |_| {}, label: tr.upload_img_label }
            p { class: "text-xs text-gray-500 mt-1", {tr.max_file_size} }
            p { class: "text-xs text-gray-500", {tr.max_file_size_2} }
        }

        // Introduction Section
        div { class: "mb-8 mt-8",
            TextArea {
                label: tr.description_label,
                placeholder: tr.description_placeholder,
                value: ctrl.artwork_input_field().description.clone(),
                on_change: move |evt: String| {
                    ctrl.update_artwork_field("description".to_string(), evt.clone())
                },
            }
        }
        // Action buttons
        div { class: "flex justify-end space-x-4 mt-8" }
    }
}
