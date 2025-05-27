use bdk::prelude::{
    by_components::icons::{arrows, validations},
    *,
};

use crate::pages::agits::_id::management::collections::CollectionTranslate;
#[component]
pub fn TableHeader(lang: Language) -> Element {
    let tr: CollectionTranslate = translate(&lang);
    rsx! {
        thead {
            tr {

                th { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                    div { class: "flex items-center",
                        span { {tr.title} }
                        arrows::UpDown {
                            class: "[&>path]:stroke-white [&>circle]:stroke-white",
                            height: 18,
                            width: 18,
                        }
                    }
                }
                th { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                    div { class: "flex items-center",
                        span { {tr.attributes} }
                                        //   arrows::UpDown { class: "[&>path]:stroke-white [&>circle]:stroke-white", height:18, width:18 }
                    }
                }
                th { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                    div { class: "flex items-center",
                        span { {tr.ways_to_sell} }
                        arrows::UpDown {
                            class: "[&>path]:stroke-white [&>circle]:stroke-white",
                            height: 18,
                            width: 18,
                        }
                    }
                }
                th { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                    div { class: "flex items-center",
                        span { {tr.owner} }
                        arrows::UpDown {
                            class: "[&>path]:stroke-white [&>circle]:stroke-white",
                            height: 18,
                            width: 18,
                        }
                    }
                }
                th { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                    div { class: "flex items-center",
                        span { {tr.current_price} }
                        arrows::UpDown {
                            class: "[&>path]:stroke-white [&>circle]:stroke-white",
                            height: 18,
                            width: 18,
                        }
                    }
                }
                th { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                    div { class: "flex items-center",
                        span { {tr.average_price} }
                        arrows::UpDown {
                            class: "[&>path]:stroke-white [&>circle]:stroke-white",
                            height: 18,
                            width: 18,
                        }
                    }
                }
                th { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                    div { class: "flex items-center",
                        span { {tr.price_change} }
                        arrows::UpDown {
                            class: "[&>path]:stroke-white [&>circle]:stroke-white",
                            height: 18,
                            width: 18,
                        }
                    }
                }
                th { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                    div { class: "flex items-center",
                        span { {tr.volume} }
                        arrows::UpDown {
                            class: "[&>path]:stroke-white [&>circle]:stroke-white",
                            height: 18,
                            width: 18,
                        }
                    }
                }
                th { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                    div { class: "flex items-center",
                        span { {tr.royalty} }
                        arrows::UpDown {
                            class: "[&>path]:stroke-white [&>circle]:stroke-white",
                            height: 18,
                            width: 18,
                        }
                    }
                }
                th { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                    div { class: "flex items-center",
                        span { {tr.status} }
                        arrows::UpDown {
                            class: "[&>path]:stroke-white [&>circle]:stroke-white",
                            height: 18,
                            width: 18,
                        }
                    }
                }
                th { class: "px-2 py-2 sm:px-4 sm:py-3 whitespace-nowrap",
                    div { class: "flex items-center",
                        span { "" }
                        validations::Extra { class: "[&>circle]:stroke-white", height: 18 }
                    }
                }
            }
        }
    }
}
