use super::*;
use crate::dagit_tests::{TestContext, setup};
use common::tables::agits::AgitQuery;

const IMAGE_URL: &str =
    "https://metadata.dev.dagit.club/images/72a11429-20c0-4d62-8cde-ff3d4d5dc0bb";

#[tokio::test]
async fn test_create_agit() {
    let TestContext { endpoint, .. } = setup().await.unwrap();

    let client = Agit::get_client(&endpoint);
    let result = client
        .create(
            "TEST_AGIT".to_string(),
            "TEST_AGIT_DESCRIPTION".to_string(),
            None,
            IMAGE_URL.to_string(),
            IMAGE_URL.to_string(),
        )
        .await;

    assert!(result.is_ok(), "Failed to create agit: {:?}", result.err());
}

#[tokio::test]
async fn test_query_agit() {
    let TestContext { endpoint, .. } = setup().await.unwrap();

    let client = Agit::get_client(&endpoint);

    let param = AgitQuery {
        bookmark: None,
        size: 10,
    };
    let result = client.query(param).await;

    assert!(result.is_ok(), "Failed to query agit: {:?}", result.err());
}

#[tokio::test]
async fn test_delete_agit() {
    let TestContext { endpoint, .. } = setup().await.unwrap();

    let client = Agit::get_client(&endpoint);

    let result = client
        .create(
            "TEST_AGIT".to_string(),
            "TEST_AGIT_DESCRIPTION".to_string(),
            None,
            IMAGE_URL.to_string(),
            IMAGE_URL.to_string(),
        )
        .await;
    assert!(result.is_ok(), "Failed to create agit: {:?}", result.err());
    let agit = result.unwrap();

    let result = client.delete(agit.id).await;

    assert!(result.is_ok(), "Failed to delete agit: {:?}", result.err());
}

// #[tokio::test]
// async fn test_update_agit() {
//     let TestContext { endpoint, .. } = setup().await.unwrap();

//     let client = Agit::get_client(&endpoint);
//     let result = client
//         .create(
//             "UPDATE_TEST_AGIT".to_string(),
//             "DESCRIPTION".to_string(),
//             None,
//             IMAGE_URL.to_string(),
//             IMAGE_URL.to_string(),
//         )
//         .await;
//     assert!(result.is_ok(), "Failed to create agit: {:?}", result.err());
//     let agit = result.unwrap();
//     let result = client
//         .update(
//             agit.id,
//             agit.title,
//             agit.description,
//             None,
//             agit.banner_url,
//             agit.logo_url,
//             true,
//         )
//         .await;
//     assert!(result.is_ok(), "Failed to update agit: {:?}", result.err());
//     let result = result.unwrap();
//     assert_eq!(result.title, "UPDATE_TEST_AGIT");
//     assert_eq!(result.authorized, true);
// }
