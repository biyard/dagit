use bdk::prelude::{dioxus::web::WebEventExt, *};
use by_components::icons::arrows::ChevronUp;

#[cfg(feature = "web")]
use eventhook::use_outside_click;
// use std::cell::RefCell;
// use std::rc::Rc;
// use wasm_bindgen::{JsCast, prelude::Closure};
// use web_sys::window;

pub mod eventhook {
    use bdk::prelude::{tracing, use_hook_with_cleanup};
    use std::{cell::RefCell, ops::Deref, rc::Rc};
    use wasm_bindgen::{
        JsCast,
        closure::Closure,
        convert::{FromWasmAbi, RefFromWasmAbi},
    };
    use web_sys::{Element, EventTarget, HtmlElement, MouseEvent, window};

    #[derive(Clone)]
    pub struct EventListenerHandle {
        cleanup: Rc<RefCell<Option<Box<dyn FnOnce()>>>>,
    }

    impl EventListenerHandle {
        pub fn new<EventKind, T>(
            target_element: T,
            event_name: &'static str,
            mut callback: impl FnMut(EventKind) + 'static,
        ) -> Self
        where
            EventKind: Sized + RefFromWasmAbi + FromWasmAbi + Clone + 'static,
            T: Clone + Deref<Target = EventTarget> + std::fmt::Debug + 'static,
        {
            let closure = Closure::wrap(Box::new(move |event: EventKind| {
                callback(event);
            }) as Box<dyn FnMut(_)>);
            if let Err(e) = target_element
                .add_event_listener_with_callback(&event_name, closure.as_ref().unchecked_ref())
            {
                tracing::error!("failed to add event listener: {e:?}");
            }

            let cleanup = Rc::new(RefCell::new(Some(Box::new(move || {
                if let Err(e) = target_element.remove_event_listener_with_callback(
                    &event_name,
                    closure.as_ref().unchecked_ref(),
                ) {
                    tracing::error!("failed to remove event listener: {e:?}");
                }
            }) as Box<dyn FnOnce()>)));
            Self { cleanup }
        }

        pub fn cleanup(&self) {
            let cleanup = self.cleanup.borrow_mut().take();
            if let Some(cleanup) = cleanup {
                cleanup();
            }
        }
    }

    impl Drop for EventListenerHandle {
        fn drop(&mut self) {
            // Only cleanup if this is the last reference.
            if Rc::strong_count(&self.cleanup) == 1 {
                self.cleanup();
            }
        }
    }

    pub fn use_on_event<EventKind, T>(
        target_element: &T,
        event_name: &'static str,
        mut callback: impl FnMut(EventKind) + 'static,
    ) where
        EventKind: Sized + RefFromWasmAbi + FromWasmAbi + Clone + 'static,
        T: Clone + Deref<Target = EventTarget> + std::fmt::Debug + 'static,
    {
        let hook = || {
            EventListenerHandle::new(target_element.clone(), event_name, move |kind| {
                callback(kind)
            })
        };

        let cleanup = |f: EventListenerHandle| {
            f.cleanup();
        };

        use_hook_with_cleanup(hook, cleanup);
    }

    // This is modified from another user on this discord.
    pub fn use_outside_click<S: ToString>(id: S, mut callback: impl FnMut(Element) + 'static) {
        let window = window().expect("");
        let document = window.document().expect("");

        let id = id.to_string();
        use_on_event(&window, "mousedown", move |ev: MouseEvent| {
            if let Some(target) = ev.target() {
                if let Some(dropdown) = document.get_element_by_id(&id) {
                    let target_element: &HtmlElement = target.unchecked_ref();
                    let target_node: &web_sys::Node = target_element.as_ref();
                    if !dropdown.contains(Some(target_node)) {
                        callback(dropdown);
                    }
                }
            }
        })
    }
}
#[component]
pub fn DropDown(
    id: String,
    #[props(default= String::default())] class: String,
    options: Vec<String>,
    placeholder: String,
    #[props(default = false)] disabled: bool,
    onchange: EventHandler<String>,
) -> Element {
    let mut is_focused = use_signal(|| false);
    let mut selected_option = use_signal(|| None::<String>);
    #[cfg(feature = "web")]
    use_outside_click(&id, move |_| is_focused.set(false));
    rsx! {
        div {
            id,
            aria_disabled: disabled,
            "aria-focused": is_focused(),
            aria_selected: selected_option().is_some(),
            class: "relative w-full group text-white aria-disabled:text-neutral-80 bg-transparent aria-focused:bg-black {class}",
            div {
                class: "flex items-center justify-between w-full border cursor-pointer border-neutral-80 group-aria-focused:border-primary px-4 py-3",
                onmounted: move |e| {
                    let el = e.as_web_event();
                    let boxed_el = Box::new(el);
                    Box::leak(boxed_el);
                },
                onclick: move |_| {
                    if !disabled {
                        let prev = is_focused();
                        is_focused.set(!prev);
                    }
                },
                span { class: "text-neutral-80 group-aria-selected:text-white",
                    if let Some(option) = selected_option() {
                        {option}
                    } else {
                        {placeholder}
                    }
                }
                div { class: "group-aria-focused:rotate-180 transition-transform duration-300 ease-in-out",
                    ChevronUp {
                        width: "18px",
                        height: "18px",
                        class: "[&>path]:stroke-white",
                    }
                }
            
            }
            if is_focused() {
                div { class: "flex flex-col absolute z-10 w-full mt-1 shadow-lg max-h-60 overflow-auto bg-black border border-primary gap-3 p-3",
                    for option in options {
                        div {
                            class: "cursor-pointer transition-colors hover:text-primary hover:bg-primary/50",
                            onclick: move |_| {
                                selected_option.set(Some(option.clone()));
                                is_focused.set(false);
                                onchange.call(option.clone());
                            },
                            "{option}"
                        }
                    }
                }
            }
        }
    }
}
