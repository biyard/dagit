use dioxus::prelude::*;
#[component]
pub fn SuccessModal(
    show: bool,
    on_confirm: EventHandler<()>
) -> Element {
    if !show {
        return rsx!(div {});
    }

    rsx! {
        // Modal backdrop with purple glow effect
        div { 
            class: "fixed inset-0 bg-black bg-opacity-50 backdrop-blur-sm z-50",
            style: "background: radial-gradient(circle at center, rgba(76, 29, 149, 0.15) 0%, rgba(0, 0, 0, 0.5) 100%)",
            onclick: move |_| on_confirm.call(()),
            
            // Modal content
            div { 
                class: "fixed inset-0 flex items-center justify-center p-4",
                onclick: move |e| e.stop_propagation(),
                
                div { 
                    class: "bg-[#000000] border border-[#333] rounded-lg shadow-2xl w-full max-w-md",
                    
                    // Modal header
                    div { class: "flex items-center justify-between px-6 pt-5 border-[#333]",
                        h2 { class: "text-xl font-semibold text-white",
                            "Collection Created"
                        }
                        button { 
                            class: "text-gray-400 hover:text-white",
                            onclick: move |_| on_confirm.call(()),
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
                    div { class: "px-6 py-2",
                        p { class: "text-white",
                            "The collection has been successfully created. Redirecting to the collection screen."
                        }
                    }
                    
                    // Modal footer
                    div { class: "flex  items-center p-6 border-[#333]",
                        button { 
                            class: " flex-1 px-4 py-2 text-sm bg-white text-black hover:bg-gray-200 min-w-[120px]",
                            onclick: move |_| on_confirm.call(()),
                            "Confirm"
                        }
                    }
                }
            }
        }
    }
}