// use api::tracing::tracer::Tracer;
use api::tracing::span::StatusCode;
use api::tracing::tracer_provider::TracerProvider;

trait TracerCreation {
    fn create_tracer(&mut self, name: &'static str, version: &'static str);
}

impl TracerCreation for TracerProvider {
    fn create_tracer(&mut self, name: &'static str, version: &'static str) {
        self.name = name;
        self.version = version;
        self.tracer.create_tracer();
    }
}

trait TracerShutdown {
    fn shutdown(&mut self);
}

impl TracerShutdown for TracerProvider {
    fn shutdown(&mut self) {
        for span in self.tracer.trace.iter_mut() {
            let status = StatusCode::Ok;
            let description = String::from("Shutdown Initiated");
            span.status.set(status, Some(description));
            span.end();
            println!("Shutdown initiated");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_tracer() {
        let mut global = TracerProvider::default();
        let test_name = env!("CARGO_PKG_NAME");
        let test_version = env!("CARGO_PKG_VERSION");
        global.create_tracer(test_name, test_version);
        assert_eq!(global.name, env!("CARGO_PKG_NAME"));
        assert_eq!(global.version, env!("CARGO_PKG_VERSION"));
        assert_eq!(global.tracer.trace.len(), 1);
    }

    #[test]
    fn shutdown() {
        let mut global = TracerProvider::default();
        let test_trace_one_name = "test_trace_one";
        let test_trace_one_version = "1.0.0";
        global.create_tracer(test_trace_one_name, test_trace_one_version);
        let test_trace_two_name = "test_trace_two";
        let test_trace_two_version = "1.0.0";
        global.create_tracer(test_trace_two_name, test_trace_two_version);
        assert_eq!(global.tracer.trace.len(), 2);
        for span in global.tracer.trace.iter() {
            assert_eq!(span.status.status_code, StatusCode::Unset);
            assert_eq!(span.status.description, None);
        }
        global.shutdown();
        for span in global.tracer.trace.iter() {
            assert_eq!(span.status.status_code, StatusCode::Ok);
            assert_eq!(
                span.status.description,
                Some(String::from("Shutdown Initiated"))
            );
        }
    }
}
