pub mod agit;
pub mod artist;
pub mod artwork;
pub mod collection;

pub mod prelude {
    pub use super::agit::*;
    pub use super::artist::*;
    pub use super::artwork::*;
    pub use super::collection::*;
}
