// use api::tracing::tracer::Tracer;
// use api::tracing::span::StatusCode;
// use api::tracing::span::Span;
// use api::tracing::tracer_provider::TracerProvider;
use crate::trace::span_exporter::Exporter;

pub struct SimpleProcessor {
    pub exporter: Exporter,
}

impl SimpleProcessor {
    pub fn init(exporter: Exporter) -> SimpleProcessor {
        SimpleProcessor { exporter }
    }
}

pub struct BatchingProcessor {
    pub exporter: Exporter,
    pub max_queue_size: u16,
    pub scheduled_delay_millis: u16,
    pub export_timeout_millis: u16,
    pub max_export_batch_size: u16,
}

impl BatchingProcessor {
    pub fn init(exporter: Exporter) -> BatchingProcessor {
        BatchingProcessor {
            exporter,
            max_queue_size: 2048,
            scheduled_delay_millis: 5000,
            export_timeout_millis: 30000,
            max_export_batch_size: 512,
        }
    }
}

// pub struct SpanProcessor {
//     pub collection: Vec<Span>,
// }
//
// impl SpanProcessor {
//     pub fn init() -> SpanProcessor {
//         SpanProcessor {
//             collection: Vec::with_capacity(10),
//         }
//     }
// }
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
    fn init_simple() {
        // let exporter = String::from("test_simple_exporter");
        let exporter = Exporter::StandardOutput;
        let simple = SimpleProcessor::init(exporter);
        // assert_eq!(simple.exporter, String::from("test_simple_exporter"));
        assert_eq!(simple.exporter, Exporter::StandardOutput);
    }

    #[test]
    fn init_batching() {
        // let exporter = String::from("test_batching_exporter");
        let exporter = Exporter::StandardOutput;
        let batching = BatchingProcessor::init(exporter);
        // assert_eq!(batching.exporter, String::from("test_batching_exporter"));
        assert_eq!(batching.exporter, Exporter::StandardOutput);
        assert_eq!(batching.max_queue_size, 2048);
        assert_eq!(batching.scheduled_delay_millis, 5000);
        assert_eq!(batching.export_timeout_millis, 30000);
        assert_eq!(batching.max_export_batch_size, 512);
    }
}
