mod error;
mod tracing;
mod wrapper;

pub mod prelude {
    pub use super::error::{Error, Result};
    pub use super::tracing::setup_forest;
    pub use super::wrapper::W;
    pub use tracing::{debug, error, info, instrument, span, trace, warn};
}
