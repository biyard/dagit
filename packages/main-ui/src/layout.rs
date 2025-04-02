#![allow(non_snake_case)]
use dioxus::prelude::*;

use dioxus_translate::Language;

use crate::{components::prelude::{Footer, Header}, pages::collection::ServiceLogo, routes::Route};
use crate::pages::collection::{SidebarItem, SidebarSection};


#[component]
pub fn RootLayout(lang: Language) -> Element {
    rsx! {
        div { class: "w-full flex justify-center bg-[#F3F3F3]",
            Header { lang: lang.clone() }
            div { class: "w-full flex flex-col min-h-lvh mt-23",
                Outlet::<Route> {}
                Footer { lang: lang.clone() }
            }
        }
    }
}


#[component]
pub fn SidebarLayout(lang: Language) -> Element {
    let mut is_sidebar_open = use_signal(|| false); // Toggle state for sidebar

    rsx! {
        div { class: "w-full min-h-lvh bg-[#F3F3F3] flex flex-col md:flex-row", // Stack on small, row on medium+
            // Hamburger menu button for small screens
            button { 
                class: "md:hidden p-4 text-white bg-[#171717] flex items-center justify-between", // Visible only on small screens
                onclick: move |_| is_sidebar_open.toggle(),
                span { "Menu" }
                svg { // Hamburger icon
                    class: "w-6 h-6",
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    path { d: "M4 6h16M4 12h16M4 18h16" }
                }
            }
            // Sidebar (hidden on small screens unless toggled)
            div { 
                class: format!(
                    "w-64 bg-[#171717] border-r border-[#333] flex flex-col fixed inset-y-0 left-0 transform {} md:static md:translate-x-0 transition-transform duration-300",
                    if *is_sidebar_open.read() { "translate-x-0" } else { "-translate-x-full" }
                ),
                SideBar {}
            }
            // Overlay for mobile when sidebar is open
            if *is_sidebar_open.read() {
                div { 
                    class: "fixed inset-0 bg-black bg-opacity-50 md:hidden",
                    onclick: move |_| is_sidebar_open.set(false) // Close sidebar when clicking overlay
                }
            }
            // Main content
            div { class: "flex-1 flex flex-col",
                Outlet::<Route> {}
            }
        }
    }
}
#[component]
pub fn SideBar()->Element{
rsx!{
                    // Navigation
         div { class: "w-64 h-full flex flex-col bg-[#171717] border-r border-[#333] text-white",
                    ServiceLogo{}
                     
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

 div { class: "p-4 border-t border-[#333] flex items-center",
    div { class: "w-8 h-8 bg-[#333] mr-2" }
    span { "(Agit Name)" }
    }
}

}
}