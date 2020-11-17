use crate::HashMap;
use api::tracing::span::span_context::SpanContext;
use api::tracing::span::span_context::Uuid;
use api::tracing::span::Span;

pub trait Sampler {
    fn should_sample(
        &self,
        context: &SpanContext,
        trace_id: &Uuid,
        span_kind: &str,
        attributes: &HashMap<String, String>,
        links: &[String],
    ) -> &str;
}

impl Sampler for Span {
    fn should_sample(
        &self,
        _context: &SpanContext,
        _trace_id: &Uuid,
        _span_kind: &str,
        _attributes: &HashMap<String, String>,
        _links: &[String],
    ) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use api::tracing::tracer_provider::TracerProvider;

    #[test]
    fn should_sample() {
        let mut global = TracerProvider::default();
        let mut tracer = global.tracer.create_tracer();

        if let Some(span) = global.get_tracer().trace.get_mut(0) {
            let test_span_context = &span.span_context;
            let test_span_trace_id = &span.span_context.trace_id;
            let test_span_name = &span.name;
            let test_span_attributes = &span.attributes;
            let test_span_link = &span.link;
            let should_sample = span.should_sample(
                test_span_context,
                test_span_trace_id,
                test_span_name,
                test_span_attributes,
                test_span_link,
            );
            assert_eq!(should_sample.is_empty(), true);
        }
    }
}
