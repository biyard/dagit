// use bdk::prelude::by_components::icons::{arrows, other_devices};
use bdk::prelude::*;

// use crate::components::button::{ButtonWithIcon, IconButton};
// use crate::components::image_upload::FileUpload;
// use crate::components::input::{Input2, TextArea};
// use crate::components::search_filter_bar::SearchFilterBar;
// use crate::pages::agits::_id::management::artists::_id::i18n::EditArtistPageTranslate;
// use crate::pages::agits::_id::management::artists::components::SingleArtistTable;
// use crate::pages::agits::_id::management::artists::controller::Controller;
// use crate::pages::agits::_id::management::artists::i18n::ArtistTranslate;
// use crate::pages::agits::_id::management::components::NftTable;
// use crate::routes::Route;
#[component]
#[allow(unused_variables)]
pub fn ArtistDetailPage(lang: Language, agit_id: ReadOnlySignal<i64>, artist_id: i64) -> Element {
    // let tr: ArtistTranslate = translate(&lang);
    // let mut view_mode = use_signal(|| "table");
    // let ctrl = Controller::new(lang, agit_id)?;
    // let artist_assets = ctrl.artworks();
    rsx! {}
    //     rsx! {
    //         div { class: "w-full min-h-screen bg-background h-full flex text-white justify-center items-center",
    //             div { class: "flex flex-col w-full h-full text-white",

    //                 // Header with back button and collector info
    //                 div { class: "flex flex-col mb-6",
    //                     div { class: "flex items-center",
    //                         div { onclick: move |_| ctrl.go_back(),
    //                             svg {
    //                                 xmlns: "http://www.w3.org/2000/svg",
    //                                 class: "h-6 w-6",
    //                                 fill: "none",
    //                                 view_box: "0 0 24 24",
    //                                 stroke: "currentColor",
    //                                 path {
    //                                     stroke_linecap: "round",
    //                                     stroke_linejoin: "round",
    //                                     stroke_width: "2",
    //                                     d: "M15 19l-7-7 7-7",
    //                                 }
    //                             }
    //                         }

    //                         div { class: "flex items-center",

    //                             h1 { class: "text-xl font-bold", {format!("{{Artist {}}}", artist_id)} }
    //                         }
    //                     }
    //                     div { class: "text-sm text-gray-400 m-2", "52 Total Artworks" }
    //                 }

    //                 // Search and view controls

    //                 SearchFilterBar {
    //                     placeholder: tr.search_by_title,
    //                     show_filter_btn: true,
    //                     on_filter_click: move |_| {},
    //                     on_search_change: move |search_text| {},
    //                     on_search: move |search_text| {},
    //                     show_all_filter_field: true,
    //                     show_art_btn: true,
    //                     on_view_mode_click: move |_| {
    //                         view_mode.set(if *view_mode.read() == "table" { "nftImages" } else { "table" });
    //                     },
    //                     show_add_btn: true,
    //                     add_btn_text: tr.new_artist,
    //                     on_add_click: move |_| ctrl.open_new_artist_form(),
    //                     remove_btn_text: tr.remove_artist,
    //                 }

    //                 // Assets table
    //                 div { class: "overflow-x-auto flex-1",
    //                     {
    //                         if *view_mode.read() == "nftImages" {
    //                             rsx! {
    //                                 NftTable { artworks: artist_assets.clone(), lang }
    //                             }
    //                         } else {
    //                             rsx! {
    //                                 SingleArtistTable {
    //                                     assets: artist_assets.clone(),
    //                                     lang,
    //                                     agit_id: agit_id(),
    //                                     artist_id,
    //                                 }
    //                             }
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }

    // #[component]
    // pub fn EditArtistPage(
    //     lang: Language,
    //     agit_id: ReadOnlySignal<i64>,
    //     artist_id: ReadOnlySignal<i64>,
    // ) -> Element {
    //     let tr: EditArtistPageTranslate = translate(&lang);
    //     let mut ctrl = Controller::new(lang, agit_id)?;
    //     let _profile_picture = use_signal(|| None::<String>);
    //     let mut is_dropdown_open = use_signal(|| false);

    //     // Handle form submission
    //     let handle_save = move |_| {
    //         // todo: similate api this
    //     };

    //     rsx! {
    //         div { class: "w-full min-h-screen bg-background h-full flex text-white items-center",
    //             div { class: "flex flex-col w-full h-full p-6",
    //                 // Header with title and back button
    //                 div { class: "flex items-center mb-6",
    //                     div { onclick: move |_| ctrl.go_back(),
    //                         svg {
    //                             xmlns: "http://www.w3.org/2000/svg",
    //                             class: "h-6 w-6",
    //                             fill: "none",
    //                             view_box: "0 0 24 24",
    //                             stroke: "currentColor",
    //                             path {
    //                                 stroke_linecap: "round",
    //                                 stroke_linejoin: "round",
    //                                 stroke_width: "2",
    //                                 d: "M15 19l-7-7 7-7",
    //                             }
    //                         }
    //                     }
    //                     div { class: "flex justify-between w-full",
    //                         h1 { class: "text-2xl font-bold", "{tr.edit} {artist_id}'s {tr.info}" }

    //                         div { class: "relative",
    //                             IconButton { onclick: move |_| is_dropdown_open.toggle() }
    //                             div {
    //                                 class: "absolute right-0 mt-2 w-48 bg-background border border-border-primary rounded-md shadow-lg z-1 hidden aria-dropdown-open:block",
    //                                 "aria-dropdown-open": is_dropdown_open,
    //                                 div { class: "py-1",
    //                                     a {
    //                                         onclick: move |_| {
    //                                             ctrl.confirm_removal_modal();
    //                                         },
    //                                         class: "block px-4 py-2 text-sm text-white hover:bg-gray-700 hover:text-white",
    //                                         {tr.remove_artist}
    //                                     }
    //                                     Link {
    //                                         to: Route::ArtistPage {
    //                                             lang,
    //                                             agit_id: agit_id(),
    //                                         },
    //                                         class: "block px-4 py-2 text-sm text-white hover:bg-gray-700 hover:text-white",
    //                                         {tr.edit_artist_info}
    //                                     }
    //                                     Link {
    //                                         to: Route::ArtistPage {
    //                                             lang,
    //                                             agit_id: agit_id(),
    //                                         },
    //                                         class: "block px-4 py-2 text-sm text-white hover:bg-gray-700 hover:text-white",
    //                                         {tr.delete_artist}
    //                                     }
    //                                 }
    //                             }
    //                         }
    //                     }
    //                 }

    //                 p { class: "text-gray-400 mb-8", "Joined Feb 20, 2025" }

    //                 // Artist Info Section
    //                 div { class: "mb-8",
    //                     div { class: "flex items-center mb-4",
    //                         h2 { class: "text-xl font-semibold", {tr.artist_info} }
    //                         arrows::ChevronDown {
    //                             class: "ml-2 [&>path]:stroke-white",
    //                             height: 20,
    //                             width: 20,
    //                         }
    //                     }

    //                     // Form fields
    //                     div { class: "space-y-4 max-w-4xl",
    //                         Input2 {
    //                             label: tr.input_artist_name_label,
    //                             placeholder: tr.input_artist_name_placeholder,
    //                             value: ctrl.artist_input_field().display_name.clone(),
    //                             on_change: move |evt: String| {
    //                                 ctrl.update_artist_field("display_name".to_string(), evt.clone())
    //                             },
    //                         }

    //                         Input2 {
    //                             label: tr.social_media_label,
    //                             placeholder: tr.social_media_label,
    //                             value: ctrl.artist_input_field().social_media.clone(),
    //                             on_change: move |evt: String| {
    //                                 ctrl.update_artist_field("social_media".to_string(), evt.clone())
    //                             },
    //                         }

    //                         Input2 {
    //                             label: tr.medium,
    //                             placeholder: tr.medium,
    //                             value: ctrl.artist_input_field().medium.clone(),
    //                             on_change: move |evt: String| { ctrl.update_artist_field("medium".to_string(), evt.clone()) },
    //                         }

    //                         Input2 {
    //                             label: tr.theme,
    //                             placeholder: tr.theme,
    //                             value: ctrl.artist_input_field().theme.clone(),
    //                             on_change: move |evt: String| { ctrl.update_artist_field("theme".to_string(), evt.clone()) },
    //                         }

    //                         Input2 {
    //                             label: tr.art_style,
    //                             placeholder: tr.art_style,
    //                             value: ctrl.artist_input_field().art_style.clone(),
    //                             on_change: move |evt: String| {
    //                                 ctrl.update_artist_field("art_style".to_string(), evt.clone())
    //                             },
    //                         }
    //                     }
    //                     // Profile Picture Upload
    //                     div { class: "flex flex-col mt-8",
    //                         label { class: "mb-1 text-sm", "Profile Picture" }
    //                         FileUpload { onclick: move |_| {}, label: tr.upload_img_label }
    //                         p { class: "text-xs text-gray-500 mt-1", {tr.max_file_size} }
    //                         p { class: "text-xs text-gray-500", {tr.max_file_size_2} }
    //                     }
    //                 }

    //                 // Introduction Section
    //                 div { class: "mb-8 mt-8 max-w-4xl",
    //                     TextArea {
    //                         label: tr.introduction_label,
    //                         placeholder: tr.introduction_placeholder,
    //                         value: ctrl.artist_input_field().introduction.clone(),
    //                         on_change: move |evt: String| {
    //                             ctrl.update_artist_field("introduction".to_string(), evt.clone())
    //                         },
    //                     }
    //                 }
    //                 // Biography Section
    //                 div { class: "mt-8 max-w-4xl",
    //                     TextArea {
    //                         label: tr.biography_label,
    //                         placeholder: tr.biography_placeholder,
    //                         value: ctrl.artist_input_field().biography.clone(),
    //                         on_change: move |evt: String| {
    //                             ctrl.update_artist_field("biography".to_string(), evt.clone())
    //                         },
    //                     }
    //                 }

    //                 // Action buttons
    //                 div { class: "flex justify-end space-x-4 mt-8",
    //                     ButtonWithIcon {
    //                         onclick: handle_save,
    //                         icon: rsx! {
    //                             other_devices::Save { class: "mr-2 [&>path]:stroke-white", height: 20, width: 20 }
    //                         },
    //                         label: tr.save_btn_txt,
    //                         disabled: ctrl.artist_input_field().display_name.is_empty(),
    //                     }
    //                 }
    //             }
    //         }
    //     }
}
