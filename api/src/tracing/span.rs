use crate::tracing::span_context::SpanContext;

pub enum ParentSpan {
    Span,
    SpanContext,
}

pub struct Span {
    pub name: String,
    pub span_context: SpanContext,
    pub parent_span: ParentSpan,
    pub span_kind: String,
    pub start_timestamp: u16,
    pub stop_timestamp: u16,
    pub attributes: Vec<String>,
    pub link: Vec<String>,
    pub events: Vec<String>,
    pub status: String,
}

impl Span {
    pub fn create(
        name: String,
        span_context: SpanContext,
        parent_span: ParentSpan,
        span_kind: String,
        start_timestamp: u16,
        stop_timestamp: u16,
        status: String,
    ) -> Span {
        Span {
            name,
            span_context,
            parent_span,
            span_kind,
            start_timestamp,
            stop_timestamp,
            attributes: Vec::new(),
            link: Vec::new(),
            events: Vec::new(),
            status,
        }
    }
}
