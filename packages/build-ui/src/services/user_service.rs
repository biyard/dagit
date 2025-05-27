use crate::config;
use bdk::prelude::*;
use common::error::ServiceError;
use common::tables::agits::Agit;
use common::tables::users::AuthProvider;
use common::{Result, tables::users::User};
use google_wallet::{FirebaseWallet, WalletEvent};
use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq)]
pub enum Status {
    Signup {
        principal: String,
        profile_url: Option<String>,
        nickname: Option<String>,
        email: Option<String>,
    },
    Login,
    Logout,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Signup { .. } => write!(f, "Signup"),
            Status::Login => write!(f, "Login"),
            Status::Logout => write!(f, "Logout"),
        }
    }
}
#[derive(Debug, Copy, Clone, Translate, Eq, PartialEq)]
pub enum Chain {
    Bitcoin,
    Ethereum,
    InternetComputer,
}

#[derive(Debug, Clone, Copy, Translate, Eq, PartialEq)]
pub enum Wallet {
    Google,
    MetaMask,
}

#[derive(Debug, Clone)]
pub enum WalletSigner {
    Firebase(FirebaseWallet),
}

pub fn get_firebase_wallet() -> google_wallet::FirebaseWallet {
    #[cfg(not(feature = "web"))]
    let firebase = google_wallet::FirebaseWallet::default();

    #[cfg(feature = "web")]
    let firebase = {
        let conf = config::get();
        let mut firebase = google_wallet::FirebaseWallet::new(
            conf.firebase.api_key.clone(),
            conf.firebase.auth_domain.clone(),
            conf.firebase.project_id.clone(),
            conf.firebase.storage_bucket.clone(),
            conf.firebase.messaging_sender_id.clone(),
            conf.firebase.app_id.clone(),
            conf.firebase.measurement_id.clone(),
        );
        let _ = firebase.try_setup_from_storage();
        tracing::debug!("get_firebase_wallet: firebase={:?}", firebase);
        firebase
    };
    firebase
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub provider: AuthProvider,
    pub principal: String,
    pub email: String,
    pub nickname: String,
    pub profile_url: Option<String>,
    pub agits: Vec<Agit>,
}

impl From<User> for UserInfo {
    fn from(user: User) -> Self {
        Self {
            provider: user.provider,
            principal: user.address,
            email: user.email,
            nickname: user.name,
            profile_url: user.profile_url,
            agits: user.agits,
        }
    }
}
#[derive(Debug, Clone, Copy, DioxusController)]
pub struct UserService {
    pub signer: Signal<Option<WalletSigner>>,
    pub user_info: Signal<Option<UserInfo>>,
    pub current_agit: Signal<Option<Agit>>,
}

impl UserService {
    pub fn init() -> std::result::Result<(), RenderError> {
        let default = use_server_future(move || async move {
            let client = User::get_client(config::get().api_url);
            match client.refresh().await {
                Ok(user) => Some(UserInfo::from(user)),
                _ => None,
            }
        })?;
        let user_info: Signal<Option<UserInfo>> =
            use_signal(|| default.value()().unwrap_or_default());
        let agit =
            user_info().and_then(|user_info| user_info.agits.get(0).map(|agit| agit.clone()));
        let user = UserService {
            signer: use_signal(|| None),
            user_info,
            current_agit: use_signal(|| agit),
        };
        use_context_provider(move || user);
        Ok(())
    }

    pub fn update_email(&mut self, email: String) {
        if let Some(user_info) = self.user_info() {
            self.user_info.set(Some(UserInfo { email, ..user_info }));
        }
    }
    pub fn update_nickname(&mut self, nickname: String) {
        if let Some(user_info) = self.user_info() {
            self.user_info.set(Some(UserInfo {
                nickname,
                ..user_info
            }));
        }
    }

