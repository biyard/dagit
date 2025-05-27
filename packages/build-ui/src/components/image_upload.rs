use bdk::prelude::*;

#[component]
pub fn FileUpload(
    #[props(default = "".to_string())] class: String,
    label: String,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        div {
            class: format!(
                "border border-dashed border-green-500 flex flex-col items-center justify-center text-center w-[340px] h-[340px] {}",
                class,
            ),
            onclick: move |e| {
                onclick.call(e);
            },
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                class: "h-12 w-12 text-gray-400 mb-2",
                fill: "none",
                view_box: "0 0 24 24",
                stroke: "currentColor",
                path {
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    stroke_width: "1",
                    d: "M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z",
                }
            }
            p { class: "text-l text-white px-5", "{label}" }
        }
    }
}
