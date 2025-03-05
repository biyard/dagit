use dioxus::prelude::*;
use crate::pages::collection::TransferConfirmationModal;
#[warn(unused_imports)]
use crate::pages::collection::Artwork;
#[component]

#[allow(unused_variables)]
pub fn NewCollectionModal(show: bool, on_close: EventHandler<()>, artworks: Signal<Vec<Artwork>>) -> Element {
    // Use `use_signal` for a Vec<usize> to store selected artwork IDs
    let mut selected_artworks = use_signal(|| Vec::<usize>::new());
    let mut show_transfer_modal = use_signal(|| false);
    // let mut show_new_collection_modal = use_signal(|| false);

    if !show {
        return rsx!(div {});
    }
    rsx! {
        // Modal backdrop with purple glow effect
        div {
            class: "fixed inset-0  bg-opacity-0 backdrop-blur-sm z-50 
            bg-[radial-gradient(circle,rgba(255,41,144,0.5)_20%,rgba(0,0,0,0)_70%)]",
            onclick: move |_| on_close.call(()),

            // Modal content
            div {
                class: "fixed inset-0  flex items-center justify-center p-4 shadow-[0_0_40px_10px_rgba(255,41,144,0.5)]",
                onclick: move |e| e.stop_propagation(),

                div {
                    class: "bg-[#000000] border border-[#333] rounded-lg shadow-2xl w-full max-w-6xl max-h-[90vh] flex flex-col",

                    // Modal header
                    div { class: "flex items-center justify-between p-6 border-b border-[#333]",
                        div { class: "flex flex-col",
                            h2 { class: "text-xl font-semibold text-white",
                                "Please select the artwork to include in the collection"
                            }
                            p { class: "text-sm text-gray-400 mt-1",
                                "Add artwork to the collection. You can also move artwork from another collection."
                            }
                        }
                        button {
                            class: "text-gray-400 hover:text-white",
                            onclick: move |_| on_close.call(()),
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

                    // Search and filters
                    div { class: "p-4 flex items-center gap-4 border-b border-[#333]",
                        button { class: "p-2 border border-[#333] bg-[#222]",
                            svg {
                                view_box: "0 0 24 24",
                                width: "24",
                                height: "24",
                                stroke: "currentColor",
                                stroke_width: "2",
                                fill: "none",
                                path { d: "M4 6h16M4 12h16M4 18h16" }
                            }
                        }
                        select { class: "bg-[#222] border border-[#333] text-white text-sm p-2 w-40",
                            option { "All" }
                        }
                        div { class: "flex-1 relative",
                            input {
                                class: "w-full bg-[#222] border border-[#333] text-white text-sm rounded-none p-2 pl-10",
                                placeholder: "Search by title",
                                r#type: "text"
                            }
                            div { class: "absolute inset-y-0 left-3 flex items-center text-[#ffffff]",
                                svg {
                                    view_box: "0 0 24 24",
                                    width: "16",
                                    height: "16",
                                    stroke: "currentColor",
                                    stroke_width: "2",
                                    fill: "none",
                                    path { d: "M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" }
                                }
                            }
                        }
                    }

                    // Table
                    div { class: "flex-1 overflow-auto",
                        table { class: "w-full text-sm text-left",
                            thead { class: "text-xs uppercase border-b border-[#333]",
                                tr {
                                    th { class: "px-4 py-3 w-8",
                                        input {
                                            r#type: "checkbox",
                                            class: "form-checkbox bg-transparent border-[#333] text-primary rounded-none accent-[#333]"
                                        }
                                    }
                                    th { class: "px-4 py-3",
                                        div { class: "flex items-center ",
                                            span {class:"text-white" , "Title", }
                                            span { class: "ml-1 text-gray-500", "%" }
                                        }
                                    }
                                    th { class: "px-4 py-3 text-white", "Collection" }
                                    th { class: "px-4 py-3 text-white", "Attributes" }
                                    th { class: "px-4 py-3",
                                        div { class: "flex items-center",
                                            span { class:"text-white","Ways to Sell",  }
                                            span { class: "ml-1 text-gray-500", "%" }
                                        }
                                    }
                                    th { class: "px-4 py-3",
                                        div { class: "flex items-center",
                                            span {  class:"text-white","Volume" }
                                            span { class: "ml-1 text-gray-500", "%" }
                                        }
                                    }
                                    th { class: "px-4 py-3",
                                        div { class: "flex items-center",
                                            span { class:"text-white", "Status", }
                                            span { class: "ml-1 text-gray-500", "%" }
                                        }
                                    }
                                    th { class: "px-4 py-3", "" }
                                }
                            }
                            tbody {
                                // Iterate over artworks using .read()
                                {artworks.read().clone().into_iter().enumerate().map(move |(index, artwork)|{
                                    rsx! {
                                        tr { class: "border-b border-[#333]", key:"{index}",
                                            td { class: "px-4 py-3",
                                                input {
                                                    r#type: "checkbox",
                                                    class: "form-checkbox bg-transparent border-[#333] text-primary rounded-none accent-[#333]",

                                                    checked: selected_artworks.read().contains(&artwork.id),
                                                    onclick: move |_| {
                                                        // Use .with_mut to modify the signal's Vec
                                                        selected_artworks.with_mut(|vec| {
                                                            if vec.contains(&artwork.id) {
                                                                vec.retain(|&x| x != artwork.id);
                                                            } else {
                                                                vec.push(artwork.id);
                                                            }
                                                        });
                                                    }
                                                }
                                            }
                                            td { class: "px-4 py-3",
                                                div { class: "flex items-center",
                                                    div { class: "w-8 h-8 bg-[#333] mr-2" }
                                                    div { class: "flex flex-col",
                                                        div { class: "flex items-center text-[#ffffff]",
                                                            span { "{artwork.title}" }
                                                            svg {
                                                                view_box: "0 0 24 24",
                                                                width: "16",
                                                                height: "16",
                                                                fill: "#10b981",
                                                                class: "ml-1",
                                                                path {
                                                                    d: "M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
                                                                }
                                                            }
                                                        }
                                                        div { class: "text-sm text-gray-400",
                                                            "{artwork.artist_name}"
                                                        }
                                                    }
                                                }
                                            }
                                            td { class: "px-4 py-3 text-[#ffffff]", "{artwork.collection.as_ref().unwrap_or(&String::new())}" }
                                            td { class: "px-4 py-3",
                                                div { class: "flex gap-2",
                                                    { artwork.attributes.iter().map(|attr| {
                                                        rsx! {
                                                            span {
                                                                class: "px-2 py-1 bg-transparent border border-[#333] text-xs rounded text-[#ffffff]",
                                                                "{attr}"
                                                            }
                                                        }
                                                    })}
                                                }
                                            }
                                            td { class: "px-4 py-3 text-[#ffffff]", "{artwork.ways_to_sell}" }
                                            td { class: "px-4 py-3",
                                                div { class: "text-[#ffffff]","{artwork.volume_eth} ETH" }
                                                div { class: "text-xs text-gray-400", "$ {artwork.volume_usd}" }
                                            }
                                            td { class: "px-4 py-3 text-[#ffffff] ", "{artwork.status}" }
                                            td { class: "px-4 py-3",
                                                button { class: "text-gray-400 hover:text-white",
                                                    svg {
                                                        view_box: "0 0 24 24",
                                                        width: "18",
                                                        height: "18",
                                                        stroke: "currentColor",
                                                        stroke_width: "2",
                                                        fill: "none",
                                                        path {
                                                            d: "M12 5v.01M12 12v.01M12 19v.01M12 6a1 1 0 110-2 1 1 0 010 2zm0 7a1 1 0 110-2 1 1 0 010 2zm0 7a1 1 0 110-2 1 1 0 010 2z"
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                })}
                            }
                        }
                    }

                    // Footer
                    div { class: "p-4 border-t border-[#333] flex flex-col justify-end self-end",
                    div { class: "text-sm text-gray-400 mb-5",
                    "{selected_artworks.read().len()} artworks have been selected."
                }
                        div { class: "flex gap-4",
                            button {
                                class: "px-4 py-2 text-sm text-gray-400 hover:text-white",
                                onclick: move |_| on_close.call(()),
                                "Cancel"
                            }
                            button {
                                class: "px-4 py-2 text-sm bg-white text-black hover:bg-gray-200",
                                onclick: move |_| {
                                    // on_close.call(());
                                   if !selected_artworks.read().is_empty() {
                                       show_transfer_modal.set(true);
                                    }
                                    
                                },
                                "Confirm"
                            }

                        
                        }
                    }
                }
            }
        }
        TransferConfirmationModal {
            show: *show_transfer_modal.read(),
            selected_count: selected_artworks.read().len(),
            on_back: move |_| show_transfer_modal.set(false),
            on_continue: move |_| {
                on_close.call(());
                // Continue with the transfer
                show_transfer_modal.set(false);
            }
        }
    }
}