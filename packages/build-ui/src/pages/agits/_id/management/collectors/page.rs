#![allow(unused)]
use bdk::prelude::{
    by_components::icons::{arrows, edit, validations},
    svg_attributes::to,
    *,
};

use super::controller::Controller;
use super::i18n::CollectorsTranslate;
use crate::{
    pages::agits::_id::management::collectors::components::CollectorsTable, routes::Route,
};
#[component]
pub fn CollectorPage(lang: Language, agit_id: ReadOnlySignal<i64>) -> Element {
    let ctrl = Controller::new(lang, agit_id)?;
    let tr: CollectorsTranslate = translate(&lang);
    let mut search_query = use_signal(String::new);
    let collectors = ctrl.collector();

    rsx! {
        div { class: "w-full min-h-screen bg-background h-full flex text-white justify-center items-center",
            div { class: "flex flex-col w-full h-full",
                div { class: "flex flex-col mb-6",
                    h1 { class: "text-2xl font-bold", "Collectors" }
                    p { class: "text-sm text-gray-400", "1,201 Total Collectors" }
                }

                div { class: "mb-6",
                    div { class: "relative",
                        div { class: "absolute inset-y-0 left-0 flex items-center pl-3 pointer-events-none",
                            edit::Search { class: "[&>path]:stroke-white [&>circle]:stroke-white" }
                        }
                        input {
                            class: "bg-border-background border border-border-primary text-white text-sm rounded-none block w-full pl-10 p-2.5",
                            placeholder: tr.search_by_title,
                            r#type: "text",
                        }
                    }
                }

                div { class: "flex-1 overflow-x-auto",
                    CollectorsTable { lang: lang.clone(), agit_id: agit_id.clone() }
                }
            }
        }
    }
}
