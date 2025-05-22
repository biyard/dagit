use bdk::prelude::{by_components::icons::arrows::UpDown, *};

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
    pub id: String,
    pub label: String,
    #[props(default = String::default())]
    pub width: String,
    pub sortable: bool,
    // #[props(optional)]
    // pub onclick: Option<EventHandler<String>>,
}

#[component]
fn DefaultTableHeaderCell(
    props: TableHeaderCellProps,
    onclick: Option<EventHandler<String>>,
) -> Element {
    rsx! {
        th {
            class: "px-4 py-2 text-left text-sm font-medium text-gray-500",
            width: props.width,
            onclick: move |_| {
                onclick
                    .as_ref()
                    .map(|onclick| {
                        onclick.call(props.id.clone());
                    });
            },
            {props.label}
            if props.sortable {
                UpDown {}
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct TableProps {
    pub columns: Vec<TableHeaderCellProps>,
    pub children: Element,
    pub on_filter_change: EventHandler<SortConfig>,
    pub render_row_header:
        Option<fn(TableHeaderCellProps, Option<EventHandler<String>>) -> Element>,
}

#[component]
pub fn Table(props: TableProps) -> Element {
    let render_row_header: fn(TableHeaderCellProps, Option<EventHandler<String>>) -> Element =
        props.render_row_header.unwrap_or(DefaultTableHeaderCell);

    let mut sort = use_signal(|| None::<String>);
    let handler = Callback::new(move |v: String| sort.set(Some(v.clone())));

    rsx! {
        table {
            thead {
                tr { {props.columns.into_iter().map(|col| render_row_header(col, Some(handler)))} }
            }
        }
    }
}
