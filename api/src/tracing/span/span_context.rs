use self::trace_state::TraceState;
pub use uuid::Uuid;

mod trace_state;

pub struct SpanContext {
    pub trace_id: Uuid,
    pub span_id: Uuid,
    pub trace_flags: String,
    pub trace_state: TraceState,
}

impl SpanContext {
    pub fn create() -> Self {
        Self {
            trace_id: Uuid::new_v4(),
            span_id: Uuid::new_v4(),
            trace_flags: String::new(),
            trace_state: TraceState::create(),
        }
    }

    pub fn is_valid(&self) -> bool {
        self.trace_id.is_nil() && self.span_id.is_nil()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let span_context = SpanContext::create();
        assert_eq!(
            span_context.trace_id.get_version(),
            Some(uuid::Version::Random)
        );
        assert_eq!(
            span_context.span_id.get_version(),
            Some(uuid::Version::Random)
        );
        assert!(span_context.trace_flags.is_empty());
        assert!(span_context.trace_state.trace_state.is_empty());
    }

    #[test]
    fn is_valid() {
        let span_context = SpanContext::create();
        assert_eq!(span_context.is_valid(), false);
    }
}
