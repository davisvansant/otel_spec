use crate::trace::span_processor::BatchingProcessor;
use crate::trace::span_processor::SpanProcessor;
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

pub struct SpanExporter {
    exporter: Exporter,
    batch: Vec<Span>,
}

impl SpanExporter {
    pub fn init(exporter: Exporter) -> SpanExporter {
        SpanExporter {
            exporter,
            batch: Vec::with_capacity(10),
        }
    }
}

// pub trait SpanExporter {
//     // fn export(&self) -> Vec<Span>;
//     fn export(&mut self, batch: SpanProcessor) -> ExportResult;
//     fn shutdown();
// }
//
// impl SpanExporter for SpanProcessor {
//     // fn export(&self) -> Vec<Span> {
//     //     self.tracer.trace.to_vec()
//     // }
//     fn export(&mut self, batch: SpanProcessor) -> ExportResult {
//         // self.tracer.trace.to_vec()
//         batch.tracer.trace.to_vec()
//     }
//
//     fn shutdown() {
//         unimplemented!()
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    // use crate::trace::tracer_provider::TracerCreation;

    #[test]
    fn init() {
        let exporter = Exporter::InMemory;
        let test_span_exporter = SpanExporter::init(exporter);
        assert_eq!(test_span_exporter.batch.len(), 0);
    }
}
