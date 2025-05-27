use super::*;
use crate::{
    controllers::v1::collector,
    dagit_tests::{TestContext, setup},
};

use common::tables::collectors::CollectorQuery;

const IMAGE_URL: &str =
    "https://metadata.dev.dagit.club/images/72a11429-20c0-4d62-8cde-ff3d4d5dc0bb";

async fn create_collector() -> Collector {
    let TestContext { endpoint, user, .. } = setup().await.unwrap();
    let client = Collector::get_client(&endpoint);
    let result = client
        .create(
            "TEST_Collection".to_string(),
            "TEDESCRIPTION".to_string(),
            None,
            IMAGE_URL.to_string(),
            IMAGE_URL.to_string(),
            false,
            247,
            vec!["1234567890".to_string(), "1234567890".to_string()],
            "0x1234567890abcdef".to_string(),
            20.5,
            "Sent".to_string(),
        )
        .await;

    assert!(
        result.is_ok(),
        "Failed to create collector: {:?}",
        result.err()
    );
    let result = result.unwrap();
    result
}
