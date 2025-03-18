#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_translate::Language;
use dioxus_translate::translate;

mod i18n;

use i18n::LoginTranslate;

use crate::pages::controller;

#[derive(PartialEq, Props, Clone)]
pub struct LoginPageProps {
    pub lang: Language,
}

#[derive(Props, Clone, PartialEq)]
pub struct InputEmailProps {
    ctrl: controller::Controller,
    email_message: String,
}

#[derive(Props, Clone, PartialEq)]
pub struct InputPasswordProps {
    ctrl: controller::Controller,
    password_message: String,
}

#[derive(Props, Clone, PartialEq)]
pub struct LoginProps {
    ctrl: controller::Controller,
    lang: Language,
    login_message: String,
    email_message: String,
    password_message: String,

    not_matched_error: String,
    not_exists_user_error: String,
    login_failed_error: String,
}

#[derive(Props, Clone, PartialEq)]
pub struct LoginButtonProps {
    login_message: String,
}

#[derive(Props, Clone, PartialEq)]
pub struct MemberInfoProps {
    lang: Language,
    find_email_message: String,
    reset_pw_message: String,
    create_account_message: String,
    check_title_message: String,
    check_description_1_message: String,
    check_description_2_message: String,
}

#[component]
pub fn LoginPage(props: LoginPageProps) -> Element {
    let ctrl = controller::Controller::init();
    let translates: LoginTranslate = translate(&props.lang.clone());

    let login_message = translates.login;
    let email_message = translates.email;
    let password_message = translates.password;

    rsx! {
        div {
            class: "bg-white dark:bg-black w-screen min-h-screen flex flex-col",

            div { class: "flex flex-row w-full justify-start items-center px-[30px] py-[3px]",
                div { class: "text-[24px] font-bold text-[#2168C3]", "You are almost there!" }
            }
            div {
                class: "flex flex-col h-full w-full justify-start items-center",
                style: "height: calc(100vh - 48px)",
                div { class: "flex flex-col w-full h-full justify-center items-center",
                    LoginComponent {
                        ctrl,
                        lang: props.lang,
                        email_message,
                        password_message,
                        login_message,
                        not_matched_error: translates.not_matched_error,
                        not_exists_user_error: translates.not_exists_user_error,
                        login_failed_error: translates.login_failed_error,
                    }
                }
            }
        }
    }
}

#[component]
pub fn LoginComponent(props: LoginProps) -> Element {
    rsx! {
        div {
            class: "fixed inset-0 flex items-center justify-center bg-black bg-opacity-50 backdrop-blur-md z-50",
            div {
                class: "bg-black border border-neutral-800 px-10 py-8 shadow-xl rounded-md w-[400px]",

                // Close button
                button {
                    class: "absolute top-4 right-4 text-white text-xl cursor-pointer",
                    onclick: move |_| { },
                    "✕"
                }

                // Title
                div { class: "text-[22px] font-bold text-white mb-4 text-center",
                    "{props.login_message}"
                }

                // Input fields
                div { class: "w-full flex flex-col gap-4",
                InputEmailComponent {
                    ctrl: props.ctrl,
                    email_message: props.email_message,
                }
                InputPasswordComponent {
                    ctrl: props.ctrl,
                    password_message: props.password_message,
                }
                }

                // Checkboxes
                div { class: "flex flex-col gap-2 mt-4 text-[14px] text-white",
                    label { class: "flex items-center gap-2",
                        input { r#type: "checkbox", class: "form-checkbox text-blue-500" }
                        "I have read and accept the "
                        a { href: "#", class: "font-bold text-blue-400 hover:underline", "Terms of Service" }
                    }
                    label { class: "flex items-center gap-2",
                        input { r#type: "checkbox", class: "form-checkbox text-blue-500" }
                        "I want to receive announcements and news."
                    }
                }

                // Login button
                button {
                    class: "w-full mt-6 py-3 rounded bg-gray-600 text-white font-bold cursor-not-allowed opacity-50",
                    disabled: "true",
                    "Finished Sign-up"
                }
            }
        }
    }
}

#[component]
pub fn LoginButton(props: LoginButtonProps) -> Element {
    rsx! {
        div {
            class: "flex flex-row w-[300px] h-[100px] bg-[#2168c3]",
            style: "width: 300px",
            div { class: "flex flex-row w-full h-full justify-center items-center text-[24px] font-bold text-white",
                "{props.login_message}"
            }
        }
    }
}

#[component]
pub fn InputPasswordComponent(props: InputPasswordProps) -> Element {
    let mut ctrl = props.ctrl;
    rsx! {
        div { class: "flex flex-col w-[300px] h-[42px] justify-start items-start",
            input {
                class: "flex flex-row px-[10px] py-[10px] w-full h-full",
                r#type: "password",
                style: "border: 1px solid #e0e0e0; color: #8e929b;",
                placeholder: "{props.password_message}",
                oninput: move |event| ctrl.set_password(event.value()),
            }
        }
    }
}

#[component]
pub fn InputEmailComponent(props: InputEmailProps) -> Element {
    let mut ctrl = props.ctrl;
    rsx! {
        div { class: "flex flex-row w-[300px] h-[42px] justify-start items-start",
            input {
                class: "flex flex-row px-[10px] py-[10px] w-full h-full",
                r#type: "text",
                style: "border: 1px solid #e0e0e0; color: #8e929b;",
                placeholder: "{props.email_message}",
                oninput: move |event| ctrl.set_email(event.value()),
            }
        }
    }
}
