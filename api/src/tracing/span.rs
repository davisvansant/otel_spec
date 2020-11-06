use crate::tracing::span::events::Event;
use crate::tracing::span::span_context::SpanContext;
use crate::tracing::span::span_kind::SpanKind;
use crate::tracing::span::status::{Status, StatusCode};
// use std::time::{Duration, SystemTime};
use crate::SystemTime;

pub(crate) mod events;
pub mod span_context;
pub mod span_kind;
pub mod status;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum ParentSpan {
    Span,
    SpanContext,
}

pub struct Span {
    pub name: String,
    pub span_context: SpanContext,
    pub parent_span: ParentSpan,
    pub span_kind: SpanKind,
    pub start_timestamp: SystemTime,
    pub stop_timestamp: Option<SystemTime>,
    pub attributes: Vec<String>,
    pub link: Vec<String>,
    pub(crate) events: Vec<Event>,
    pub status: Status,
}

impl Span {
    pub(crate) fn create(// name: String,
        // span_context: SpanContext,
        // parent_span: ParentSpan,
        // span_kind: Option<SpanKind>,
        // start_timestamp: u16,
        // stop_timestamp: u16,
        // status: Status,
    ) -> Span {
        Span {
            // name,
            name: String::with_capacity(32),
            span_context: SpanContext::create(),
            // parent_span,
            // span_kind,
            parent_span: ParentSpan::Span,
            span_kind: SpanKind::Internal,
            start_timestamp: SystemTime::now(),
            stop_timestamp: None,
            attributes: Vec::with_capacity(10),
            link: Vec::with_capacity(10),
            events: Vec::with_capacity(10),
            status: Status::default(),
        }
    }

    pub fn get_context(&self) -> &SpanContext {
        &self.span_context
    }

    pub fn is_recording(&self) -> bool {
        self.stop_timestamp.is_none()
    }

    pub fn set_attributes(&mut self) {
        unimplemented!()
    }

    pub fn add_event(&mut self, name: String) {
        let event = Event::new(name);
        self.events.push(event)
    }

    pub fn set_status(&mut self, status: StatusCode, description: Option<String>) {
        self.status.set(status, description);
    }

    pub fn update_name(&mut self, name: String) {
        self.name = name
    }

    pub fn end(&mut self) {
        self.stop_timestamp = Some(SystemTime::now())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let span = Span::create(
            // String::from("test_span"),
            // ParentSpan::Span,
            // String::from("test_span_kind"),
            // None,
        );

        // assert_eq!(span.name, String::from("test_span"));
        assert!(span.name.is_empty());
        assert_eq!(span.span_context.trace_id.is_nil(), false);
        assert_eq!(span.span_context.span_id.is_nil(), false);
        assert_eq!(span.span_context.trace_flags.is_empty(), true);
        assert!(span.span_context.trace_state.trace_state.is_empty());
        assert_eq!(span.parent_span, ParentSpan::Span);
        assert_eq!(span.span_kind, SpanKind::Internal);
        assert_ne!(span.start_timestamp, SystemTime::now());
        assert!(span.stop_timestamp.is_none());
        assert!(span.attributes.is_empty());
        assert!(span.link.is_empty());
        assert!(span.events.is_empty());
        assert_eq!(span.status.status_code, StatusCode::Unset);
        assert!(span.status.description.is_none());
    }

    #[test]
    fn get_context() {
        let span = Span::create(
            // String::from("test_span"),
            // ParentSpan::Span,
            // String::from("test_span_kind"),
            // None,
        );

        let context = span.get_context();
        assert_eq!(context.trace_id.is_nil(), false);
        assert_eq!(context.span_id.is_nil(), false);
        assert_eq!(context.trace_flags.is_empty(), true);
        assert!(context.trace_state.trace_state.is_empty());
    }

    #[test]
    fn is_recording() {
        let mut span = Span::create(
            // String::from("test_span"),
            // ParentSpan::Span,
            // String::from("test_span_kind"),
            // None,
        );
        assert!(span.is_recording());
        span.end();
        assert_eq!(span.is_recording(), false);
    }

    // #[test]
    // fn set_attributes() {
    //     unimplemented!()
    // }

    #[test]
    fn add_event() {
        let mut span = Span::create(
            // String::from("test_span"),
            // ParentSpan::Span,
            // String::from("test_span_kind"),
            // None,
        );

        assert_eq!(span.events.len(), 0);
        span.add_event(String::from("test_event"));
        assert_eq!(span.events.len(), 1);
    }

    #[test]
    fn set_status() {
        let mut span = Span::create(
            // String::from("test_span"),
            // ParentSpan::Span,
            // String::from("test_span_kind"),
            // None,
        );

        assert_eq!(span.status.status_code, StatusCode::Unset);
        span.set_status(StatusCode::Ok, None);
        assert_ne!(span.status.status_code, StatusCode::Unset);
        assert_eq!(span.status.status_code, StatusCode::Ok);
        span.set_status(StatusCode::Error, None);
        assert_ne!(span.status.status_code, StatusCode::Ok);
        assert_eq!(span.status.status_code, StatusCode::Error);
    }

    #[test]
    fn update_name() {
        let mut span = Span::create(
            // String::from("test_span"),
            // ParentSpan::Span,
            // String::from("test_span_kind"),
            // None,
        );

        // assert_eq!(span.name, String::from("test_span"));
        assert!(span.name.is_empty());
        span.update_name(String::from("changed_span_name"));
        assert_eq!(span.name, String::from("changed_span_name"));
    }

    #[test]
    fn end() {
        let mut span = Span::create(
            // String::from("test_span"),
            // ParentSpan::Span,
            // String::from("test_span_kind"),
            // None,
        );

        assert!(span.stop_timestamp.is_none());
        span.end();
        assert!(span.stop_timestamp.is_some());
        assert_ne!(span.stop_timestamp.unwrap(), SystemTime::now());
        assert!(span.stop_timestamp.unwrap() > span.start_timestamp);
    }
}
