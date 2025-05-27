use crate::services::user_service::Wallet;
use bdk::prelude::*;

use by_components::icons::logo::Google;
mod metamask;

#[component]
pub fn WalletIcon(wallet: Wallet, #[props(default = "".to_string())] class: String) -> Element {
    rsx! {
        div { class: "flex justify-center items-center {class}",
            match wallet {
                Wallet::Google => rsx! {
                    Google {}
                },
                Wallet::MetaMask => rsx! {
                    metamask::MetaMaskIcon {}
                },
            }
        }
    }
}
