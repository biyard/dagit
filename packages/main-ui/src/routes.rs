use dioxus::prelude::*;
use dioxus_translate::Language;

#[allow(unused)]
use crate::layout::RootLayout;

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

    #[end_nest]
    
    #[redirect("/", || Route::MainPage { lang: Language::Ko })]
    #[route("/:..route")]
    NotFoundPage {
        route: Vec<String>,
    },
}
