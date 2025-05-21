use bdk::prelude::{
    by_components::icons::{arrows, other_devices, settings, validations},
    *,
};

use crate::{
    components::{
        button::{ButtonWithIcon, IconButton},
        image_upload::FileUpload,
        input::{Input2, TextArea},
    },
    pages::agits::_id::management::artists::{
        controller::Controller, create::i18n::CreateArtworkPageTranslate,
    },
    routes::Route,
};
#[component]
pub fn CreateArtistPage(lang: Language, agit_id: ReadOnlySignal<i64>) -> Element {
    let tr: CreateArtworkPageTranslate = translate(&lang);
    let mut ctrl = Controller::new(lang, agit_id)?;
    let _profile_picture = use_signal(|| None::<String>);
    let mut is_dropdown_open = use_signal(|| false);
    // Handle form submission
    let handle_save = move |_| {
        // todo: similate api this
    };

    rsx! {
        div { class: "w-full min-h-screen bg-background h-full flex text-white items-center",
            div { class: "flex flex-col w-full h-full  p-6",
                // Header with title and back button
                div { class: "flex items-center mb-6",
                    div { onclick: move |_| ctrl.go_back(),
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            class: "h-6 w-6",
                            fill: "none",
                            view_box: "0 0 24 24",
                            stroke: "currentColor",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                d: "M15 19l-7-7 7-7",
                            }
                        }
                    }

                    div { class: "flex justify-between w-full",
                        h1 { class: "text-2xl font-bold", {tr.title} }

                        div { class: "relative",
                            IconButton {
                                onclick: move |_| is_dropdown_open.toggle(),
                                icon: rsx! {
                                    settings::Settings2 { class: "ml-2 [&>path]:stroke-white", height: 20, width: 20 }
                                },
                            }
                            div {
                                class: "absolute right-0 mt-2 w-48 bg-background border border-border-primary rounded-md shadow-lg z-1 hidden aria-dropdown-open:block",
                                "aria-dropdown-open": is_dropdown_open,
                                div { class: "py-1",
                                    Link {
                                        to: Route::ArtistPage {
                                            lang,
                                            agit_id: agit_id(),
                                        },
                                        class: "block px-4 py-2 text-sm text-white hover:bg-gray-700 hover:text-white",
                                        "Artist Detail"
                                    }
                                    Link {
                                        to: Route::ArtistPage {
                                            lang,
                                            agit_id: agit_id(),
                                        },
                                        class: "block px-4 py-2 text-sm text-white hover:bg-gray-700 hover:text-white",
                                        "Edit Artist Info"
                                    }
                                    Link {
                                        to: Route::ArtistPage {
                                            lang,
                                            agit_id: agit_id(),
                                        },
                                        class: "block px-4 py-2 text-sm text-white hover:bg-gray-700 hover:text-white",
                                        "Delete Artist"
                                    }
                                }
                            }
                        }
                    }
                }

                p { class: "text-gray-400 mb-8", {tr.new_artist_description} }

                // Artist Info Section
                div { class: "mb-8",
                    div { class: "flex items-center mb-4",
                        h2 { class: "text-xl font-semibold", {tr.artist_info} }
                        arrows::ChevronDown {
                            class: "ml-2 [&>path]:stroke-white",
                            height: 20,
                            width: 20,
                        }
                    }

                    // Form fields
                    div { class: "space-y-4 max-w-4xl",
                        Input2 {
                            label: tr.input_artist_name_label,
                            placeholder: tr.input_artist_name_placeholder,
                            value: ctrl.artist_input_field().display_name.clone(),
                            on_change: move |evt: String| {
                                ctrl.update_artist_field("display_name".to_string(), evt.clone())
                            },
                        }

                        Input2 {
                            label: tr.social_media_label,
                            placeholder: tr.social_media_label,
                            value: ctrl.artist_input_field().social_media.clone(),
                            on_change: move |evt: String| {
                                ctrl.update_artist_field("social_media".to_string(), evt.clone())
                            },
                        }

                        Input2 {
                            label: tr.medium,
                            placeholder: tr.medium,
                            value: ctrl.artist_input_field().medium.clone(),
                            on_change: move |evt: String| { ctrl.update_artist_field("medium".to_string(), evt.clone()) },
                        }

                        Input2 {
                            label: tr.theme,
                            placeholder: tr.theme,
                            value: ctrl.artist_input_field().theme.clone(),
                            on_change: move |evt: String| { ctrl.update_artist_field("theme".to_string(), evt.clone()) },
                        }

                        Input2 {
                            label: tr.art_style,
                            placeholder: tr.art_style,
                            value: ctrl.artist_input_field().art_style.clone(),
                            on_change: move |evt: String| {
                                ctrl.update_artist_field("art_style".to_string(), evt.clone())
                            },
                        }
                    }

                    // Profile Picture Upload
                    div { class: "flex flex-col mt-8",
                        label { class: "mb-1 text-sm", "Profile Picture" }
                        FileUpload { onclick: move |_| {}, label: tr.upload_img_label }
                        p { class: "text-xs text-gray-500 mt-1", {tr.max_file_size} }
                        p { class: "text-xs text-gray-500", {tr.max_file_size_2} }
                    }
                }

                // Introduction Section
                div { class: "mb-8 mt-8 max-w-4xl",
                    TextArea {
                        label: tr.introduction_label,
                        placeholder: tr.introduction_placeholder,
                        value: ctrl.artist_input_field().introduction.clone(),
                        on_change: move |evt: String| {
                            ctrl.update_artist_field("introduction".to_string(), evt.clone())
                        },
                    }
                }

                // Biography Section
                div { class: "mt-8 max-w-4xl",
                    TextArea {
                        label: tr.biography_label,
                        placeholder: tr.biography_placeholder,
                        value: ctrl.artist_input_field().biography.clone(),
                        on_change: move |evt: String| {
                            ctrl.update_artist_field("biography".to_string(), evt.clone())
                        },
                    }
                }

                // Action buttons
                div { class: "flex justify-end space-x-4 mt-8",
                    ButtonWithIcon {
                        onclick: handle_save,
                        icon: rsx! {
                            other_devices::Save { class: "mr-2 [&>path]:stroke-white", height: 20, width: 20 }
                        },
                        label: tr.save_btn_txt,
                        disabled: ctrl.artist_input_field().display_name.is_empty(),
                    }
                    ButtonWithIcon {
                        onclick: move |_| ctrl.create_artist(),
                        icon: rsx! {
                            validations::Add { class: "mr-2 [&>path]:stroke-white [&>circle]:stroke-white" }
                        },
                        label: tr.add_btn_txt,
                        disabled: ctrl.artist_input_field().display_name.is_empty(),
                    }
                }
            }
        }
    }
}
