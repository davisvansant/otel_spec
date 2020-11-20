// use api::tracing::tracer::Tracer;
// use api::tracing::span::StatusCode;
use api::tracing::span::Span;
// use api::tracing::tracer_provider::TracerProvider;

pub struct SpanProcessor {
    pub collection: Vec<Span>,
}

impl SpanProcessor {
    pub fn init() -> SpanProcessor {
        SpanProcessor {
            collection: Vec::with_capacity(10),
        }
    }
}
// trait SpanProcessor {
//     fn on_start();
//     fn on_end();
//     fn shutdown();
//     fn force_flush();
//     fn simple(exporter: String);
//     fn batching(
//         exporter: String,
//         max_queue_size: u8,
//         scheduled_delay_millis: u8,
//         export_timeout_millis: u8,
//         max_export_batch_size: u8,
//     );
// }

// impl SpanProcessor for TracerProvider {
//     fn on_start() {
//         unimplemented!();
//     }
//     fn on_end() {
//         unimplemented!();
//     }
//     fn shutdown() {
//         unimplemented!();
//     }
//     fn force_flush() {
//         unimplemented!();
//     }
//     fn simple(&mut self, exporter: String) {
//         self.tracer.
//     }
// }

// pub trait BuiltIn {
//     fn simple(exporter: String);
//     fn batching(
//         exporter: String,
//         max_queue_size: u8,
//         scheduled_delay_millis: u8,
//         export_timeout_millis: u8,
//         max_export_batch_size: u8,
//     );
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        let processor = SpanProcessor::init();
        assert_eq!(processor.collection.len(), 0);
    }
}
