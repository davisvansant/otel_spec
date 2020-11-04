use crate::tracing::span::span_kind::SpanKind;
use crate::tracing::span::status::Status;
use crate::tracing::span::ParentSpan;
use crate::tracing::span::Span;
use crate::tracing::span_context::SpanContext;

pub struct Tracer {
    pub span: Vec<Span>,
}

impl Tracer {
    pub fn init() -> Tracer {
        // unimplemented!()
        Tracer { span: Vec::new() }
    }

    pub fn create_tracer(
        &mut self,
        name: String,
        // span_context: SpanContext,
        parent_span: ParentSpan,
        span_kind: Option<SpanKind>,
        // start_timestamp: u16,
        // stop_timestamp: u16,
        // status: Status,
    ) {
        let span = Span::create(
            name,
            // span_context,
            parent_span,
            span_kind,
            // start_timestamp,
            // stop_timestamp,
            // status,
        );
        self.span.push(span)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        let tracer = Tracer::init();
        assert!(tracer.span.is_empty());
        assert_eq!(tracer.span.len(), 0);
    }

    #[test]
    fn create() {
        let mut tracer = Tracer::init();
        tracer.create_tracer(
            String::from("test_span_one"),
            ParentSpan::Span,
            // String::from("test_span_kind"),
            None,
        );
        assert_eq!(tracer.span.is_empty(), false);
        assert_eq!(tracer.span.len(), 1);

        tracer.create_tracer(
            String::from("test_span_two"),
            ParentSpan::Span,
            // String::from("test_span_kind"),
            None,
        );
        assert_eq!(tracer.span.len(), 2);

        tracer.create_tracer(
            String::from("test_span_three"),
            ParentSpan::Span,
            // String::from("test_span_kind"),
            // None,
            Some(SpanKind::Producer),
        );
        assert_eq!(tracer.span.len(), 3);

        tracer.create_tracer(
            String::from("test_span_three"),
            ParentSpan::Span,
            // String::from("test_span_kind"),
            // None,
            Some(SpanKind::Consumer),
        );
        assert_eq!(tracer.span.len(), 4);
    }
}
