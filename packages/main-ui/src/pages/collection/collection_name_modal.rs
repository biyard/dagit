use dioxus::prelude::*;

use crate::pages::collection::SuccessModal;
#[component]
pub fn CollectionNameModal(
    show: bool,
    on_back: EventHandler<()>,
    on_add: EventHandler<()>
) -> Element {
    let mut collection_name = use_signal(|| String::new());
    let mut show_success = use_signal(|| false);
    
    if !show {
        return rsx!(div {});
    }

    rsx! {
        // Modal backdrop with purple glow effect
        div { 
            class: "fixed inset-0 bg-black bg-opacity-50 backdrop-blur-sm z-50",
            style: "background: radial-gradient(circle at center, rgba(76, 29, 149, 0.15) 0%, rgba(0, 0, 0, 0.5) 100%)",
            onclick: move |_| on_back.call(()),
            
            // Modal content
            div { 
                class: "fixed inset-0 flex items-center justify-center p-4",
                onclick: move |e| e.stop_propagation(),
                
                div { 
                    class: "bg-[#000000] border border-[#333] rounded-lg shadow-2xl w-full max-w-md",
                    
                    // Modal header
                    div { class: "flex items-center justify-between px-6 pt-6 border-[#333]",
                        h2 { class: "text-xl font-semibold text-white",
                            "What is the name of collection?"
                        }
                        button { 
                            class: "text-gray-400 hover:text-white",
                            onclick: move |_| on_back.call(()),
                            svg {
                                class: "w-6 h-6",
                                view_box: "0 0 24 24",
                                stroke: "currentColor",
                                stroke_width: "2",
                                fill: "none",
                                path { d: "M6 18L18 6M6 6l12 12" }
                            }
                        }
                    }
                    
                    // Modal body
                    div { class: "px-6 pb-5",
                        p { class: "text-sm text-gray-400 mb-4",
                            "Choose a collection name."
                        }
                        
                        // Collection name input
                        div { class: "mb-4",
                            label { 
                                class: "block text-sm font-medium text-[#AEAAAB] mb-2",
                                "Collection Name"
                            }
                            input {
                                class: "w-full bg-transparent border border-[#6B6B6B] text-white text-sm rounded-none p-2",
                                placeholder: "Collection name",
                                value: "{collection_name}",
                                oninput: move |evt| collection_name.set(evt.value().clone())
                            }
                        }
                        
                        // Short URL input
                        div {
                            label { 
                                class: "block text-sm font-medium text-[#AEAAAB] mb-2",
                                "Short URL"
                            }
                            div { class: "flex",
                                span { 
                                    class: "inline-flex items-center px-3 text-sm text-gray-400 border-[#6B6B6B] border border-r-0",
                                    "dagit.com/"
                                }
                                input {
                                    class: "flex-1 bg-transparent border border-[#6B6B6B] text-white text-sm rounded-none p-2",
                                    placeholder: "(collection)",
                                    // Fix: Use proper string interpolation for Dioxus
                                    value: {
                                        let name = collection_name.read();
                                        name.to_lowercase().replace(" ", "-")
                                    }
                                }
                            }
                        }
                    }
                    
                    // Modal footer
                    div { class: "flex items-center justify-end gap-4 p-6 border-t border-[#333]",
                        button { 
                            class: "px-5 py-2 text-l text-gray-400 hover:text-white",
                            onclick: move |_| on_back.call(()),
                            "Back"
                        }
                        button { 
                            class: "px-4 py-2 text-l bg-white text-black hover:bg-gray-200",
                            onclick: move |_|{
                               
                                show_success.set(true);
                            },
                            "Add Collection"
                        }
                    }
                }
            }
        }

        // Add the SuccessModal
        SuccessModal {
            show: *show_success.read(),
            on_confirm: move |_| {
                show_success.set(false);
                on_add.call(());
            }
        }

    }
}