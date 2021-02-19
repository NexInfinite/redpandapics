mod home;
mod endpoints;
mod boop;
mod tired;
mod fact;
mod funny;
mod tall;
mod random;

pub use crate::endpoints::home::home_mod::home_endpoints;
pub use crate::endpoints::endpoints::endpoints_mod::endpoints_endpoint;
pub use crate::endpoints::boop::boop_mod::boop_endpoint;
pub use crate::endpoints::tired::tired_mod::tired_endpoint;
pub use crate::endpoints::fact::fact_mod::fact_endpoint;
pub use crate::endpoints::funny::funny_mod::funny_endpoint;
pub use crate::endpoints::tall::tall_mod::tall_endpoint;
pub use crate::endpoints::random::random_mod::random_endpoint;