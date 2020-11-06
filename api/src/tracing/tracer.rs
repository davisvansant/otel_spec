// use crate::tracing::span::span_context::SpanContext;
// use crate::tracing::span::span_kind::SpanKind;
// use crate::tracing::span::status::Status;
// use crate::tracing::span::ParentSpan;
use crate::tracing::span::Span;
// use crate::SystemTime;

pub struct Tracer {
    pub trace: Vec<Span>,
}

impl Tracer {
    pub fn init() -> Tracer {
        // unimplemented!()
        Tracer {
            trace: Vec::with_capacity(10),
        }
    }

    pub fn create_tracer(
        &mut self,
        // name: String,
        // span_context: SpanContext,
        // parent_span: ParentSpan,
        // span_kind: Option<SpanKind>,
        // start_timestamp: u16,
        // stop_timestamp: u16,
        // status: Status,
    ) {
        // let span = Span::create(
        //     // name,
        //     // span_context,
        //     parent_span,
        //     // span_kind,
        //     // start_timestamp,
        //     // stop_timestamp,
        //     // status,
        // );
        let span = Span::create();
        self.trace.push(span)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        let tracer = Tracer::init();
        assert!(tracer.trace.is_empty());
        assert_eq!(tracer.trace.len(), 0);
    }

    #[test]
    fn create_span() {
        // let mut tracer: Span = Tracer::create_span(ParentSpan::Span);
        // let mut tracer: Span = Tracer::create_span();
        let mut tracer = Tracer::init();
        tracer.create_tracer(
            // String::from("test_span_one"),
            // ParentSpan::Span,
            // String::from("test_span_kind"),
            // None,
        );
        assert_eq!(tracer.trace.is_empty(), false);
        assert_eq!(tracer.trace.len(), 1);

        tracer.create_tracer(
            // String::from("test_span_two"),
            // ParentSpan::Span,
            // String::from("test_span_kind"),
            // None,
        );
        assert_eq!(tracer.trace.len(), 2);

        tracer.create_tracer(
            // String::from("test_span_three"),
            // ParentSpan::Span,
            // String::from("test_span_kind"),
            // None,
            // Some(SpanKind::Producer),
        );
        assert_eq!(tracer.trace.len(), 3);

        tracer.create_tracer(
            // String::from("test_span_three"),
            // ParentSpan::Span,
            // String::from("test_span_kind"),
            // None,
            // Some(SpanKind::Consumer),
        );
        assert_eq!(tracer.trace.len(), 4);
    }
}
