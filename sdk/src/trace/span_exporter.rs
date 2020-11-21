use api::tracing::span::Span;
use api::tracing::tracer_provider::TracerProvider;

#[derive(Debug, PartialEq)]
pub enum Exporter {
    Jaeger,
    Zipkin,
    Prometheus,
    OpenTelemetryProtocol,
    StandardOutput,
    InMemory,
}

pub enum ExportResult {
    Success,
    Failure,
}

pub trait SpanExporter {
    // fn export(&self) -> Vec<Span>;
    fn export(batch: TracerProvider) -> ExportResult;
    fn shutdown();
}

// impl SpanExporter for TracerProvider {
//     // fn export(&self) -> Vec<Span> {
//     //     self.tracer.trace.to_vec()
//     // }
//     fn export(batch: TracerProvider) -> Vec<Span> {
//         // self.tracer.trace.to_vec()
//         batch.tracer.trace.to_vec()
//     }
//
//     fn shutdown() {
//         unimplemented!()
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::trace::tracer_provider::TracerCreation;
//
//     #[test]
//     fn export() {
//         let mut global = TracerProvider::default();
//         let test_trace_one_name = "test_trace_one";
//         let test_trace_one_version = "1.0.0";
//         global.create_tracer(test_trace_one_name, test_trace_one_version);
//         let test_trace_two_name = "test_trace_two";
//         let test_trace_two_version = "1.0.0";
//         global.create_tracer(test_trace_two_name, test_trace_two_version);
//         assert_eq!(global.tracer.trace.len(), 2);
//         let mut export = global.export();
//         assert_eq!(export.len(), 2);
//     }
// }
