use dioxus::prelude::*;
use dioxus_translate::Language;

#[allow(unused)]
use crate::pages::*;

#[derive(Clone, Routable)]
#[rustfmt::skip]
pub enum Route {
    #[nest("/:lang")]
        #[layout(RootLayout)]
            #[route("/")]
            RootPage { lang: Language },
        #[end_layout]
    #[end_nest]
    
    #[redirect("/", || Route::RootPage { lang: Language::Ko })]
    #[route("/:..route")]
    NotFoundPage {
        route: Vec<String>,
    },
}
