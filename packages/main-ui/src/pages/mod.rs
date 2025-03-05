mod _route;
mod collection;
mod main;
mod platform;

pub mod prelude {
    pub use super::_route::NotFoundPage;
    pub use super::collection::CollectionsPage;
    pub use super::main::MainPage;
    pub use super::platform::PlatformPage;
}
