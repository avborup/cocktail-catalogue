use axum::body::Body;
use tower_http::{
    classify::{ServerErrorsAsFailures, SharedClassifier},
    trace::TraceLayer,
};
use tracing::{Span, Subscriber};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{fmt::MakeWriter, layer::SubscriberExt, EnvFilter, Registry};
use uuid::Uuid;

/// Get a `tracing` subscriber that writes to `sink`. By default, the `RUST_LOG`
/// environment variable is used to configure the subscriber, but if it's not
/// set, `fallback_env_filter` is used instead.
pub fn get_subscriber_with_fallback<Sink>(
    fallback_env_filter: &str,
    sink: Sink,
) -> impl Subscriber + Send + Sync
where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(fallback_env_filter));

    let formatting_layer = BunyanFormattingLayer::new(env!("CARGO_PKG_NAME").into(), sink);

    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

pub(crate) fn make_http_span_layer() -> TraceLayer<
    SharedClassifier<ServerErrorsAsFailures>,
    impl Fn(&axum::http::Request<Body>) -> Span + Clone,
> {
    TraceLayer::new_for_http().make_span_with(|req: &axum::http::Request<Body>| {
        let request_id = Uuid::new_v4();

        tracing::debug_span!(
            "request",
            id = %request_id,
            method = %req.method(),
            uri = %req.uri(),
        )
    })
}
