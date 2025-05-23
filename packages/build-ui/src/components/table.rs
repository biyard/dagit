use bdk::prelude::*;
use by_components::icons::arrows::UpDown;

#[derive(Clone, PartialEq, Debug)]
pub enum SortDirection {
    Asc,
    Desc,
}

#[derive(Clone, PartialEq, Debug)]
pub struct SortConfig {
    pub key: String,
    pub direction: SortDirection,
}

pub trait TableRow: Clone + PartialEq + 'static {
    fn id(&self) -> String {
        String::new()
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct TableHeaderCellProps {
    pub label: String,
    #[props(default = String::default())]
    pub width: String,
    #[props(default = true)]
    pub sortable: bool,
    // #[props(optional)]
    // pub onclick: Option<EventHandler<String>>,
}

#[component]
fn DefaultTableHeaderCell(
    props: TableHeaderCellProps,
    onclick: Option<EventHandler<String>>,
) -> Element {
    let label = props.label.clone();

    rsx! {
        th {
            width: props.width,
            onclick: move |_| {
                onclick
                    .as_ref()
                    .map(|onclick| {
                        onclick.call(label.clone());
                    });
            },
            div { class: "w-full flex gap-1 py-4 text-white text-[15px]/[25px] cursor-pointer justify-center items-center",
                {props.label}
                if props.sortable {
                    UpDown { class: "[&>path]:stroke-white" }
                }
            }

        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct TableProps {
    pub columns: Vec<TableHeaderCellProps>,
    pub children: Element,
    pub on_filter_change: EventHandler<String>,
    pub render_row_header:
        Option<fn(TableHeaderCellProps, Option<EventHandler<String>>) -> Element>,
}

#[component]
pub fn Table(props: TableProps) -> Element {
    let render_row_header: fn(TableHeaderCellProps, Option<EventHandler<String>>) -> Element =
        props.render_row_header.unwrap_or(DefaultTableHeaderCell);

    let mut sort = use_signal(|| None::<String>);
    let handler = Callback::new(move |v: String| {
        sort.set(Some(v.clone()));
        props.on_filter_change.call(v);
    });

    rsx! {
        table { class: "w-full table-fixed border-collapse text-white",
            thead {
                tr { {props.columns.into_iter().map(|col| render_row_header(col, Some(handler)))} }
            }
            tbody { class: "[&>tr]:odd:bg-neutral-90", {props.children} }
        }
    }
}
