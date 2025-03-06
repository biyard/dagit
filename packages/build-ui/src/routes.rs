use dioxus::prelude::*;
use dioxus_translate::Language;

#[allow(unused)]
use crate::pages::*;

#[derive(Clone, Routable)]
#[rustfmt::skip]
pub enum Route {
    #[nest("/:lang")]
        #[layout(HeaderLayout)]
            #[route("/")]
            RootPage { lang: Language},
        #[end_layout]
        #[nest("/agits/:agit_id")]
            #[layout(NavigationLayout)]
                #[route("/")]
                    HomePage { lang: Language, agit_id: i64 },
            #[end_layout]
        #[end_nest]
    #[end_nest]
    #[redirect("/", || Route::RootPage { lang: Language::Ko })]
    #[route("/:..route")]
    NotFoundPage {
        route: Vec<String>,
    },
}
