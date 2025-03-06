// 404 Page
mod _route;

// Root Page
pub mod components;
mod i18n;
mod layout;
pub mod page;

pub use _route::*;
pub use layout::RootLayout;
pub use page::RootPage;
