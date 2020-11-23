// use api::tracing::tracer::Tracer;
// use api::tracing::span::StatusCode;
// use crate::HashMap;
use api::tracing::span::span_context::SpanContext;
use api::tracing::span::Span;
// use api::tracing::tracer_provider::TracerProvider;
use crate::trace::span_exporter::Exporter;

// #[derive(Eq, PartialEq, Ord, PartialOrd)]
pub struct SpanProcessor<'a> {
    // pub collection: Vec<HashMap<String, String>>,
    pub collection: Vec<(&'a Span, &'a SpanContext)>,
    pub collecting: bool,
}

impl<'a> SpanProcessor<'a> {
    pub fn init(capacity: u16) -> SpanProcessor<'static> {
        SpanProcessor {
            collection: Vec::with_capacity(capacity.into()),
            collecting: true,
        }
    }

    pub fn on_start(&mut self, span: &'a Span, parent_context: &'a SpanContext) {
        if self.collecting {
            // let mut span_object: HashMap<String, String> = HashMap::new();
            // span_object.insert((span, parent_context));
            // self.collection.push(span_object);
            self.collection.push((span, parent_context));
        } else {
            println!("Span Processor is Shutdown");
        }
    }

    pub fn on_end(&mut self, span: &'a Span) {
        if self.collecting {
            self.collection.retain(|(x, _)| x != &span);
        } else {
            println!("Span Processor is Shutdown");
        }
    }

    pub fn shutdown(&mut self) {
        self.collecting = false;
        println!("Shutdown Initiated");
        println!("Processor Collecting Status - {:?}", self.collecting);
    }

    pub fn force_flush(&mut self) {
        if self.collecting {
            // setup exporter here
            self.collection.clear();
        }
    }
}

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

