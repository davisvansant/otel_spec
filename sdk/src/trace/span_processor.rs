// use api::tracing::tracer::Tracer;
// use api::tracing::span::StatusCode;
use api::tracing::tracer_provider::TracerProvider;

trait SpanProcessor {
    fn on_start();
    fn on_end();
    fn shutdown();
    fn force_flush();
}

impl SpanProcessor for TracerProvider {
    fn on_start() {
        unimplemented!();
    }
    fn on_end() {
        unimplemented!();
    }
    fn shutdown() {
        unimplemented!();
    }
    fn force_flush() {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn () {
    //
    // }
    //
}
