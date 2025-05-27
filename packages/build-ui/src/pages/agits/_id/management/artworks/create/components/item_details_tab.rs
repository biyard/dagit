// use crate::components::button::ButtonWithIcon;
use crate::components::{dropdown_input::DropdownInput, input::Input2};
use crate::pages::agits::_id::management::artworks::controller::Controller;
use crate::pages::agits::_id::management::artworks::create::i18n::CreateArtworkPageTranslate;
use bdk::prelude::by_components::icons::other_devices;
use bdk::prelude::*;
use by_components::icons::arrows;

#[component]
pub fn ItemDetailsTab(lang: Language, agit_id: ReadOnlySignal<i64>) -> Element {
    let mut ctrl = Controller::new(lang, agit_id)?;
    let tr: CreateArtworkPageTranslate = translate(&lang);
    rsx! {
        div { class: "mb-8",
            div { class: "flex items-center mb-4",
                h2 { class: "text-xl font-semibold", {tr.artist_info} }
                arrows::ChevronDown {
                    class: "ml-2 [&>path]:stroke-white",
                    height: 20,
                    width: 20,
                }
            }
            hr { class: "mt-4 mb-4 text-neutral-80" }
            div { class: "space-y-4 max-w-4xl",
                Input2 {
                    label: tr.display_name_label,
                    placeholder: tr.display_name_label,
                    value: ctrl.artwork_input_field().display_name.clone(),
                    on_change: move |evt: String| {
                        ctrl.update_artwork_field("display_name".to_string(), evt.clone())
                    },
                }
            }
        }
        div { class: "mb-8 mt-8",
            div { class: "flex items-center mb-4",
                h2 { class: "text-xl font-semibold", {tr.sale_info} }
                arrows::ChevronDown {
                    class: "ml-2 [&>path]:stroke-white",
                    height: 20,
                    width: 20,
                }
            }
            hr { class: "mt-4 mb-4 text-neutral-80" }
            div { class: "space-y-4 max-w-4xl",
                DropdownInput {
                    label: tr.ways_to_sell,
                    placeholder: tr.ways_to_sell,
                    value: ctrl.artwork_input_field().ways_to_sell.clone(),
                    options: vec!["Trans".to_string(), "Offer".to_string()],
                    on_change: move |evt: String| {
                        ctrl.update_artwork_field("ways_to_sell".to_string(), evt.clone())
                    },
                }
                DropdownInput {
                    label: tr.rarity,
                    placeholder: tr.rarity,
                    value: ctrl.artwork_input_field().rarity.clone(),
                    options: vec!["Trans".to_string(), "Offer".to_string()],
                    on_change: move |evt: String| { ctrl.update_artwork_field("rarity".to_string(), evt.clone()) },
                }
                Input2 {
                    width_class: "w-64".to_string(),
                    label: tr.stock,
                    placeholder: tr.stock,
                    value: ctrl.artwork_input_field().stock.clone(),
                    on_change: move |evt: String| { ctrl.update_artwork_field("stock".to_string(), evt.clone()) },
                }
                DropdownInput {
                    label: tr.price,
                    placeholder: tr.price,
                    value: ctrl.artwork_input_field().price.clone(),
                    options: vec!["Trans".to_string(), "Offer".to_string()],
                    on_change: move |evt: String| { ctrl.update_artwork_field("price".to_string(), evt.clone()) },
                }
                DropdownInput {
                    label: tr.royalty,
                    placeholder: tr.royalty,
                    value: ctrl.artwork_input_field().royalty.clone(),
                    options: vec!["Trans".to_string(), "Offer".to_string()],
                    on_change: move |evt: String| { ctrl.update_artwork_field("royalty".to_string(), evt.clone()) },
                }
                DropdownInput {
                    label: tr.lock_up_period,
                    placeholder: tr.lock_up_period,
                    value: ctrl.artwork_input_field().lock_up_period.clone(),
                    options: vec!["Trans".to_string(), "Offer".to_string()],
                    on_change: move |evt: String| {
                        ctrl.update_artwork_field("lock_up_period".to_string(), evt.clone())
                    },
                }
            }
        }
        div { class: "mb-8 mt-8",
            div { class: "flex items-center mb-4",
                h2 { class: "text-xl font-semibold", {tr.attributes} }
                arrows::ChevronDown {
                    class: "ml-2 [&>path]:stroke-white",
                    height: 20,
                    width: 20,
                }
            }
            hr { class: "mt-4 mb-4 text-neutral-80" }
            div { class: "space-y-4 max-w-4xl",
                DropdownInput {
                    label: tr.collection,
                    placeholder: tr.collection,
                    value: ctrl.artwork_input_field().collection.clone(),
                    options: vec!["Trans".to_string(), "Offer".to_string()],
                    on_change: move |evt: String| {
                        ctrl.update_artwork_field("collection".to_string(), evt.clone())
                    },
                }
                DropdownInput {
                    label: tr.medium,
                    placeholder: tr.medium,
                    value: ctrl.artwork_input_field().medium.clone(),
                    options: vec!["Trans".to_string(), "Offer".to_string()],
                    on_change: move |evt: String| { ctrl.update_artwork_field("medium".to_string(), evt.clone()) },
                }
                DropdownInput {
                    label: tr.theme,
                    placeholder: tr.theme,
                    value: ctrl.artwork_input_field().theme.clone(),
                    options: vec!["Trans".to_string(), "Offer".to_string()],
                    on_change: move |evt: String| { ctrl.update_artwork_field("theme".to_string(), evt.clone()) },
                }
                DropdownInput {
                    label: tr.art_style,
                    placeholder: tr.art_style,
                    value: ctrl.artwork_input_field().art_style.clone(),
                    options: vec!["Trans".to_string(), "Offer".to_string()],
                    on_change: move |evt: String| {
                        ctrl.update_artwork_field("art_style".to_string(), evt.clone())
                    },
                }
                Input2 {
                    label: tr.royalty,
                    placeholder: tr.royalty,
                    value: ctrl.artwork_input_field().royalty.clone(),
                    on_change: move |evt: String| { ctrl.update_artwork_field("royalty".to_string(), evt.clone()) },
                }
                Input2 {
                    label: tr.material,
                    placeholder: tr.material,
                    value: ctrl.artwork_input_field().material.clone(),
                    on_change: move |evt: String| {
                        ctrl.update_artwork_field("material".to_string(), evt.clone())
                    },
                }
                Input2 {
                    label: tr.color,
                    placeholder: tr.color,
                    value: ctrl.artwork_input_field().color.clone(),
                    on_change: move |evt: String| { ctrl.update_artwork_field("color".to_string(), evt.clone()) },
                }
                DropdownInput {
                    label: tr.size,
                    placeholder: tr.size,
                    value: ctrl.artwork_input_field().size.clone(),
                    options: vec!["inch".to_string(), "cm".to_string()],
                    on_change: move |evt: String| { ctrl.update_artwork_field("size".to_string(), evt.clone()) },
                }
                DropdownInput {
                    label: tr.weight,
                    placeholder: tr.weight,
                    value: ctrl.artwork_input_field().weight.clone(),
                    options: vec!["Gram".to_string(), "Kg".to_string()],
                    on_change: move |evt: String| { ctrl.update_artwork_field("weight".to_string(), evt.clone()) },
                }
                Input2 {
                    width_class: "w-64".to_string(),
                    label: tr.year,
                    placeholder: tr.year,
                    value: ctrl.artwork_input_field().lock_up_period.clone(),
                    on_change: move |evt: String| { ctrl.update_artwork_field("year".to_string(), evt.clone()) },
                }
            }
        }
        // Action buttons
        div { class: "flex justify-end space-x-4 mt-8" }
    }
}
