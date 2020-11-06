use crate::tracing::span::Span;
use crate::tracing::tracer::Tracer;

pub struct TracerProvider {
    name: &'static str,
    version: &'static str,
    tracer: Tracer,
}

impl TracerProvider {
    pub fn default() -> TracerProvider {
        TracerProvider {
            name: env!("CARGO_PKG_NAME"),
            version: env!("CARGO_PKG_VERSION"),
            tracer: Tracer::init(),
        }
    }

    fn get_tracer(&mut self) -> &mut Tracer {
        &mut self.tracer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        let global = TracerProvider::default();
        assert_eq!(global.name, env!("CARGO_PKG_NAME"));
        assert_eq!(global.version, env!("CARGO_PKG_VERSION"));
        assert!(global.tracer.trace.is_empty());
        assert_eq!(global.tracer.trace.len(), 0);
    }

    #[test]
    fn get_tracer() {
        let mut global = TracerProvider::default();
        let mut tracer = global.get_tracer();
        assert!(tracer.trace.is_empty());
        assert_eq!(tracer.trace.len(), 0);

        tracer.create_tracer();
        assert_eq!(tracer.trace.is_empty(), false);
        assert_eq!(tracer.trace.len(), 1);
    }
}
