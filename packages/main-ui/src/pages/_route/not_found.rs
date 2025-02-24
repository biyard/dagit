use dioxus::prelude::*;

#[component]
pub fn NotFoundPage(route: Vec<String>) -> Element {
    rsx! {
        div {
            "404 Not Found"
            div {
                "Route: "
                {route.join("/")}
            }
        }
    }
}
