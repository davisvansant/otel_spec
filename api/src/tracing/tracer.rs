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
        span_kind: String,
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
