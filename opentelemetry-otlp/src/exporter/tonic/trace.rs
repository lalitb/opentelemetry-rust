use core::fmt;

use opentelemetry::{otel_debug, trace::TraceError};
use opentelemetry_proto::tonic::collector::trace::v1::{
    trace_service_client::TraceServiceClient, ExportTraceServiceRequest,
};
use opentelemetry_proto::transform::trace::tonic::group_spans_by_resource_and_scope;
use opentelemetry_sdk::trace::{ExportResult, SpanData, SpanExporter};
use tokio::sync::Mutex;
use tonic::{codegen::CompressionEncoding, service::Interceptor, transport::Channel, Request};

use super::BoxInterceptor;

pub(crate) struct TonicTracesClient {
    inner: Option<ClientInner>,
    #[allow(dead_code)]
    // <allow dead> would be removed once we support set_resource for metrics.
    resource: opentelemetry_proto::transform::common::tonic::ResourceAttributesWithSchema,
}

struct ClientInner {
    client: TraceServiceClient<Channel>,
    interceptor: Mutex<BoxInterceptor>,
}

impl fmt::Debug for TonicTracesClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("TonicTracesClient")
    }
}

impl TonicTracesClient {
    pub(super) fn new(
        channel: Channel,
        interceptor: BoxInterceptor,
        compression: Option<CompressionEncoding>,
    ) -> Self {
        let mut client = TraceServiceClient::new(channel);
        if let Some(compression) = compression {
            client = client
                .send_compressed(compression)
                .accept_compressed(compression);
        }

        otel_debug!(name: "TonicsTracesClientBuilt");

        TonicTracesClient {
            inner: Some(ClientInner {
                client,
                interceptor: Mutex::new(interceptor),
            }),
            resource: Default::default(),
        }
    }
}

impl SpanExporter for TonicTracesClient {
    fn export(
        &self,
        batch: Vec<SpanData>,
    ) -> impl std::future::Future<Output = ExportResult> + Send {
        async move {
            let (mut client, metadata, extensions) = match &self.inner {
                Some(inner) => {
                    let (m, e, _) = inner
                        .interceptor
                        .lock()
                        .await // tokio::sync::Mutex doesn't return a poisoned error, so we can safely use the interceptor here
                        .call(Request::new(()))
                        .map_err(|e| TraceError::Other(Box::new(e)))?
                        .into_parts();
                    (inner.client.clone(), m, e)
                }
                None => {
                    return Err(TraceError::Other("exporter is already shut down".into()));
                }
            };

            let resource_spans = group_spans_by_resource_and_scope(batch, &self.resource);

            otel_debug!(name: "TonicsTracesClient.CallingExport");

            client
                .export(Request::from_parts(
                    metadata,
                    extensions,
                    ExportTraceServiceRequest { resource_spans },
                ))
                .await
                .map_err(crate::Error::from)?;

            Ok(())
        }
    }

    fn shutdown(&mut self) {
        let _ = self.inner.take();
    }

    fn set_resource(&mut self, resource: &opentelemetry_sdk::Resource) {
        self.resource = resource.into();
    }
}
