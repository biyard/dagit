mod page;

mod analytics;
mod design;
mod extension_tool;
mod hub;
mod management;
mod orders;

pub use analytics::{ReportPage, TrafficPage};
pub use page::HomePage;

pub use design::DesignPage;
pub use extension_tool::ExtensionToolPage;
pub use hub::{DaoPage, FaqPage, OraclePage};
pub use management::{
    ArtistDetailPage, ArtistPage, ArtworkPage, CollectionDetailPage, CollectionPage,
    CollectorDetailPage, CollectorPage, CreateArtistPage, CreateArtworkPage, EditArtistPage,
};
pub use orders::{SalesRequestPage, ShippingLabelPage};
