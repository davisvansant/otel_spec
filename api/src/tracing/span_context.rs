use self::trace_state::TraceState;

pub mod trace_state;

pub struct SpanContext {
    pub trace_id: u16,
    pub span_id: u8,
    pub trace_flags: String,
    pub trace_state: TraceState,
}
