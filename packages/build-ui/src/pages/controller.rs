use bdk::prelude::{dioxus_popup::PopupService, *};
use common::tables::agits::Agit;

use crate::pages::components::{
    BuildAgitModal, BuildAgitResult, ConnectWalletModal, UserInfoModal, UserInfoResult,
};
use crate::routes::Route;
use crate::services::user_service::{Chain, Status, UserInfo, UserService, Wallet};

use super::components::BlockchainSelectionModal;
use super::i18n::{
    BlockchainSelectionModalTranslate, BuildAgitModalTranslate, ConnectWalletModalTranslate,
    UserInfoModalTranslate,
};

#[derive(Debug, Clone, Copy, DioxusController)]
pub struct Controller {
    lang: Language,
    popup: PopupService,
    user: UserService,

    email: Signal<Option<String>>,
    nickname: Signal<Option<String>>,
    terms_agreed_at: Signal<Option<i64>>,
    ads_agreed_at: Signal<Option<i64>>,
}

impl Controller {
    pub fn get_user_info(&self) -> Option<(UserInfo, Agit)> {
        if let Some(agit) = self.user.current_agit() {
            Some((self.user.user_info().unwrap(), agit))
        } else {
            None
        }
    }
    pub fn new(lang: Language) -> Result<Self, RenderError> {
        let popup: PopupService = use_context();
        let user: UserService = use_context();
        let ctrl = Controller {
            lang,
            popup,
            user,
            email: use_signal(|| None),
            nickname: use_signal(|| None),
            terms_agreed_at: use_signal(|| None),
            ads_agreed_at: use_signal(|| None),
        };
        Ok(ctrl)
    }

    pub fn open_select_blockchain_modal(&self) {
        let mut popup = self.popup.clone();
        let tr: BlockchainSelectionModalTranslate = translate(&self.lang);
        let ctrl = self.clone();
        popup
            .open(rsx! {
                BlockchainSelectionModal {
                    lang: self.lang,
                    on_select: move |chain| {
                        tracing::debug!("Selected blockchain: {:?}", chain);
                        ctrl.open_connect_wallet_modal(chain);
                    },
                }
            })
            .with_id("select_blockchain_modal")
            .with_title(tr.title);
    }

    fn open_connect_wallet_modal(&self, chain: Chain) {
        let mut popup = self.popup.clone();
        let tr: ConnectWalletModalTranslate = translate(&self.lang);
        let mut ctrl = self.clone();
        let nav = use_navigator();

        popup
            .open(rsx! {
                ConnectWalletModal {
                    lang: self.lang,
                    on_select: move |wallet: Wallet| async move {
                        tracing::debug!("Selected wallet: {:?}", wallet);
                        if let Ok(status) = ctrl.user.login(chain, wallet).await {
                            tracing::debug!("Login status: {:?}", ctrl.user);
                            match status {
                                Status::Login => {
                                    if let Some(agit) = ctrl.user.current_agit() {
                                        nav.push(Route::HomePage {
                                            lang: ctrl.lang,
                                            agit_id: agit.id,
                                        });
                                    }
                                    popup.close();
                                }
                                Status::Signup { principal: _, profile_url: _, nickname, email } => {
                                    (ctrl.nickname).set(nickname);
                                    ctrl.email.set(email);
                                    ctrl.open_user_info_modal();
                                }
                                _ => {
                                    popup.close();
                                }
                            }
                        }
                    },
                }
            })
            .with_id("connect_wallet_modal")
            .with_title(tr.title);
    }

    // // New method that takes the selected blockchain and wallet
    fn open_user_info_modal(&self) {
        let mut popup = self.popup.clone();
        let tr: UserInfoModalTranslate = translate(&self.lang);
        let mut ctrl = self.clone();
        popup
            .open(rsx! {
                UserInfoModal {
                    nickname: ctrl.nickname(),
                    email: ctrl.email(),
                    lang: self.lang,
                    on_button_click: move |result: UserInfoResult| async move {
                        tracing::debug!("Selected wallet: {:?}", result);
                        ctrl.terms_agreed_at.set(Some(result.terms_agreed_at));
                        ctrl.ads_agreed_at.set(result.ads_agreed_at);
                        ctrl.email.set(Some(result.email));
                        ctrl.nickname.set(Some(result.nickname));
                        ctrl.open_build_agit_modal();
                    },
                }
            })
            .with_id("user_info_modal")
            .with_title(tr.title);
    }

    #[allow(dead_code)]
    fn open_build_agit_modal(&self) {
        let mut popup = self.popup.clone();
        let tr: BuildAgitModalTranslate = translate(&self.lang);
        let mut ctrl = self.clone();
        let lang = self.lang;
        let nav = use_navigator();
        popup
            .open(rsx! {
                BuildAgitModal {
                    lang: self.lang,
                    on_button_click: move |result: BuildAgitResult| async move {
                        tracing::debug!("Result: {:?}", result);
                        match ctrl
                            .user
                            .signup(
                                result.name,
                                ctrl.email().unwrap_or_default(),
                                ctrl.nickname().unwrap_or_default(),
                                ctrl.terms_agreed_at().unwrap(),
                                ctrl.ads_agreed_at(),
                            )
                            .await
                        {
                            Ok(status) => {
                                tracing::debug!("Signup status: {:?}", status);
                                match status {
                                    Status::Login => {
                                        if let Some(agit) = ctrl.user.current_agit() {
                                            nav.push(Route::HomePage {
                                                lang: ctrl.lang,
                                                agit_id: agit.id,
                                            });
                                        }
                                        popup.close();
                                    }
                                    e @ _ => {
                                        btracing::error!("Error: {e}");
                                        popup.close();
                                    }
                                }
                            }
                            Err(e) => {
                                btracing::e!(lang, e);
                                popup.close();
                            }
                        }
                    },
                }
            })
            .with_id("build_agit_modal")
            .with_title(tr.title);
    }
}
