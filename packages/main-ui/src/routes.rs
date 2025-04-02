use dioxus::prelude::*;
use dioxus_translate::Language;
// use crate::global_attributes::lang;

#[allow(unused)]
use crate::layout::{RootLayout, SidebarLayout};

use crate::pages::prelude::*;
// use crate::pages::NotFoundPage;

#[derive(Clone, Routable)]
#[rustfmt::skip]
pub enum Route {
    #[nest("/:lang")]
        #[route("/platform")]
        PlatformPage { lang : Language},
        #[layout(RootLayout)]
            #[route("/")]
            MainPage { lang: Language }, 
            
        #[end_layout]
            //  This is for all the route using thesame sidebar
            #[layout(SidebarLayout)]
               #[route("/collections")]
               CollectionsPage { lang: Language },
            #[end_layout]


    #[end_nest]
    
    #[redirect("/", || Route::MainPage { lang: Language::Ko })]
    #[route("/:..route")]
    NotFoundPage {
        route: Vec<String>,
    },
}