// pub trait Processor {
//     fn on_start(&mut self);
//     fn on_end(&mut self);
// }
//
// impl Processor for Span {
//     fn on_start(&mut self) {
//         unimplemented!();
//     }
//
//     fn on_end(&mut self) {
//         unimplemented!();
//     }
// }

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
    use crate::trace::tracer_provider::TracerCreation;
    use api::tracing::tracer_provider::TracerProvider;

    #[test]
    fn init() {
        // let exporter = String::from("test_simple_exporter");
        let capacity: u16 = 2048;
        let processor = SpanProcessor::init(capacity);
        // let simple = SimpleProcessor::init(exporter);
        // assert_eq!(simple.exporter, String::from("test_simple_exporter"));
        // assert_eq!(simple.exporter, Exporter::StandardOutput);
        assert_eq!(processor.collection.len(), 0);
        assert_eq!(processor.collecting, true);
    }

    #[test]
    fn on_start() {
        let mut global = TracerProvider::default();
        let test_name = env!("CARGO_PKG_NAME");
        let test_version = env!("CARGO_PKG_VERSION");
        global.create_tracer(test_name, test_version);
        let capacity: u16 = 2048;
        let mut processor = SpanProcessor::init(capacity);
        assert_eq!(processor.collection.len(), 0);
        assert_eq!(processor.collecting, true);
        for span in global.tracer.trace.iter() {
            let test_span = &span;
            let test_parent_context = &span.span_context;
            processor.on_start(test_span, test_parent_context);
        }
        assert_eq!(processor.collection.len(), 1);
    }

    #[test]
    fn on_start_shutdown() {
        let mut global = TracerProvider::default();
        let test_name = env!("CARGO_PKG_NAME");
        let test_version = env!("CARGO_PKG_VERSION");
        global.create_tracer(test_name, test_version);
        let capacity: u16 = 2048;
        let mut processor = SpanProcessor::init(capacity);
        assert_eq!(processor.collection.len(), 0);
        assert_eq!(processor.collecting, true);
        processor.shutdown();
        assert_eq!(processor.collecting, false);
        for span in global.tracer.trace.iter() {
            let test_span = &span;
            let test_parent_context = &span.span_context;
            processor.on_start(test_span, test_parent_context);
        }
        assert_eq!(processor.collection.len(), 0);
    }

    #[test]
    fn on_end() {
        let mut global = TracerProvider::default();
        let test_name = env!("CARGO_PKG_NAME");
        let test_version = env!("CARGO_PKG_VERSION");
        global.create_tracer(test_name, test_version);
        let capacity: u16 = 2048;
        let mut processor = SpanProcessor::init(capacity);
        assert_eq!(processor.collection.len(), 0);
        assert_eq!(processor.collecting, true);
        for mut span in global.tracer.trace.iter_mut() {
            span.name = String::from("test_span_name");
        }

        for span in global.tracer.trace.iter() {
            let test_span = &span;
            let test_parent_context = &span.span_context;
            processor.on_start(test_span, test_parent_context);
        }

        assert_eq!(processor.collection.len(), 1);

        for span in global.tracer.trace.iter() {
            processor.on_end(span);
        }
        assert_eq!(processor.collection.len(), 0);
        for span in global.tracer.trace.iter() {
            processor.on_end(span);
        }
        assert_eq!(processor.collection.len(), 0);
    }

    #[test]
    fn on_end_shutdown() {
        let mut global = TracerProvider::default();
        let test_name = env!("CARGO_PKG_NAME");
        let test_version = env!("CARGO_PKG_VERSION");
        global.create_tracer(test_name, test_version);
        let capacity: u16 = 2048;
        let mut processor = SpanProcessor::init(capacity);
        assert_eq!(processor.collection.len(), 0);
        assert_eq!(processor.collecting, true);
        processor.shutdown();

        for span in global.tracer.trace.iter() {
            let test_span = &span;
            let test_parent_context = &span.span_context;
            processor.on_start(test_span, test_parent_context);
        }
        assert_eq!(processor.collection.len(), 0);

        for span in global.tracer.trace.iter() {
            processor.on_end(span);
        }
        assert_eq!(processor.collection.len(), 0);

        for span in global.tracer.trace.iter() {
            processor.on_end(span);
        }
        assert_eq!(processor.collection.len(), 0);
    }

    #[test]
    fn shutdown() {
        // let exporter = String::from("test_simple_exporter");
        let capacity: u16 = 2048;
        let mut processor = SpanProcessor::init(capacity);
        // let simple = SimpleProcessor::init(exporter);
        // assert_eq!(simple.exporter, String::from("test_simple_exporter"));
        // assert_eq!(simple.exporter, Exporter::StandardOutput);
        // assert_eq!(processor.collection.len(), 0);
        assert_eq!(processor.collecting, true);
        processor.shutdown();
        assert_eq!(processor.collecting, false);
    }

    #[test]
    fn force_flush() {
        let mut global = TracerProvider::default();
        let test_name = env!("CARGO_PKG_NAME");
        let test_version = env!("CARGO_PKG_VERSION");
        global.create_tracer(test_name, test_version);
        let capacity: u16 = 2048;
        let mut processor = SpanProcessor::init(capacity);
        assert_eq!(processor.collection.len(), 0);
        assert_eq!(processor.collecting, true);
        for span in global.tracer.trace.iter() {
            let test_span = &span;
            let test_parent_context = &span.span_context;
            processor.on_start(test_span, test_parent_context);
        }
        assert_eq!(processor.collection.len(), 1);
        processor.force_flush();
        assert_eq!(processor.collection.len(), 0);
    }

    #[test]
    fn force_flush_shutdown() {
        let mut global = TracerProvider::default();
        let test_name = env!("CARGO_PKG_NAME");
        let test_version = env!("CARGO_PKG_VERSION");
        global.create_tracer(test_name, test_version);
        let capacity: u16 = 2048;
        let mut processor = SpanProcessor::init(capacity);
        assert_eq!(processor.collection.len(), 0);
        assert_eq!(processor.collecting, true);
        for span in global.tracer.trace.iter() {
            let test_span = &span;
            let test_parent_context = &span.span_context;
            processor.on_start(test_span, test_parent_context);
        }
        assert_eq!(processor.collection.len(), 1);
        processor.shutdown();
        processor.force_flush();
        assert_eq!(processor.collection.len(), 1);
    }

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
