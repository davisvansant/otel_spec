// use crate::trace::span_processor::BatchingProcessor;
use crate::trace::span_processor::SpanProcessor;
use api::tracing::span::Span;
// use api::tracing::tracer_provider::TracerProvider;

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

pub struct SpanExporter<'a> {
    pub exporter: Exporter,
    pub batch: Vec<&'a Span>,
}

impl<'a> SpanExporter<'a> {
    pub fn init(exporter: Exporter) -> SpanExporter<'a> {
        SpanExporter {
            exporter,
            batch: Vec::with_capacity(10),
        }
    }
}

pub trait ExporterInterface<'a> {
    fn export(&mut self, batch: &'a mut SpanProcessor) -> ExportResult;
    fn shutdown();
}

impl<'a> ExporterInterface<'a> for SpanExporter<'a> {
    fn export(&mut self, batch: &'a mut SpanProcessor) -> ExportResult {
        let span = batch.collection.drain(..);
        for s in span.as_slice().iter() {
            self.batch.push(s.0);
        }

        ExportResult::Success
    }

    fn shutdown() {}
}
// //
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
    use crate::trace::tracer_provider::TracerCreation;
    use api::tracing::tracer_provider::TracerProvider;

    #[test]
    fn init() {
        let exporter = Exporter::InMemory;
        let test_span_exporter = SpanExporter::init(exporter);
        assert_eq!(test_span_exporter.batch.len(), 0);
    }

    #[test]
    fn export() {
        let mut global = TracerProvider::default();
        let test_name = env!("CARGO_PKG_NAME");
        let test_version = env!("CARGO_PKG_VERSION");
        global.create_tracer(test_name, test_version);
        let capacity: u16 = 2048;
        let mut processor = SpanProcessor::init(capacity);
        for span in global.tracer.trace.iter() {
            let test_span = &span;
            let test_parent_context = &span.span_context;
            processor.on_start(test_span, test_parent_context);
        }
        let exporter = Exporter::InMemory;
        let mut test_span_exporter = SpanExporter::init(exporter);
        assert_eq!(test_span_exporter.batch.len(), 0);
        test_span_exporter.export(&mut processor);
        assert_eq!(test_span_exporter.batch.len(), 1);
    }
}
