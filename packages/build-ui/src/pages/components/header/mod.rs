use crate::{components::ServiceLogo, routes::Route};
use dioxus::prelude::*;
use dioxus_popup::PopupService;
use dioxus_translate::{Language, translate};
use i18n::HeaderTranslate;
mod i18n;

#[derive(Clone, PartialEq)]
enum Blockchain {
    Ethereum,
    Solana,
}

#[derive(Clone, PartialEq)]
struct Wallet {
    name: &'static str,
    image_url: &'static str,
}

fn get_wallets(blockchain: Option<&Blockchain>) -> Vec<Wallet> {
    match blockchain {
        Some(Blockchain::Ethereum) => vec![
            Wallet {
                name: "MetaMask",
                image_url: "https://cryptologos.cc/logos/metamask-icon.svg",
            },
            Wallet {
                name: "Trust Wallet",
                image_url: "https://cryptologos.cc/logos/trust-wallet-icon.svg",
            },
        ],
        Some(Blockchain::Solana) => vec![
            Wallet {
                name: "Phantom",
                image_url: "https://cryptologos.cc/logos/phantom-icon.svg",
            },
            Wallet {
                name: "Solflare",
                image_url: "https://cryptologos.cc/logos/solflare-icon.svg",
            },
        ],
        None => Vec::new(),
    }
}

#[component]
fn BlockchainPopup() -> Element {
    let mut popup: PopupService = use_context();
    let mut selected_blockchain = use_signal(|| None);
    
    rsx! {
        div {
            class: if popup.is_opened() { "fixed inset-0 bg-black bg-opacity-50 flex justify-center items-center backdrop-blur-md z-[101]" } else { "hidden" },
            onclick: move |_| popup.close(),

            if popup.is_opened() {
                div {
                    class: "bg-black border-neutral-800 relative border px-8 py-6 shadow-lg",
                    onclick: move |e| e.stop_propagation(),
                    button {
                        class: "absolute top-4 right-4 cursor-pointer text-white",
                        onclick: move |_| popup.close(),
                        "✕"
                    }
                    h2 { class: "text-white text-lg font-bold", "Choose Blockchain" }
                    div {
                        class: "flex flex-col gap-4 mt-4",
                        button {
                            class: "flex items-center gap-4 px-4 py-3 border rounded-md text-white bg-gray-800 hover:bg-gray-700",
                            onclick: move |_| {
                                selected_blockchain.set(Some(Blockchain::Ethereum));
                                popup.with_title("Choose Wallet").open(WalletPopup(WalletPopupProps { selected_blockchain: selected_blockchain}));
                            },
                            img { class: "w-8 h-8", src: "https://cryptologos.cc/logos/ethereum-eth-logo.svg" }
                            span { "Ethereum" }
                        }
                        button {
                            class: "flex items-center gap-4 px-4 py-3 border rounded-md text-white bg-gray-800 hover:bg-gray-700",
                            onclick: move |_| {
                                selected_blockchain.set(Some(Blockchain::Solana));
                                popup.with_title("Choose Wallet").open(WalletPopup(WalletPopupProps { selected_blockchain: selected_blockchain}));
                            },
                            img { class: "w-8 h-8", src: "https://cryptologos.cc/logos/solana-sol-logo.svg" }
                            span { "Solana" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn WalletPopup(selected_blockchain: Signal<Option<Blockchain>>) -> Element {
    let mut popup: PopupService = use_context();
    // let wallets = selected_blockchain.as_ref().map(get_wallets).unwrap_or_default();
    // let wallets = use_signal(|| Vec::new()); // Initialize as an empty vector

    // if let Some(blockchain) = selected_blockchain {
    //     wallets.set(get_wallets(&blockchain)); // Update it when blockchain is selected
    // }

    let wallets = get_wallets(selected_blockchain.read().as_ref());

    rsx! {
    div {
        class: if popup.is_opened() { "fixed inset-0 bg-black bg-opacity-50 flex justify-center items-center backdrop-blur-md z-[101]" } else { "hidden" },
        onclick: move |_| popup.close(),

        if popup.is_opened() {
            div {
                class: "bg-black border-neutral-800 relative border px-8 py-6 shadow-lg",
                onclick: move |e| e.stop_propagation(),
                button {
                    class: "absolute top-4 right-4 cursor-pointer text-white",
                    onclick: move |_| popup.close(),
                    "✕"
                }
                h2 { class: "text-white text-lg font-bold", "Choose Wallet" }
                div {
                    class: "flex flex-col gap-4 mt-4",
                    // for wallet in wallets.iter() {
                    //     button {
                    //         class: "flex items-center gap-4 px-4 py-3 border rounded-md text-white bg-gray-800 hover:bg-gray-700",
                    //         onclick: move |_| {
                    //             println!("Connecting to {}...", wallet.name);
                    //         },
                    //         img { class: "w-8 h-8", src: "{wallet.image_url}" }
                    //         span { "{wallet.name}" }
                    //     }
                    // }
                    for wallet in wallets.clone().into_iter() {
                        button {
                            class: "flex items-center gap-4 px-4 py-3 border rounded-md text-white bg-gray-800 hover:bg-gray-700",
                            onclick: {
                                move |_| {
                                    println!("Connecting to {}...", wallet.name.to_string());
                                }},
                                img { class: "w-8 h-8", src: "{wallet.image_url}" }
                                span { "{wallet.name}" }
                            },
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn Header(lang: Language) -> Element {
    let tr: HeaderTranslate = translate(&lang);
    let mut popup: PopupService = use_context();
    let is_logged_in = use_signal(|| false);

    rsx! {
        nav { class: "w-full flex justify-between items-center bg-transparent py-4 text-white px-4 z-100",
            ServiceLogo { width: "110", height: "24", class: "fill-white" }
            div { class: "flex gap-x-10",
                Link { to: Route::RootPage { lang }, "{tr.solution}" }
                Link { to: Route::RootPage { lang }, "{tr.pricing}" }
                Link { to: Route::RootPage { lang }, "{tr.faq}" }
            }
            if *is_logged_in.read() {
                div { class: "flex flex-row items-center gap-4",
                    div { class: "px-4 py-2 flex items-center gap-x-2",
                        div { class: "rounded-full bg-white w-10 h-10" }
                        span { "USER NAME" }
                    }
                    Link {
                        class: "bg-transparent border border-white px-5 py-2.5 text-white hover:bg-white hover:text-black transition-all",
                        to: Route::HomePage { lang, agit_id: 0 },
                        "{tr.my_agit}"
                    }
                }
            } else {
                button {
                    class: "bg-white text-black px-5 py-2.5 rounded-md hover:bg-gray-200 transition-all",
                    onclick: move |_| {
                        popup.with_title("Choose Blockchain").open(BlockchainPopup());
                    },
                    "{tr.login}"
                }
            }
        }
    }
}
