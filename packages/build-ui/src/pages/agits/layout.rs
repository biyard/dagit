#![allow(non_snake_case)]
use crate::components::popup_zone::PopupZone;
use crate::{routes::Route, services::user_service::UserService};
use bdk::prelude::{by_components::loaders::cube_loader::CubeLoader, *};

use super::Navigation;

#[component]
pub fn NavigationLayout(lang: Language, agit_id: i64) -> Element {
    let nav = use_navigator();
    let user: UserService = use_context();
    use_effect(move || {
        if user.user_info().is_none() {
            nav.push(Route::RootPage { lang });
        }
    });

    rsx! {
        div { class: "flex flex-row bg-background min-h-svh max-w-screen overflow-x-hidden",
            PopupZone {}
            Navigation { lang, agit_id }

            SuspenseBoundary {
                fallback: |_| {
                    rsx! {
                        div { class: "w-full h-screen flex justify-center items-center", CubeLoader {} }
                    }
                },
                div { class: "py-10 px-20 w-full overflow-x-scroll flex flex-col", Outlet::<Route> {} }
            }
        }
    }
}
