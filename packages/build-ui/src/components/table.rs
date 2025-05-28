use bdk::prelude::*;
use by_components::icons::{arrows::UpDown, validations::Extra};

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
    pub min_width: String,
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
            min_width: props.min_width,
            onclick: move |_| {
                onclick
                    .as_ref()
                    .map(|onclick| {
                        onclick.call(label.clone());
                    });
            },
            div { class: "w-full flex gap-1 py-4 text-white whitespace-nowrap text-[15px]/[25px] cursor-pointer justify-center items-center",
                {props.label}
                if props.sortable {
                    UpDown { class: "[&>path]:stroke-white" }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct DropDownOptionProps {
    pub label: String,
    pub onclick: EventHandler<String>,
}

#[component]
fn DropDownOption(props: DropDownOptionProps) -> Element {
    let label = props.label.clone();
    rsx! {
        a {
            href: "#",
            class: "block px-5 py-2.5 text-sm text-white hover:text-primary hover:bg-bg-hover whitespace-nowrap",
            role: "menuitem",
            tabindex: "-1",
            onclick: move |_| {
                props.onclick.call(label.clone());
            },
            {props.label}
        }
    }
}
#[derive(Props, Clone, PartialEq)]
pub struct TableProps {
    pub children: Element,
    pub on_filter_change: EventHandler<String>,

    pub columns: Vec<TableHeaderCellProps>,
    pub render_row_header:
        Option<fn(TableHeaderCellProps, Option<EventHandler<String>>) -> Element>,

    pub dropdown_options: Option<Vec<DropDownOptionProps>>,
    pub render_dropdown_option: Option<fn(DropDownOptionProps) -> Element>,
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

    let mut show_dropdown = use_signal(|| false);
    let render_dropdown_option: fn(DropDownOptionProps) -> Element =
        props.render_dropdown_option.unwrap_or(DropDownOption);
    rsx! {
        table { class: "min-w-full table-fixed border-collapse text-white",
            thead {
                tr {
                    {props.columns.into_iter().map(|col| render_row_header(col, Some(handler)))}

                    if let Some(dropdown_options) = &props.dropdown_options {
                        TableRowOptionCell {
                            class: "[&>div>svg]:!rotate-0",
                            show: show_dropdown(),
                            toggle: move |v| {
                                show_dropdown.set(v);
                            },
                            dropdown_options: dropdown_options.clone(),
                            render_dropdown_option,
                        }
                    }
                }
            }
            tbody { class: "[&>tr]:odd:bg-neutral-90", {props.children} }
        }
    }
}

#[component]
pub fn TableRowOptionCell(
    #[props(default = "".to_string())] class: String,
    show: bool,
    toggle: EventHandler<bool>,
    dropdown_options: Vec<DropDownOptionProps>,
    render_dropdown_option: fn(DropDownOptionProps) -> Element,
) -> Element {
    rsx! {
        th {
            class: "min-w-20",
            onclick: move |_| {
                toggle.call(!show);
            },
            div { class: "w-full flex cursor-pointer justify-center items-center {class}",
                div { class: "group relative", aria_hidden: !show,
                    Extra { class: "rotate-90 [&>circle]:fill-white size-8 border border-primary group-aria-hidden:border-transparent  bg-black group-aria-hidden:bg-transparent" }
                    div {
                        id: "menu-dropdown",
                        class: "absolute right-0 top-0 origin-top-right group-aria-hidden:hidden border border-primary bg-black translate-y-[30px]",
                        role: "menu",
                        tabindex: "-1",
                        {dropdown_options.iter().map(|option| { render_dropdown_option(option.clone()) })}
                    }
                }
            }
        }
    }
}
