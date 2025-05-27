use base64::{Engine, engine::general_purpose};
use common::tables::users::User;
use rest_api::{Signature, Signer, signature::SignatureAlgorithm};
use ring::signature::{Ed25519KeyPair, KeyPair};

use crate::dagit_tests::{TestContext, setup};

fn generate_private_key_ed25519() -> Vec<u8> {
    let seed = ring::rand::SystemRandom::new();
    let key_pair = ring::signature::Ed25519KeyPair::generate_pkcs8(&seed)
        .expect("Could not generate a key pair.");
    key_pair.as_ref().to_vec()
}

fn generate_public_key(private_key: &[u8]) -> Vec<u8> {
    let key_pair =
        ring::signature::Ed25519KeyPair::from_pkcs8(private_key).expect("Invalid private key");
    let public_key_bytes = key_pair.public_key().as_ref().to_vec();

    public_key_bytes
}
struct TestSigner {
    private_key: Vec<u8>,
}
impl TestSigner {
    pub fn new(private_key: Vec<u8>) -> Self {
        TestSigner { private_key }
    }
}
impl Signer for TestSigner {
    fn sign(&self, msg: &str) -> Result<Signature, Box<dyn std::error::Error>> {
        let key_pair = Ed25519KeyPair::from_pkcs8(&self.private_key)?;
        let signature = key_pair.sign(msg.as_bytes()).as_ref().to_vec();
        let public_key = generate_public_key(&self.private_key);

        Ok(Signature {
            signature,
            algorithm: SignatureAlgorithm::EdDSA,
            public_key,
        })
    }
    fn signer(&self) -> String {
        let public_key = generate_public_key(&self.private_key);
        let v = general_purpose::STANDARD.encode(&public_key);
        v
    }
}

// #[tokio::test]
// async fn test_user_signup() {
//     let TestContext { endpoint, .. } = setup().await.unwrap();
//     let signer = TestSigner::new(generate_private_key_ed25519());
//     let domain = option_env!("AUTH_DOMAIN").unwrap_or("dagit.club");
//     rest_api::set_signer(Box::new(signer));
//     rest_api::set_message(domain.to_string());
//     let user_client = User::get_client(&endpoint);
//     let id = uuid::Uuid::new_v4().to_string();
//     let res = user_client
//         .signup(
//             common::tables::users::AuthProvider::Google,
//             format!("test-user-{id}"),
//             format!("user-{id}@test.com"),
//             format!("test-user"),
//             None,
//             chrono::Utc::now().timestamp(),
//             None,
//         )
//         .await;

//     assert!(res.is_ok(), "Failed to signup: {:?}", res.err());
//     let signup_user = res.unwrap();
//     let res = user_client
//         .login(
//             common::tables::users::AuthProvider::Google,
//             format!("test-user-{id}"),
//         )
//         .await;
//     assert!(res.is_ok(), "Failed to login: {:?}", res.err());
//     let login_user = res.unwrap();
//     assert_eq!(signup_user, login_user);
//     rest_api::remove_signer();
// }