    pub async fn signup(
        &mut self,
        agit_title: String,
        email: String,
        nickname: String,
        terms_agreed_at: i64,
        ads_agreed_at: Option<i64>,
    ) -> Result<Status> {
        let client = User::get_client(config::get().api_url);
        if let Some(user_info) = self.user_info() {
            let user_info = user_info.clone();
            let res = client
                .signup(
                    user_info.provider,
                    user_info.principal,
                    email,
                    nickname,
                    user_info.profile_url,
                    agit_title,
                    terms_agreed_at,
                    ads_agreed_at,
                )
                .await;
            match res {
                Ok(user) => {
                    self.current_agit
                        .set(user.agits.get(0).map(|agit| agit.clone()));
                    Ok(Status::Login)
                }
                Err(_) => {
                    self.user_info.set(None);
                    Err(ServiceError::SignupFailed)
                }
            }
        } else {
            Err(ServiceError::Unknown(
                "User info is not set. Please login first.".to_string(),
            ))
        }
    }
    pub async fn login(&mut self, chain: Chain, wallet: Wallet) -> Result<Status> {
        tracing::debug!("UserService::login: chain={:?}, wallet={:?}", chain, wallet);
        match chain {
            Chain::InternetComputer => match wallet {
                Wallet::Google => {
                    tracing::debug!("UserService::login: Google");
                    return self.firebase_login().await;
                }
                _ => {
                    btracing::error!("UserService::login: Unsupported wallet");
                }
            },
            _ => {
                btracing::error!("UserService::login: Unsupported chain");
            }
        };
        Err(ServiceError::Unsupported(
            "Unsupported chain or wallet".to_string(),
        ))
    }
    pub async fn firebase_login(&mut self) -> Result<Status> {
        let mut firebase = get_firebase_wallet();
        let (evt, user_info, principal) = match firebase.request_wallet_with_google().await {
            Ok(evt) => {
                let principal = firebase.get_principal();
                let user_info = firebase.get_user_info();
                if user_info.is_none() || principal.is_empty() {
                    return Err(ServiceError::Unauthorized);
                }
                (evt, user_info.unwrap(), principal)
            }
            Err(e) => {
                tracing::error!("Error during login: {:?}", e);
                return Err(ServiceError::Unauthorized);
            }
        };
        tracing::debug!("UserService::firebase_login: evt={:?}", evt);
        let next_status = match evt {
            WalletEvent::Login => {
                tracing::debug!("UserService::firebase_login: login");

                self.user_info.set(Some(UserInfo {
                    provider: AuthProvider::Google,
                    principal: principal.clone(),
                    email: user_info.0.clone(),
                    nickname: user_info.1.clone(),
                    profile_url: Some(user_info.2.clone()),
                    agits: vec![],
                }));

                let client = User::get_client(config::get().api_url);
                let res = client
                    .login(
                        common::tables::users::AuthProvider::Google,
                        principal.clone(),
                    )
                    .await;
                match res {
                    Ok(user) => {
                        self.current_agit
                            .set(user.agits.get(0).map(|agit| agit.clone()));

                        self.user_info.set(Some(UserInfo {
                            provider: AuthProvider::Google,
                            principal: user.address,
                            email: user.email,
                            nickname: user.name,
                            profile_url: user.profile_url,
                            agits: user.agits,
                        }));
                        Status::Login
                    }
                    Err(e) => {
                        tracing::error!("Error during login: {:?}", e);
                        Status::Signup {
                            principal,
                            email: Some(user_info.0),
                            nickname: Some(user_info.1),
                            profile_url: Some(user_info.2),
                        }
                    }
                }
            }
            WalletEvent::Signup => {
                tracing::debug!("UserService::firebase_login: signup");
                self.user_info.set(Some(UserInfo {
                    provider: AuthProvider::Google,
                    principal: principal.clone(),
                    email: user_info.0.clone(),
                    nickname: user_info.1.clone(),
                    profile_url: Some(user_info.2.clone()),
                    agits: vec![],
                }));
                Status::Signup {
                    principal,
                    email: Some(user_info.0),
                    nickname: Some(user_info.1),
                    profile_url: Some(user_info.2),
                }
            }
            WalletEvent::Logout => {
                tracing::debug!("UserService::firebase_login: logout");
                self.user_info.set(None);
                rest_api::remove_signer();
                Status::Logout
            }
        };

        if next_status == Status::Login {
            self.signer.set(Some(WalletSigner::Firebase(firebase)));
            rest_api::set_signer(Box::new(self.clone()));
        } else {
            self.signer.set(None);
        }
        Ok(next_status)
    }
}

impl rest_api::Signer for UserService {
    fn signer(&self) -> String {
        match (self.signer)() {
            Some(WalletSigner::Firebase(firebase)) => firebase.get_principal(),
            None => "".to_string(),
        }
    }

    fn sign(
        &self,
        msg: &str,
    ) -> std::result::Result<rest_api::Signature, Box<dyn std::error::Error>> {
        tracing::debug!("UserService::sign: msg={}", msg);
        match (self.signer)() {
            Some(WalletSigner::Firebase(firebase)) => {
                if !firebase.get_login() {
                    tracing::debug!("UserService::sign: not login {firebase:?}");
                    return Err(ServiceError::Unauthorized.into());
                }

                let sig = firebase.sign(msg);
                if sig.is_none() {
                    return Err(ServiceError::Unauthorized.into());
                }
                let sig = rest_api::Signature {
                    signature: sig.unwrap().as_ref().to_vec(),
                    public_key: firebase.public_key().unwrap_or_default(),
                    algorithm: rest_api::signature::SignatureAlgorithm::EdDSA,
                };

                return Ok(sig);
            }
            None => {
                tracing::debug!("UserService::sign: no signer");
                return Err(ServiceError::Unauthorized.into());
            }
        }
    }
}
