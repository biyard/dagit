use bdk::prelude::*;

use crate::components::WalletIcon;
use crate::pages::components::ModalOption;
use crate::services::user_service::Wallet;
#[component]
pub fn ConnectWalletModal(on_select: EventHandler<Wallet>, lang: Language) -> Element {
    rsx! {
        div { class: "flex flex-col gap-2.5 mt-8",
            ModalOption {
                label: Wallet::Google.translate(&lang),
                icon: rsx! {
                    WalletIcon { class: "w-10 h-10", wallet: Wallet::Google }
                },
                onclick: move |_| on_select(Wallet::Google),
            }
            ModalOption {
                label: Wallet::MetaMask.translate(&lang),
                disabled: true,
                icon: rsx! {
                    WalletIcon { class: "w-10 h-10", wallet: Wallet::MetaMask }
                },
                onclick: move |_| on_select(Wallet::MetaMask),
            }
        }
    }
}
