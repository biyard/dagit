use bdk::prelude::*;

#[component]
pub fn SecondaryButton(
    #[props(default = "".to_string())] class: String,
    children: Element,
    #[props(default = false)] disabled: bool,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        button {
            class: format!(
                "px-4.5 py-3 border border-white bg-black active:bg-white active:text-black hover:bg-white hover:text-black disabled:border-neutral-80 disabled:text-neutral-80 disabled:bg-btn-disabled {}",
                class,
            ),
            onclick: move |e| {
                if !disabled {
                    onclick.call(e);
                }
            },
            disabled,
            {children}
        }
    }
}

#[component]
pub fn PrimaryButton(
    #[props(default = "".to_string())] class: String,
    children: Element,
    #[props(default = true)] disabled: bool,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        button {
            class: format!(
                "px-4.5 py-3 border border-black text-white bg-black active:bg-primary/25 active:border-primary hover:bg-primary/25 hover:border-primary disabled:border-neutral-80 disabled:text-neutral-80 disabled:bg-btn-disable {}",
                class,
            ),
            onclick: move |e| {
                if !disabled {
                    onclick.call(e);
                }
            },
            disabled,
            {children}
        }
    }
}

#[component]
pub fn IconButton(
    children: Element,
    onclick: EventHandler<MouseEvent>,
    #[props(default = false)] disabled: bool,
    // #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        button {
            class: "group border border-neutral-80 hover:border-black hover:bg-white p-2.5 ",
            onclick: move |e| {
                onclick.call(e);
            },
            div { class: "size-6 [&>svg>path]:stroke-white group-hover:[&>svg>path]:stroke-black",
                {children}
            }
        }
    }
}
