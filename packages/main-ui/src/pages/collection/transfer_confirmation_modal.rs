use dioxus::prelude::*;

use crate::pages::collection::CollectionNameModal;

#[allow(unused_variables)]
#[component]
pub fn TransferConfirmationModal(
    show: bool,
    selected_count: usize,
    on_back: EventHandler<()>,
    on_continue: EventHandler<()>
) -> Element {
    let mut show_name_modal = use_signal(|| false);



    if !show {
        return rsx! (div{});
    }

    rsx! {
        // Modal backdrop with purple glow effect
        div { 
            class: "fixed inset-0 bg-[#000000] bg-opacity-50 backdrop-blur-sm z-50",
            style: "background: radial-gradient(circle at center, rgba(76, 29, 149, 0.15) 0%, rgba(0, 0, 0, 0.5) 100%)",
            onclick: move |_| on_back.call(()),
            
            // Modal content
            div { 
                class: "fixed inset-0 flex items-center justify-center p-4",
                onclick: move |e| e.stop_propagation(),
                
                div { 
                    class: "bg-[#000000] border border-[#333] rounded-lg shadow-2xl w-full max-w-md",
                    
                    // Modal header
                    div { class: "flex items-center justify-between px-6 pt-6  pb-2 border-[#333]",
                        h2 { class: "text-xl font-semibold text-white",
                            "Transfer Artwork"
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
                    
                    div { class: "px-6 pr-10",
                        p { class: "text-white",
                            "The {selected_count} selected artworks are already included in another collection. Would you like to transfer the artworks to the designated collection?"
                        }
                    }
                    
                    // Modal footer
                    div { class: "flex items-center justify-between gap-4 p-6 border-[#333]",
                        button { 
                            class: "px-4 py-2 text-l text-[#ffffff] hover:text-white",
                            onclick: move |_| on_back.call(()),
                            "Back"
                        }
                        button { 
                            class: "px-10 py-3 text-l bg-white  text-black hover:bg-gray-200",
                            onclick: move |_| {
                                show_name_modal.set(true);
                                // on_continue.call(());
                            },
                            "Continue"
                        }
                    }
                }
            }
        }
        CollectionNameModal{
            show: *show_name_modal.read(),
            on_back: move |_| show_name_modal.set(false),
            on_add: move |_| {
                show_name_modal.set(false);
                on_continue.call(());
            }
        }
    }
}