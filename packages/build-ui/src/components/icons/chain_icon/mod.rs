mod bitcoin;
mod ethereum;
mod icp;

use bdk::prelude::*;

use crate::services::user_service::Chain;

#[component]
pub fn ChainIcon(chain: Chain, #[props(default = "".to_string())] class: String) -> Element {
    rsx! {
        div { class: "flex justify-center items-center {class}",
            match chain {
                Chain::Bitcoin => rsx! {
                    bitcoin::BitcoinIcon {}
                },
                Chain::Ethereum => rsx! {
                    ethereum::EthereumIcon {}
                },
                Chain::InternetComputer => rsx! {
                    icp::IcpIcon {}
                },
            }
        }
    }
}
