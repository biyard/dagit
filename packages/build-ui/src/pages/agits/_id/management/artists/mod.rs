// mod components;
// mod controller;
// mod i18n;
// mod models;
mod page;

mod create;
pub use create::CreateArtistPage;
mod _id;
pub use _id::{ArtistDetailPage, EditArtistPage};

pub use page::ArtistPage;
