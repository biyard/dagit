use by_components::icons::validations::Clear;
use dioxus::prelude::*;
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
            onclick: move |_| {
                popup.close();
            },
            if popup.is_opened() {
                div {
                    class: "bg-black border-neutral-800 relative border-[1px] px-[30px] py-[25px] shadow-[0_0_100px_0px_rgba(255,41,144,0.50)]",
                    onclick: move |e| {
                        e.stop_propagation();
                    },
                    if (popup.close)() {
                        button {
                            class: "absolute top-[25px] right-[25px] cursor-pointer bg-transparent group",
                            onclick: move |_| {
                                popup.close();
                            },
                            Clear { class: "[&>path]:stroke-white group-hover:bg-gray-100" }
                        }
                    }
                    div {
                        id: popup.get_id(),
                        class: "flex flex-col items-center justify-center gap-[25px] text-white",
                        match popup.get_title() {
                            Some(title) => {
                                rsx! {
                                    div { class: "text-[20px] font-bold", "{title}" }
                                }
                            }
                            None => rsx! {},
                        }
                        {popup.render()}
                    }
                }
            }
        }
    }
}
