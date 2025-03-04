mod _route;
pub mod collection;
mod main;

pub mod prelude {
    pub use super::_route::NotFoundPage;
    pub use super::collection::CollectionsPage;
    pub use super::main::MainPage;
}
