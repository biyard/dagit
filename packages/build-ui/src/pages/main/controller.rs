#![allow(non_snake_case)]
#![allow(unused)]
use dioxus::prelude::*;
use dioxus_logger::tracing;

#[derive(Clone, Copy)]
pub struct Controller {
    pub images: Resource<Vec<MockImage>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Image {
    pub url: String,
    pub title: String,
    pub tags: Vec<String>,
    pub artist: String,
    pub price: String,
    pub location: String,
    pub agit_name: String,
    pub description: String,
    pub followers: i64,
    pub votes: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct MockImage {
    pub url: String,
    pub author: String,
}

impl Controller {
    pub fn init() -> Result<Self, RenderError> {
        let mock_images: Resource<Vec<MockImage>> = use_server_future(move || async move {
            match reqwest::get("https://picsum.photos/v2/list?limit=50").await {
                Ok(v) => v.json::<Vec<MockImage>>().await.unwrap(),
                Err(e) => {
                    tracing::error!("Error: {:?}", e);
                    vec![]
                }
            }
        })?;
        let ctrl = Self {
            images: mock_images,
        };
        use_context_provider(|| ctrl);

        Ok(ctrl)
    }

    pub fn get_images(&self) -> Vec<Image> {
        let mut v = vec![];

        for i in 0..10 {
            v.push(Image {
                title: format!("Image {}", i),
                url: format!("https://loremflickr.com/400/390?random={}", i),
                tags: vec!["tag1".to_string(), "tag2".to_string()],
                price: format!("{}", i * 1000),
                artist: format!("Artist {}", i),
                location: "Portland OR".to_string(),
                agit_name: "Agit Name".to_string(),
                description: "Explore powerful artworks that amplify voices for equality, diversity, and justice. This collection brings together artists who use their..".to_string(),
                votes: 5_500_000,
                followers: 315_000,
            });
        }
        v
    }
}
