use crate::components::ServiceLogo;
use bdk::prelude::*;

#[component]
pub fn ComingSoon() -> Element {
    rsx! {
        div { class: "w-full h-full flex flex-col text-white justify-center items-center gap-10",
            ServiceLogo { width: "500px" }
            h1 { class: "text-white text-3xl", "Coming Soon" }
        
        }
    }
}
