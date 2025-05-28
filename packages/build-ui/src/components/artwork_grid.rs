use bdk::prelude::*;

#[component]
pub fn ArtworkGrid(
    onclick: EventHandler<i64>,
    items: Vec<GridItemProps>,
    renderer: Option<fn(GridItemProps, EventHandler<i64>) -> Element>,
) -> Element {
    let onclick = Callback::new(move |v: i64| onclick.call(v));

    let renderer: fn(GridItemProps, EventHandler<i64>) -> Element = renderer.unwrap_or(GridItem);

    rsx! {
        div { class: "grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4" }

        {
            items
                .into_iter()
                .map(|item| {
                    let id = item.id;
                    renderer(
                        item,
                        Callback::new(move |_| {
                            onclick.call(id);
                        }),
                    )
                })
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct GridItemProps {
    pub id: i64,
    pub title: String,
    pub artist_name: String,
    pub image_url: String,
}

#[component]
pub fn GridItem(props: GridItemProps, on_click: EventHandler<i64>) -> Element {
    rsx! {
        div {
            class: "bg-gray-800 rounded-lg p-4 cursor-pointer",
            onclick: move |_| {
                on_click.call(props.id);
            },
            img {
                src: props.image_url,
                class: "w-full h-48 object-cover rounded-t-lg",
            }
            h3 { class: "text-lg font-semibold text-white mt-2", {props.title} }
            p { class: "text-sm text-gray-400 mt-1", "by {props.artist_name}" }
        }
    }
}
