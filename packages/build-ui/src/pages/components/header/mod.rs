use crate::{components::ServiceLogo, routes::Route};
use dioxus::prelude::*;
use dioxus_popup::PopupService;
use dioxus_translate::{Language, translate};
use i18n::HeaderTranslate;
mod i18n;

#[allow(dead_code)]
#[derive(Clone, PartialEq)]
enum Blockchain {
    Ethereum,
    Solana,
}

#[allow(dead_code)]
#[derive(Clone, PartialEq)]
struct Wallet {
    name: &'static str,
    image_url: &'static str,
}

struct ValidationService;

impl ValidationService {
    fn is_valid_display_name(name: &str) -> bool {
        // Name should be between 3 and 20 characters
        name.len() >= 3 && name.len() <= 20
    }

    fn is_valid_short_url(url: &str) -> bool {
        if !url.starts_with("https://") {
            return false;
        }

        let domain_part = url.strip_prefix("https://").unwrap_or("");

        if domain_part.is_empty() || !domain_part.contains('.') {
            return false;
        }

        let parts: Vec<&str> = domain_part.split('.').collect();

        if parts.len() < 2 {
            return false;
        }

        let domain = parts[0];
        let tld = parts.last().unwrap();

        if domain.is_empty() || tld.is_empty() {
            return false;
        }

        if !tld.chars().all(|c| c.is_alphabetic()) {
            return false;
        }

        true
    }
}

#[allow(dead_code)]
fn get_wallets(blockchain: Option<&Blockchain>) -> Vec<Wallet> {
    match blockchain {
        Some(Blockchain::Ethereum) => vec![
            Wallet {
                name: "MetaMask",
                image_url: "https://icons8.com/icon/Oi106YG9IoLv/metamask-logo",
            },
            Wallet {
                name: "Trust Wallet",
                image_url: "https://icons8.com/icon/Oi106YG9IoLv/metamask-logo",
            },
            Wallet {
                name: "MetaMask",
                image_url: "https://icons8.com/icon/Oi106YG9IoLv/metamask-logo",
            },
            Wallet {
                name: "Trust Wallet",
                image_url: "https://icons8.com/icon/Oi106YG9IoLv/metamask-logo",
            },
            Wallet {
                name: "MetaMask",
                image_url: "https://icons8.com/icon/Oi106YG9IoLv/metamask-logo",
            },
        ],
        Some(Blockchain::Solana) => vec![
            Wallet {
                name: "Phantom",
                image_url: "https://cryptologos.cc/logos/solana-sol-logo.svg",
            },
            Wallet {
                name: "Solflare",
                image_url: "https://cryptologos.cc/logos/solana-sol-logo.svg",
            },
            Wallet {
                name: "Phantom",
                image_url: "https://cryptologos.cc/logos/solana-sol-logo.svg",
            },
            Wallet {
                name: "Solflare",
                image_url: "https://cryptologos.cc/logos/solana-sol-logo.svg",
            },
            Wallet {
                name: "Phantom",
                image_url: "https://cryptologos.cc/logos/solana-sol-logo.svg",
            },
        ],
        None => Vec::new(),
    }
}

#[component]
pub fn BlockchainPopup() -> Element {
    let mut popup: PopupService = use_context();
    let mut selected_blockchain = use_signal(|| None);

    rsx! {
    if popup.is_opened() {
            div {
                class: "flex flex-col gap-4 mt-2 w-[300px]",
                button {
                    class: "flex gap-4 px-4 py-3 border text-white hover:bg-[#1CCB93] hover:border-[#1CCB93]",
                    onclick: move |_| {
                        selected_blockchain.set(Some(Blockchain::Ethereum));
                        popup.with_title("Choose Wallet").open(rsx! {
                            WalletPopup { selected_blockchain }
                        });
                    },
                    img { class: "w-8 h-8", src: "https://cryptologos.cc/logos/ethereum-eth-logo.svg" }
                    span { class: "", "Ethereum" }
                }
                button {
                    class: "flex items-center gap-4 px-4 py-3 border  text-white hover:bg-[#1CCB93]",
                    onclick: move |_| {
                        selected_blockchain.set(Some(Blockchain::Solana));
                        popup.with_title("Choose Wallet").open(rsx! {
                            WalletPopup { selected_blockchain }
                        });
                    },
                    img { class: "w-8 h-8", src: "https://cryptologos.cc/logos/solana-sol-logo.svg" }
                    span { "Solana" }
                }
                button {
                    class: "flex gap-4 px-4 py-3 border text-white hover:bg-[#1CCB93] hover:border-[#1CCB93]",
                    onclick: move |_| {
                        selected_blockchain.set(Some(Blockchain::Ethereum));
                        popup.with_title("Choose Wallet").open(rsx! {
                            WalletPopup { selected_blockchain }
                        });
                    },
                    img { class: "w-8 h-8", src: "https://cryptologos.cc/logos/ethereum-eth-logo.svg" }
                    span { class: "", "Ethereum" }
                }
                button {
                    class: "flex items-center gap-4 px-4 py-3 border  text-white hover:bg-[#1CCB93] hover:border-[#1CCB93]",
                    onclick: move |_| {
                        selected_blockchain.set(Some(Blockchain::Solana));
                        popup.with_title("Choose Wallet").open(rsx! {
                            WalletPopup { selected_blockchain }
                        });
                    },
                    img { class: "w-8 h-8", src: "https://cryptologos.cc/logos/solana-sol-logo.svg" }
                    span { "Solana" }
                }
                button {
                    class: "flex gap-4 px-4 py-3 border text-white hover:bg-[#1CCB93] hover:border-[#1CCB93]",
                    onclick: move |_| {
                        selected_blockchain.set(Some(Blockchain::Ethereum));
                        popup.with_title("Choose Wallet").open(rsx! {
                            WalletPopup { selected_blockchain }
                        });
                    },
                    img { class: "w-8 h-8", src: "https://cryptologos.cc/logos/ethereum-eth-logo.svg" }
                    span { class: "", "Ethereum" }
                }
            }
        }
    }
}

