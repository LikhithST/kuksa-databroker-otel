use opentelemetry::{KeyValue, runtime};
use opentelemetry::sdk::{Resource, trace, trace::BatchConfig};
use opentelemetry::trace::TraceError;
use opentelemetry_otlp::WithExportConfig;

pub fn init_trace() -> Result<trace::Tracer, TraceError> {

    // let batch_processor = trace::BatchSpanProcessor::builder(exporter, runtime::Tokio)
    //     .with_max_queue_size(2048)  // Increase queue size
    //     .with_max_export_batch_size(512)  // Max batch size for exports
    //     .with_scheduled_delay(std::time::Duration::from_millis(100))  // Export every 100ms
    //     .build();

    opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint("http://localhost:4317"),
        ).with_batch_config(trace::BatchConfig::default().with_max_queue_size(8192))
        .with_trace_config(
            trace::config().with_resource(Resource::new(vec![KeyValue::new(
                opentelemetry_semantic_conventions::resource::SERVICE_NAME,
                "kuksa-rust-app",
            )])),
        )
        .install_batch(runtime::Tokio)
}
