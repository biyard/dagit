use bdk::prelude::*;
#[component]
pub fn TabButton(
    label: String,
    is_active: bool,
    #[props(default = None)] badge_text: Option<String>,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        button {
            class: format!(
                "pb-2 px-5 flex items-center space-x-1 {}",
                if is_active {
                    "border-b border-white font-semibold text-white"
                } else {
                    "text-gray-400 hover:text-white"
                },
            ),
            onclick: move |evt| onclick(evt),
            span { "{label}" }
            if let Some(badge) = badge_text {
                span { class: "ml-1 px-1.5 py-0.5 rounded text-xs bg-white text-black font-semibold",
                    "{badge}"
                }
            }
        }
    }
}
