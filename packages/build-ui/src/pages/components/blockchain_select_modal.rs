use bdk::prelude::*;

use crate::components::ChainIcon;
use crate::pages::components::ModalOption;
use crate::services::user_service::Chain;
#[component]
pub fn BlockchainSelectionModal(on_select: EventHandler<Chain>, lang: Language) -> Element {
    rsx! {
        div { class: "flex flex-col gap-2.5 mt-8",
            ModalOption {
                label: Chain::InternetComputer.translate(&lang),
                disabled: false,
                icon: rsx! {
                    ChainIcon { class: "w-10 h-10", chain: Chain::InternetComputer }
                },
                onclick: move |_| on_select(Chain::InternetComputer),
            }
            ModalOption {
                label: Chain::Bitcoin.translate(&lang),
                disabled: true,
                icon: rsx! {
                    ChainIcon { class: "w-10 h-10", chain: Chain::Bitcoin }
                },
                onclick: move |_| on_select(Chain::Bitcoin),
            }
            ModalOption {
                label: Chain::Ethereum.translate(&lang),
                disabled: true,
                icon: rsx! {
                    ChainIcon { class: "w-10 h-10", chain: Chain::Ethereum }
                },
                onclick: move |_| on_select(Chain::Ethereum),
            }
        }
    }
}
