#![allow(non_snake_case)]
use dioxus::prelude::*;



#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Controller {
    email: Signal<String>,
    password: Signal<String>,
    not_matched_error: Signal<bool>,
    not_exists_error: Signal<bool>,
    login_failed_error: Signal<bool>,
}

impl Controller {
    pub fn init() -> Self {
        let ctrl = Self {
            email: use_signal(|| "".to_string()),
            password: use_signal(|| "".to_string()),
            not_matched_error: use_signal(|| false),
            not_exists_error: use_signal(|| false),
            login_failed_error: use_signal(|| false),
        };

        use_context_provider(|| ctrl);

        ctrl
    }

    pub fn set_email(&mut self, email: String) {
        self.email.set(email);
    }

    pub fn set_password(&mut self, password: String) {
        self.password.set(password);
    }

}