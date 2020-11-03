pub mod span;
pub mod span_context;
pub mod tracer;
pub mod tracer_provider;

pub struct Time {
    pub timestamp: u16,
    pub duration: u16,
}
