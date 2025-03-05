use dioxus::prelude::*;
use dioxus_translate::Language;

use crate::pages::collection::{Artwork, Collection, FilterSidebar, NewCollectionModal, ServiceLogo, SidebarItem, SidebarSection};





#[allow(unused_variables)]
#[component]
pub fn CollectionsPage(lang: Language) -> Element {
    let mut show_filters = use_signal(|| false);
    let mut show_modal = use_signal(|| false);
    let collections = use_signal(|| {
        (1..15).map(|id| Collection {
            id,
            name: "(Collection Name)".to_string(),
            verified: true,
            floor_price_eth: 2.370,
            floor_price_usd: 8147.63,
            floor_change_eth: 2.370,
            floor_change_usd: 8147.63,
            volume_change_24h: 12.0,
            volume_change_7d: -8.0,
            volume_eth: 2.370,
            volume_usd: 8147.63,
            owners: "Num".to_string(),
            stock: "Num".to_string(),
            status: "Active".to_string(),
        }).collect::<Vec<_>>()
    });

    let artwork = use_signal(|| {
        (0..4).map(|id| Artwork {
            id,
            title: "(Art Title)".to_string(),
            artist_name: "Artist Name".to_string(),
            verified: true,
            collection: Some("Happy".to_string()),
            attributes: vec!["Paid".to_string(), "Verified".to_string()],
            ways_to_sell: "Bid".to_string(),
            volume_eth: 2.370,
            volume_usd: 8147.63,
            status: "Active".to_string(),
        }).collect::<Vec<_>>()
    });

    rsx! {
        div { class: "min-h-screen bg-[#171717] text-white flex",

            // Sidebar
            div { class: "w-64 bg-[#171717] border-r border-[#333] flex flex-col",
               ServiceLogo{}
                
                // Navigation
                nav { class: "flex-1",
                    SidebarItem { label: "Home", active: false }
                    
                    SidebarSection { label: "Orders" }
                    SidebarItem { label: "Sales Request", active: false, indent: true }
                    SidebarItem { label: "Shipping Label", active: false, indent: true }
                    
                    SidebarSection { label: "Management" }
                    SidebarItem { label: "Artworks", active: false, indent: true }
                    SidebarItem { label: "Collections", active: true, indent: true }
                    SidebarItem { label: "Artists", active: false, indent: true }
                    
                    SidebarSection { label: "Community" }
                    SidebarItem { label: "Collectors", active: false, indent: true }
                    SidebarItem { label: "DAO", active: false, indent: true }
                    SidebarItem { label: "Oracle", active: false, indent: true }
                    SidebarItem { label: "FAQ", active: false, indent: true }
                    
                    SidebarSection { label: "Customers" }
                    
                    SidebarSection { label: "Analytics" }
                    SidebarItem { label: "Report", active: false, indent: true }
                    SidebarItem { label: "Traffic", active: false, indent: true }
                    
                    SidebarSection { label: "Design" }
                    
                    SidebarSection { label: "Extension Tool" }
                }
                
                // User profile at bottom
                div { class: "p-4 border-t border-[#333] flex items-center",
                    div { class: "w-8 h-8 bg-[#333] mr-2" }
                    span { "(Agit Name)" }
                }
            }
            
            // Main content
            div { class: "flex-1 flex flex-col",
                // Header
                div { class: "p-6",
                    h1 { class: "text-2xl font-bold font-Pretendard", "Collections" }
                    p { class: "text-sm text-gray-400", "1,120 Total Collections" }
                }
                
                // Search and filters
                div { class: "p-4 flex items-center",
                    button { class: "p-2 border border-[#333] mr-2",
                    onclick: move |_| show_filters.toggle(),
                    
                        svg {
                            view_box: "0 0 24 24",
                            width: "24",
                            height: "24",
                            stroke: "currentColor",
                            stroke_width: "2",
                            fill: "none",
                            path {
                                d: "M4 6h16M4 12h16M4 18h16"
                            }
                        }
                    }
                    
                    div { class: "relative flex-1 mr-4",
                        div { class: "absolute inset-y-0 left-0 flex items-center pl-3 pointer-events-none",
                            svg {
                                view_box: "0 0 24 24",
                                width: "18",
                                height: "18",
                                stroke: "currentColor",
                                stroke_width: "2",
                                fill: "none",
                                path {
                                    d: "M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
                                }
                            }
                        }
                        input {
                            class: "bg-[#222] border border-[#333] text-white text-sm rounded-none block w-full pl-10 p-2.5",
                            placeholder: "Search by title",
                            r#type: "text"
                        }
                    }
                    
                    button { class: "bg-[#222] border border-[#333] text-white px-4 py-2 flex items-center",
                    onclick: move |_| show_modal.toggle(),
                        svg {
                            view_box: "0 0 24 24",
                            width: "18",
                            height: "18",
                            stroke: "currentColor",
                            stroke_width: "2",
                            fill: "none",
                            class: "mr-2",
                            path {
                                d: "M12 4v16m8-8H4"
                            }
                        }
                        "New Collection"
                    }
                }
                div { class: "flex flex-1",
                if *show_filters.read()
                
                 {
                   FilterSidebar {}
                }
                        
                        // Table body
                       // Corrected table structure to ensure header and body alignment
div { class: "flex-1 overflow-auto",
table { class: "w-full text-sm text-left border-collapse",
    // Table header
    thead { class: "text-xs uppercase bg-[#1a1a1a] border-b border-[#333]",
        tr {
            th { class: "px-4 py-3 whitespace-nowrap", 
                div { class: "flex items-center", 
                    span { "#" }
                    span { class: "ml-1 text-gray-500", "%" }
                }
            }
            th { class: "px-4 py-3 whitespace-nowrap", 
                div { class: "flex items-center", 
                    span { "Collection" }
                    span { class: "ml-1 text-gray-500", "%" }
                }
            }
            th { class: "px-4 py-3 whitespace-nowrap", 
                div { class: "flex items-center", 
                    span { "Floor Price" }
                    span { class: "ml-1 text-gray-500", "%" }
                }
            }
            th { class: "px-4 py-3 whitespace-nowrap", 
                div { class: "flex items-center", 
                    span { "Floor Change" }
                    span { class: "ml-1 text-gray-500", "%" }
                }
            }
            th { class: "px-4 py-3 whitespace-nowrap", 
                div { class: "flex items-center", 
                    span { "Volume Change" }
                    span { class: "ml-1 text-gray-500", "%" }
                }
            }
            th { class: "px-4 py-3 whitespace-nowrap", 
                div { class: "flex items-center", 
                    span { "Volume" }
                    span { class: "ml-1 text-gray-500", "%" }
                }
            }
            th { class: "px-4 py-3 whitespace-nowrap", 
                div { class: "flex items-center", 
                    span { "Owners" }
                    span { class: "ml-1 text-gray-500", "%" }
                }
            }
            th { class: "px-4 py-3 whitespace-nowrap", 
                div { class: "flex items-center", 
                    span { "Stock" }
                    span { class: "ml-1 text-gray-500", "%" }
                }
            }
            th { class: "px-4 py-3 whitespace-nowrap", 
                div { class: "flex items-center", 
                    span { "Status" }
                    span { class: "ml-1 text-gray-500", "%" }
                }
            }
            th { class: "px-4 py-3 whitespace-nowrap", "" } // For the actions column
        }
    }
    
    // Table body
    tbody {
        {collections.read().iter().enumerate().map(|(index, collection)| {
            rsx! {
                tr { key: "{index}", class: "border-b border-[#333]",
                    td { class: "px-4 py-3 whitespace-nowrap", "{collection.id}" }
                    td { class: "px-4 py-3 whitespace-nowrap",
                        div { class: "flex items-center", 
                            div { class: "w-8 h-8 bg-[#333] mr-2" }
                            span { "{collection.name}" }
                            // Verified icon
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
                    }
                    td { class: "px-4 py-3 whitespace-nowrap",
                        div { "{collection.floor_price_eth} ETH" }
                        div { class: "text-xs text-gray-400", "$ {collection.floor_price_usd}" }
                    }
                    td { class: "px-4 py-3 whitespace-nowrap ",
                        div { "{collection.floor_change_eth} ETH" }
                        div { class: "text-xs text-gray-400", "$ {collection.floor_change_usd}" }
                    }
                    td { class: "px-4 py-3 whitespace-nowrap",
                    div { class: "flex items-center space-x-4 p-l-4",
                        div { class: "flex flex-col",
                            div { class: "text-green-500", "+ {collection.volume_change_24h}%" }
                            div { class: "text-xs text-gray-400", "24h" }
                        }
                        div { class: "flex flex-col",
                            div { class: "text-red-500", "{collection.volume_change_7d}%" }
                            div { class: "text-xs text-gray-400", "7d" }
                        }
                    }
                }


                    td { class: "px-4 py-3 whitespace-nowrap",
                        div { "{collection.volume_eth} ETH" }
                        div { class: "text-xs text-gray-400", "$ {collection.volume_usd}" }
                    }
                    td { class: "px-4 py-3 whitespace-nowrap", "{collection.owners}" }
                    td { class: "px-4 py-3 whitespace-nowrap", "{collection.stock}" }
                    td { class: "px-4 py-3 whitespace-nowrap", "{collection.status}" }
                    td { class: "px-4 py-3 whitespace-nowrap",
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
                       
               
                }
                }
            }
            NewCollectionModal {
                show: *show_modal.read(),
                on_close: move |_| show_modal.set(false),
                artworks: artwork.clone(),
            }
        }

        }
   
   
// Add this new component
