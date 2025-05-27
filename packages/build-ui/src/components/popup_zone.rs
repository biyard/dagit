use bdk::prelude::*;
use by_components::icons::validations::Clear;
use dioxus_popup::PopupService;

#[component]
pub fn PopupZone() -> Element {
    let mut popup: PopupService = use_context();

    rsx! {
        div {
            class: format!(
                "{}",
                match popup.is_opened() {
                    true => {
                        "fixed top-0 left-0 w-screen h-screen bg-black bg-opacity-50 flex justify-center items-center backdrop-blur-[4px] bg-black/25 z-[101]"
                    }
                    false => "hidden",
                },
            ),
            if popup.is_opened() {
                div { class: "text-white bg-black border-neutral-800 relative border-[1px] px-[30px] py-[25px] shadow-[0_0_100px_0px_rgba(255,41,144,0.50)]",
                    div { id: popup.get_id(), class: "flex flex-col",
                        div { class: "w-full h-full flex flex-row items-center justify-between gap-[25px]",
                            div { class: "text-[20px] font-bold",
                                match popup.get_title() {
                                    Some(title) => {
                                        rsx! {
                                            span { {title} }
                                        }
                                    }
                                    None => rsx! {},
                                }
                            }
                            if (popup.close)() {
                                button {
                                    class: "cursor-pointer bg-transparent group",
                                    onclick: move |_| {
                                        popup.close();
                                    },
                                    Clear { class: "[&>path]:stroke-white group-hover:bg-gray-100" }
                                }
                            }
                        }
                        {popup.render()}
                    }
                }
            }
        }
    }
}
