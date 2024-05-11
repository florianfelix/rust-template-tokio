use tracing_forest::util::*;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, Registry};

pub async fn setup_forest() -> tracing::Span {
    Registry::default().with(ForestLayer::default()).init();

    let span = tracing::span!(tracing::Level::DEBUG, "main span");
    span
}