#[component]
fn WalletPopup(selected_blockchain: Signal<Option<Blockchain>>) -> Element {
    let mut popup: PopupService = use_context();
    let mut selected_wallet = use_signal(|| None::<Wallet>);
    let wallets = get_wallets(selected_blockchain().as_ref());

    rsx! {
    if popup.is_opened() {
            div {
                class: "flex flex-col gap-4 mt-4 w-[300px]",
                for wallet in wallets.clone().into_iter() {
                    button {
                        class: "flex items-center gap-4 px-4 py-3 border  text-white hover:bg-[#1CCB93] hover:border-[#1CCB93]",
                        onclick: {
                            let wallet_clone = wallet.clone();
                            move |_| {
                                println!("Connecting to {}...", wallet_clone.name.to_string());
                                selected_wallet.set(Some(wallet_clone.clone()));
                                popup.open(rsx! {
                                    NameSettingPopup {  }
                                }).with_title("You are almost there!");
                            }
                        },
                        img { class: "w-8 h-8", src: "{wallet.image_url}" }
                        span { "{wallet.name}" }
                    }
                }
            }
        }
    }
}

#[component]
fn NameSettingPopup() -> Element {
    let mut popup: PopupService = use_context();

    let mut display_name = use_signal(|| String::new());
    let mut short_url = use_signal(|| String::new());
    let mut terms_accepted = use_signal(|| false);
    let mut newsletter_accepted = use_signal(|| false);

    let mut is_name_valid = use_signal(|| false);
    let mut is_short_url_valid = use_signal(|| false);

    let is_form_valid =
        *is_name_valid.read() && *is_short_url_valid.read() && *terms_accepted.read();

    use_effect(move || {
        is_name_valid.set(ValidationService::is_valid_display_name(
            &display_name.read(),
        ));
        is_short_url_valid.set(ValidationService::is_valid_short_url(&short_url.read()));
    });

    rsx! {
    if popup.is_opened() {
        div {
            class: "flex flex-col gap-1 mt-2",
            p { class: "text-white/80 mt-2 text-nowrap", "Choose a display name and enter your email address" }

            div { class: "mt-6",
                label { class: "block text-white mb-2", "Agit Name" }
                input {
                    class: "w-full bg-transparent border border-neutral-700 text-white p-3 rounded-sm focus:outline-none focus:border-[#1CCB93]",
                    placeholder: "name120",
                    value: "{display_name}",
                    oninput: move |e| {
                        display_name.set(e.value().clone());
                        is_name_valid.set(ValidationService::is_valid_display_name(&e.value()));
                    }
                }
            }

            div { class: "mt-6",
                label { class: "block text-white mb-2", "Short URL" }
                input {
                    class: "w-full bg-transparent border border-neutral-700 text-white p-3 rounded-sm focus:outline-none focus:border-[#1CCB93]",
                    placeholder: "https://dagit.com",
                    value: "{short_url}",
                    oninput: move |e| {
                        short_url.set(e.value().clone());
                        is_short_url_valid.set(ValidationService::is_valid_short_url(&e.value()));


                    }
                }
            }

            div { class: "mt-3 flex items-center gap-2",
                input {
                    r#type: "checkbox",
                    checked: "{terms_accepted}",
                    onchange: move |_| {
                        let current = *terms_accepted.read();
                        terms_accepted.set(!current);
                    }
                }
                label { class: "text-white text-nowrap",
                    "I have read and accept the "
                    span { class: "font-bold cursor-pointer", "Terms of Service" }
                    "."
                }
            }

            div { class: "mt-3 flex items-center gap-2",
                input {
                    r#type: "checkbox",
                    checked: "{newsletter_accepted}",
                    onchange: move |_| {
                        let current = *newsletter_accepted.read();
                        newsletter_accepted.set(!current);
                    }
                }
                label { class: "text-white text-nowrap", "I want to receive announcements and news from d.Agit." }
            }

            div { class: "mt-6",
                button {
                    class: if is_form_valid {
                        "w-full bg-white text-black py-3 font-medium cursor-pointer"
                    } else {
                        "w-full bg-gray-500 text-gray-300 py-3 font-medium cursor-not-allowed opacity-50"
                    },
                    disabled: !is_form_valid,
                    onclick: move |_| {
                        if is_form_valid {
                            println!("Sign-up completed! Name: {}, Short-URL: {}", display_name.read(), short_url.read());
                            popup.close();
                            // TODO Account creation func...
                        }
                    },
                    "Finished Sign-up"
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
                    class: "text-white cursor-pointer px-5 py-2.5 transition-all",
                    onclick: move |_| {
                        popup.open(rsx! {
                            BlockchainPopup {
                                // lang,
                            }
                        }).with_title("Choose Blockchain");
                    },
                    "{tr.login}"
                }
            }
        }
    }
}
